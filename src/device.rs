use hidapi::DeviceInfo;
use crate::StreamDeckType;
use crate::Error;

/// Lists all Streamdeck devices without opening them.
///
/// # Arguments
///
/// * 'api' - The HidApi object to use for finding the devices.
///
/// # Examples
///
/// ```
/// use streamdeck_hid_rs::list_devices;
///
/// fn main() {
///     // Create a HidApi object.
///     // We have to create it ourself, streamdeck-hid-rs does not hide
///     // that it uses this object and we could use the same instance
///     // for other hid related stuff.
///     let hidapi = hidapi::HidApi::new().unwrap();
///
///     // List devices lists the available devices without opening them.
///     // It just lists the device types and the corresponding device id.
///     let devices = list_devices(&hidapi).unwrap();
///
///     println!("List of streamdeck devices:\n");
///     for device in devices {
///         println!("{}", device.0.name());
///     }
/// }
/// ```
pub fn list_devices(api: &hidapi::HidApi) -> Result<Vec<(StreamDeckType, hidapi::DeviceInfo)>, Error> {
    let mut result: Vec<(StreamDeckType, hidapi::DeviceInfo)> = Vec::new();

    for device in api.device_list() {
        if let Some(device_type) = StreamDeckType::from_vendor_and_product_id(
            device.vendor_id(),
            device.product_id()
        ) {
            result.push((device_type, device.clone()));
        }
    }
    Ok(result)
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_list_devices() {
        todo!();
    }


}
