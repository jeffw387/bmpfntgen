pub mod bitmap;
pub mod char_sets;
pub mod error;
pub mod glyph;
pub mod meta;

pub use self::meta::MetaFormat;
pub use bitmap::Bitmap;
pub use char_sets::CharSets;
pub use error::Error;
pub use glyph::Glyph;
use rusttype::{Font, FontCollection, Scale};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::str::FromStr;

pub fn load_ttf(font_path: &str) -> Result<Font, Error> {
    let font_bytes = fs::read(font_path)?;
    let collection = FontCollection::from_bytes(font_bytes)?;

    let font = collection.into_font()?;
    Ok(font)
}

pub struct RenderResult<'a> {
    pub bitmap: Bitmap,
    pub glyphs: Vec<Glyph<'a>>,
}

pub fn layout_and_render<'a>(
    font: &'a Font,
    char_set: CharSets,
    font_height: f32,
) -> Result<RenderResult<'a>, Error> {
    let mut bitmap = Bitmap::default();

    let char_string: &str = char_set.into();

    let v_metrics = font.v_metrics(Scale::uniform(font_height));
    let y_offset = v_metrics.ascent;
    let scale = Scale::uniform(font_height);
    let glyphs =
        glyph::get_glyphs_from_str(&font, char_string, scale, y_offset)?;
    let mut positioned_glyphs = vec![];
    for glyph in glyphs.iter() {
        positioned_glyphs.push(bitmap::resize_bitmap(glyph.clone(), &mut bitmap)?);
    }
    for glyph in positioned_glyphs.iter() {
        bitmap::render_into_bitmap(glyph, &mut bitmap)?;
    }
    let result = RenderResult { bitmap, glyphs: positioned_glyphs };
    Ok(result)
}

#[derive(Debug)]
pub enum ImageFormat {
    PNG,
    BMP,
    TIFF,
    JPEG,
    ICO,
}

impl FromStr for ImageFormat {
    type Err = &'static str;

    fn from_str(image_format: &str) -> Result<ImageFormat, &'static str> {
        match image_format {
            "PNG" => Ok(ImageFormat::PNG),
            "BMP" => Ok(ImageFormat::BMP),
            "TIFF" => Ok(ImageFormat::TIFF),
            "JPEG" => Ok(ImageFormat::JPEG),
            "ICO" => Ok(ImageFormat::ICO),
            _ => Err("Invalid image format!"),
        }
    }
}

fn make_image_output_path(
    image_format: &ImageFormat,
    name: &str,
    path: &str,
) -> String {
    let ext = match image_format {
        ImageFormat::PNG => ".png",
        ImageFormat::BMP => ".bmp",
        ImageFormat::TIFF => ".tiff",
        ImageFormat::JPEG => ".jpg",
        ImageFormat::ICO => ".ico",
    };
    String::from(path) + name + ext
}

fn make_metadata_output_path(
    meta_format: &MetaFormat,
    name: &str,
    path: &str,
) -> String {
    let ext = match meta_format {
        MetaFormat::Default => ".json",
    };
    String::from(path) + name + ext
}

pub fn save(
    output_name: &str,
    output_path: &str,
    result: RenderResult,
    meta_format: MetaFormat,
    image_format: ImageFormat,
) -> Result<(), Error> {
    let image_output_path =
        make_image_output_path(&image_format, output_name, output_path);
    let meta_output_path =
        make_metadata_output_path(&meta_format, output_name, output_path);

    image::save_buffer(
        image_output_path,
        &result.bitmap.data,
        result.bitmap.width as u32,
        result.bitmap.height as u32,
        image::Gray(8),
    )?;

    let meta_file = meta::create_meta_file(&meta_format, &result.glyphs)?;
    let fout = &mut BufWriter::new(File::create(meta_output_path)?);
    fout.write(meta_file.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_png_path() {
        let name = "test_file";
        let format = ImageFormat::PNG;
        let path = "./";

        let result = make_image_output_path(&format, name, path);
        assert_eq!(result, "./test_file.png");
    }
}
