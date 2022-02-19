use log::debug;
use image::RgbImage;
use crate::StreamDeckType;
use crate::Error;
use crate::image::image_packages;

/// The state a button can be in or change to.
#[derive(Clone, PartialEq, Debug)]
pub enum ButtonState {
    Down,
    Up
}

/// Event send, when a button changes its state!
#[derive(Debug, Clone)]
pub struct ButtonEvent {
    pub button_id: u32,
    pub state: ButtonState,
}

pub struct StreamDeckDevice {
    pub device_type: StreamDeckType,
    hid_device: hidapi::HidDevice,
}

impl StreamDeckDevice {
    /// Lists all Streamdeck devices without opening them.
    ///
    /// # Arguments
    ///
    /// * 'api' - The HidApi object to use for finding the devices.
    ///
    /// # Examples
    ///
    /// ```
    /// use streamdeck_hid_rs::StreamDeckDevice;
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
    ///     let devices = StreamDeckDevice::list_devices(&hidapi).unwrap();
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

    /// Open a Streamdeck device.
    ///
    /// The DeviceInfo can be taken from the return value of [list_devices].
    ///
    /// # Arguments
    ///
    /// * 'api' - The HidApi object to use for finding the devices.
    /// * 'divice_info' - The information about the device, for example taken from
    ///                   [list_devices].
    ///
    /// # Example
    ///
    /// ```
    /// use streamdeck_hid_rs::StreamDeckDevice;
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
    ///     let devices = StreamDeckDevice::list_devices(&hidapi).unwrap();
    ///
    ///     println!("List of streamdeck devices:\n");
    ///     for device in devices {
    ///         let device = StreamDeckDevice::open(&hidapi, &device.1);
    ///         // ... do something with device ...
    ///     }
    /// }
    /// ```
    pub fn open(api: &hidapi::HidApi, device_info: &hidapi::DeviceInfo) -> Result<StreamDeckDevice, Error> {
        let device_type = StreamDeckType::from_vendor_and_product_id(
            device_info.vendor_id(),
            device_info.product_id(),
        );
        if let Some(device_type) = device_type {
            let hid_device = api.open(
                device_type.get_vendor_id(),
                device_type.get_product_id()
            ).map_err(|e| Error::HidError(e))?;
            Ok(StreamDeckDevice{
                hid_device,
                device_type,
            })
        } else {
            Err(Error::NotAStreamDeckDevice)
        }
    }

    /// Open the first found StreamDeck device that is found.
    ///
    /// If there are multiple devices, just the first one is taken. Which one this is, is random.
    ///
    /// # Arguments
    ///
    /// * 'api' - The HidApi object to use for finding the devices.
    ///
    /// # Example
    ///
    /// ```
    /// use streamdeck_hid_rs::StreamDeckDevice;
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
    ///     let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    ///     // ... do something with device ...
    /// }
    /// ```
    pub fn open_first_device(api: &hidapi::HidApi) -> Result<StreamDeckDevice, Error> {
        let mut all_devices = StreamDeckDevice::list_devices(api)?;
        if !all_devices.is_empty() {
            return StreamDeckDevice::open(api, &all_devices.remove(0).1);
        }
        Err(Error::NoDeviceFound)
    }

    /// Set the brightness of the device.
    ///
    /// # Arguments
    ///
    /// * 'brighness' - The brighness to set, must be between 0 and 100.
    ///
    /// # Example
    /// ```
    /// use streamdeck_hid_rs::StreamDeckDevice;
    ///
    /// fn main() {
    ///     let hidapi = hidapi::HidApi::new().unwrap();
    ///     let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    ///     // make it completely dark!
    ///     device.set_brightness(0);
    /// }
    /// ```
    pub fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        self.hid_device.send_feature_report(
            &self.device_type.brightness_packet(brightness)
        ).map_err(|e| Error::HidError(e))?;
        Ok(())
    }

    /// Reset communication with a device.
    ///
    /// This might be needed, if the connection has been interupted or the
    /// device is in an invalid/unknown state for some other reason.
    ///
    /// # Example
    /// ```
    /// use streamdeck_hid_rs::StreamDeckDevice;
    ///
    /// fn main() {
    ///     let hidapi = hidapi::HidApi::new().unwrap();
    ///     let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    ///     // Ensure communication is reseted
    ///     device.reset();
    ///     // More things with the device
    /// }
    /// ```
    pub fn reset(&self) -> Result<(), Error> {
        self.hid_device.write(
            &self.device_type.reset_key_stream_packet())
            .map_err(|e| Error::HidError(e))?;
        self.hid_device.send_feature_report(
            self.device_type.reset_packet()
        ).map_err(|e| Error::HidError(e))?;
        Ok(())
    }

    /// Set the image for a button!
    ///
    /// Changes the image on a specific button.
    ///
    /// # Example
    /// ```
    /// use streamdeck_hid_rs::StreamDeckDevice;
    ///
    /// fn main() {
    ///     let hidapi = hidapi::HidApi::new().unwrap();
    ///     let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    ///     let mut image = image::RgbImage::new(
    ///                   device.device_type.button_image_size().0,
    ///                   device.device_type.button_image_size().1
    ///     );
    ///     // Do something with image
    ///
    ///     device.set_button_image(0, &image);
    ///     // More things with the device
    /// }
    /// ```
    pub fn set_button_image(&self, button_id: u8, image: &RgbImage) -> Result<(), Error> {
        let image_packages = image_packages(
            self.device_type.clone(),
            image,
            button_id)?;
        for image_package in image_packages {
            let image_package_len = image_package.len();
            let result = self.hid_device.write(&image_package)
                .map_err(|e| Error::HidError(e))?;
            if result != image_package_len {
                return Err(Error::IncorrectWriteLengthError)
            }
        }
        Ok(())
    }

    /// Wait for button events!
    ///
    /// The Idea is, that this runs in its own thread waiting for events on the device
    /// and calling the closure when an event occurs.
    ///
    /// # Example
    /// ```
    /// use streamdeck_hid_rs::StreamDeckDevice;
    ///
    /// fn main() {
    ///     let hidapi = hidapi::HidApi::new().unwrap();
    ///     let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    ///
    ///     device.on_button_events(|event| {
    ///         println!("Button {} changed to {:?}", event.button_id, event.state)
    ///     }).unwrap();
    /// }
    ///
    pub fn on_button_events<F>(&self, cb: F) -> Result<(), Error>
        where F: Fn(ButtonEvent) -> ()
    {
        let length: usize = self.device_type.button_read_offset() + self.device_type.total_num_buttons() as usize;
        let mut inbuffer = vec![0; length];

        let mut button_state = vec![ButtonState::Up; self.device_type.total_num_buttons() as usize];

        loop {
            match self.hid_device.read(&mut inbuffer) {
                Result::Ok(_) => {},
                Result::Err(e) => {
                    return Err(Error::HidError(e))
                }
            };
            debug!("Streamdeck read: {:?}", inbuffer);
            for button_id in 0..self.device_type.total_num_buttons() {
                if inbuffer[button_id + self.device_type.button_read_offset()] == 0 {
                    if button_state[button_id] == ButtonState::Down {
                        cb(ButtonEvent {
                            button_id: button_id as u32,
                            state: ButtonState::Up
                        });
                        button_state[button_id] = ButtonState::Up;
                    }
                } else {
                    if button_state[button_id] == ButtonState::Up {
                        cb(ButtonEvent {
                            button_id: button_id as u32,
                            state: ButtonState::Down
                        });
                        button_state[button_id] = ButtonState::Down;
                    }
                }
            }
        }
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_list_devices() {
        todo!();
    }

    #[test]
    fn test_set_brightness() {
        todo!();
    }

    #[test]
    fn test_reset() {
        todo!();
    }

    #[test]
    fn test_set_button_image() {
        todo!();
    }
}
