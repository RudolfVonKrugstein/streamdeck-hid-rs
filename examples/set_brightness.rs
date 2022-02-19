use streamdeck_hid_rs::StreamDeckDevice;

fn main() {
    let hidapi = hidapi::HidApi::new().unwrap();
    let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    // make it completely dark!
    device.set_brightness(0);
}