use super::error::Error;
use super::glyph::Glyph;
use rusttype::Point;

#[derive(Default)]
pub struct Bitmap {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

fn update_glyph_pos(
    mut glyph: Glyph<'_>,
    offset: Point<f32>,
) -> Result<Glyph<'_>, Error> {
    let mut pglyph = glyph.pglyph.ok_or(Error::BoundingBoxError)?;
    let old_pos = pglyph.position();
    let new_pos = rusttype::point(
        old_pos.x + offset.x,
        old_pos.y + offset.y,
    );
    pglyph.set_position(new_pos);
    let bb = pglyph.pixel_bounding_box().ok_or(Error::BoundingBoxError)?;
    glyph.min = bb.min;
    glyph.max = bb.max;
    glyph.pglyph = Some(pglyph);
    Ok(glyph)
}

pub fn resize_bitmap<'a>(
    glyph: Glyph<'a>,
    bmp: &mut Bitmap,
) -> Result<Glyph<'a>, Error> {
    let start_x = bmp.width;
    let pos_offset = rusttype::point(
        start_x as f32,
        0.0,
    );
    let glyph = update_glyph_pos(glyph, pos_offset)?;
    let glyph_height = (glyph.max.y) as usize;
    if bmp.height < glyph_height {
        bmp.height = glyph_height;
    };
    let pos_max = glyph.max.x as usize;
    bmp.width = pos_max + 1;
    let new_len = bmp.width * bmp.height;
    if bmp.data.len() < new_len {
        bmp.data.resize(new_len, 0);
    };
    Ok(glyph)
}

pub fn render_into_bitmap(
    glyph: &Glyph<'_>,
    bmp: &mut Bitmap,
) -> Result<(), Error> {
    let pglyph = 
        glyph
        .pglyph
        .clone()
        .ok_or(Error::NoGlyphsLoaded)?;

    let bbox = pglyph.pixel_bounding_box().ok_or(Error::BoundingBoxError)?;
    pglyph
        .draw(|x, y, v| {
            let x_offset = bbox.min.x as usize;
            let y_offset = bbox.min.y as usize;
            let x = x as usize + x_offset;
            let y = y as usize + y_offset;
            let pixel_value = (v * 255.0).round() as u8;
            let pixel_index = x + (y * bmp.width);
            bmp.data[pixel_index] = pixel_value;
        });
    Ok(())
}
