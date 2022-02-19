use streamdeck_hid_rs::StreamDeckDevice;

fn main() {
    let hidapi = hidapi::HidApi::new().unwrap();
    let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();

    device.on_button_events(|event| {
        println!("Button {} changed to {:?}", event.button_id, event.state)
    }).unwrap();
}