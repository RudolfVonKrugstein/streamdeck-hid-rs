//! Example illustrating how to list Streamdecks using
//! this library.
//!

fn main() {
    //! Create a HidApi object.
    //! We have to create it ourself, streamdeck-hid-rs does not hide
    //! that it uses this object and we could use the same instance
    //! for other hid related stuff.
    let hidapi = hidapi::HidApi::new().unwrap();

    //! List devices lists the available devices without opening them.
    //! It just lists the device types and the corresponding device id.
    let devices = list_devices(&mut hidapi);

    println!("List of streamdeck devices:\n");
    for device in devices {
        println!("{} device with id {}", device.0.name(), device.1);
    }
}
