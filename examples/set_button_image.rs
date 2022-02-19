use streamdeck_hid_rs::StreamDeckDevice;

fn main() {
    let hidapi = hidapi::HidApi::new().unwrap();
    let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    let image = image::RgbImage::new(
        device.device_type.button_image_size().0,
        device.device_type.button_image_size().1
    );
    let red: image::Rgb<u8> = image::Rgb::from([255, 0, 0]);
    let image = imageproc::drawing::draw_filled_circle(
        &image,
        ((device.device_type.button_image_size().0/2) as i32,
         (device.device_type.button_image_size().1/2) as i32),
        (device.device_type.button_image_size().1 as f32/2.1) as i32,
        red
    );

    device.set_brightness(100);

    for button_idx in 0 .. device.device_type.total_num_buttons() {
        device.set_button_image(button_idx as u8, &image).unwrap();
    }
}