#[derive(Debug)]
pub enum Error {
    NotAStreamDeckDevice,
    NoDeviceFound,
    HidError(hidapi::HidError),
}
