#[derive(Debug)]
pub enum Error {
    NotAStreamDeckDevice,
    NoDeviceFound,
    HidError(hidapi::HidError),
    DimensionMismatch(u32, u32),
    ImageEncodingError(image::ImageError),
    IncorrectWriteLengthError,
}
