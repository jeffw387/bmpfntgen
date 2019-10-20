use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    RT(rusttype::Error),
    IMG(image::ImageError),
    SERDE(serde_json::Error),
    NoGlyphsLoaded,
    BoundingBoxError,
    LastGlyphNotFound,
    FirstGlyphNotFound,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<rusttype::Error> for Error {
    fn from(err: rusttype::Error) -> Error {
        Error::RT(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Error {
        Error::IMG(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SERDE(err)
    }
}
