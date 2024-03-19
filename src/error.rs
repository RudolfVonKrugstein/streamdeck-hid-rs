use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum StreamDeckError {
    NotAStreamDeckDevice,
    NoDeviceFound,
    HidError(hidapi::HidError),
    DimensionMismatch(u32, u32),
    ImageEncodingError(image::ImageError),
    IncorrectWriteLengthError,
}

impl Display for StreamDeckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            StreamDeckError::NotAStreamDeckDevice => {
                write!(f, "streamdeck error: not a streamdeck device")
            }
            StreamDeckError::NoDeviceFound => {
                write!(f, "streamdeck error: no streamdeck device found")
            }
            StreamDeckError::HidError(error) => write!(f, "{error}"),
            StreamDeckError::DimensionMismatch(x, y) => {
                write!(f, "streamdeck error: image size mismatch: ({x}, {y})")
            }
            StreamDeckError::ImageEncodingError(error) => write!(f, "{error}"),
            StreamDeckError::IncorrectWriteLengthError => {
                write!(f, "streamdeck error: incorrect write length")
            }
        }
    }
}

impl Error for StreamDeckError {}

impl From<hidapi::HidError> for StreamDeckError {
    fn from(e: hidapi::HidError) -> Self {
        Self::HidError(e)
    }
}

impl From<image::ImageError> for StreamDeckError {
    fn from(e: image::ImageError) -> Self {
        Self::ImageEncodingError(e)
    }
}
