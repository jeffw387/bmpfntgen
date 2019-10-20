use super::error::Error;
use rusttype::{Font, Point, PositionedGlyph, Scale};

#[derive(Clone)]
pub struct Glyph<'a> {
    pub pglyph: Option<PositionedGlyph<'a>>,
    pub codepoint: char,
    pub min: Point<i32>,
    pub max: Point<i32>,
    pub offset: Point<f32>,
}

pub fn get_glyphs_from_str<'a>(
    font: &'a Font,
    char_string: &str,
    scale: Scale,
    y_offset: f32,
) -> Result<Vec<Glyph<'a>>, Error> {
    let mut glyphs = vec![];
    for c in char_string.chars() {
        let rglyph = font.glyph(c);
        let sglyph = rglyph.scaled(scale);
        let h_metrics = sglyph.h_metrics();
        let x_offset = h_metrics.left_side_bearing;
        let offset = rusttype::point(x_offset, y_offset);
        let pglyph = sglyph.positioned(offset);
        let bbox =
            pglyph.pixel_bounding_box().ok_or(Error::BoundingBoxError)?;

        let glyph = Glyph {
            pglyph: Some(pglyph),
            codepoint: c,
            min: bbox.min,
            max: bbox.max,
            offset,
        };
        glyphs.push(glyph);
    }
    Ok(glyphs)
}
