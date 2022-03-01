use std::sync::Arc;
use std::thread;
use streamdeck_hid_rs::StreamDeckDevice;

fn main() {
    let mut hidapi = hidapi::HidApi::new().unwrap();
    let device = Arc::new(StreamDeckDevice::open_first_device(&mut hidapi).unwrap());
    let image = image::RgbImage::new(
        device.device_type.button_image_size().0,
        device.device_type.button_image_size().1,
    );
    // let red: image::Rgb<u8> = image::Rgb::from([255, 0, 0]);
    // let image = imageproc::drawing::draw_filled_circle(
    //     &image,
    //     ((device.device_type.button_image_size().0/2) as i32,
    //      (device.device_type.button_image_size().1/2) as i32),
    //     (device.device_type.button_image_size().1 as f32/2.1) as i32,
    //     red
    // );
    let send_device = device.clone();
    let t = thread::spawn(move || {
        send_device.on_button_events(|event| {
           println!("Button {} changed to {:?}", event.button_id, event.state);
       }).unwrap();
    });

    device.set_brightness(100).unwrap();

    for button_idx in 0..device.device_type.total_num_buttons() {
        device.set_button_image(button_idx as u8, &image).unwrap();
    }
    t.join().unwrap();
}
