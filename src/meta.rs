use super::error::Error;
use super::glyph::Glyph;
use rusttype::Point;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug)]
pub enum MetaFormat {
    Default,
}

impl FromStr for MetaFormat {
    type Err = &'static str;

    fn from_str(meta_format: &str) -> Result<MetaFormat, &'static str> {
        match meta_format {
            _ => Ok(MetaFormat::Default),
        }
    }
}

pub fn create_meta_file(
    fmt: &MetaFormat,
    glyphs: &Vec<Glyph<'_>>,
) -> Result<String, Error> {
    match fmt {
        MetaFormat::Default => make_default(glyphs),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SerdePoint<N> {
    x: N,
    y: N,
}

impl From<Point<i32>> for SerdePoint<i32> {
    fn from(p: Point<i32>) -> SerdePoint<i32> {
        SerdePoint { x: p.x, y: p.y }
    }
}

impl From<Point<f32>> for SerdePoint<f32> {
    fn from(p: Point<f32>) -> SerdePoint<f32> {
        SerdePoint { x: p.x, y: p.y }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DefaultMetaGlyph {
    codepoint: char,
    min: SerdePoint<i32>,
    max: SerdePoint<i32>,
    offset: SerdePoint<f32>,
}

impl From<&Glyph<'_>> for DefaultMetaGlyph {
    fn from(glyph: &Glyph<'_>) -> DefaultMetaGlyph {
        DefaultMetaGlyph {
            codepoint: glyph.codepoint,
            min: glyph.min.into(),
            max: glyph.max.into(),
            offset: glyph.offset.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DefaultMetaObject {
    glyphs: Vec<DefaultMetaGlyph>,
}

fn make_default(glyphs: &Vec<Glyph<'_>>) -> Result<String, Error> {
    let mut obj = DefaultMetaObject { glyphs: vec![] };
    for glyph in glyphs.iter() {
        obj.glyphs.push(glyph.into());
    }
    serde_json::to_string(&obj).map_err(Error::SERDE)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serialize() {
        let glyphs = vec![
            Glyph {
                pglyph: None,
                codepoint: 'a',
                min: rusttype::point(0, 0),
                max: rusttype::point(31, 31),
                offset: rusttype::point(10.0, 0.0),
            },
            Glyph {
                pglyph: None,
                codepoint: '1',
                min: rusttype::point(32, 0),
                max: rusttype::point(63, 31),
                offset: rusttype::point(11.0, 1.0),
            },
        ];

        let result_string = make_default(&glyphs).unwrap();
        let result_t: DefaultMetaObject =
            serde_json::from_str(&result_string).unwrap();
        let result_v = serde_json::to_value(&result_t).unwrap();
        let expected = json!({
            "glyphs": [
            {
                "codepoint": "a",
                "min": {
                    "x": 0,
                    "y": 0
                },
                "max": {
                    "x": 31,
                    "y": 31
                },
                "offset": {
                    "x": 10.0,
                    "y": 0.0
                }
            },
            {
                "codepoint": "1",
                "min": {
                    "x": 32,
                    "y": 0
                },
                "max": {
                    "x": 63,
                    "y": 31
                },
                "offset": {
                    "x": 11.0,
                    "y": 1.0
                }
            }
            ]
        });
        assert_eq!(result_v, expected);
    }
}
