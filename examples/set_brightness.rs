use streamdeck_hid_rs::StreamDeckDevice;

fn main() {
    let mut hidapi = hidapi::HidApi::new().unwrap();
    let device = StreamDeckDevice::open_first_device(&mut hidapi).unwrap();
    // make it completely dark!
    device.set_brightness(0).unwrap();
}