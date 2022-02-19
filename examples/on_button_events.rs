use streamdeck_hid_rs::StreamDeckDevice;

fn main() {
    let mut hidapi = hidapi::HidApi::new().unwrap();
    let device = StreamDeckDevice::open_first_device(&mut hidapi).unwrap();

    device.on_button_events(|event| {
        println!("Button {} changed to {:?}", event.button_id, event.state)
    }).unwrap();
}