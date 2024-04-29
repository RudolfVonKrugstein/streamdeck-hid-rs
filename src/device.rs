use std::fmt;

use crate::hid_api_traits::*;
use crate::image::image_packages;
use crate::StreamDeckError;
use crate::StreamDeckType;
use image::RgbImage;
use log::debug;

/// The state a button can be in or change to.
#[derive(Clone, PartialEq, Debug)]
pub enum ButtonState {
    Down,
    Up,
}

impl fmt::Display for ButtonState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonState::Down => write!(f, "Down"),
            ButtonState::Up => write!(f, "Up"),
        }
    }
}

/// Event send, when a button changes its state!
#[derive(Debug, Clone)]
pub struct ButtonEvent {
    pub button_id: u32,
    pub state: ButtonState,
}

impl fmt::Display for ButtonEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Button-Id: {}, State: {}", self.button_id, self.state)
    }
}

pub struct StreamDeckDevice<API: HidApiTrait> {
    pub device_type: StreamDeckType,
    hid_device: API::HidDevice,
}

unsafe impl Sync for StreamDeckDevice<hidapi::HidApi> {}

impl<API: HidApiTrait> StreamDeckDevice<API> {
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
    ///     # let hidapi = streamdeck_hid_rs::hid_api_traits::create_api_mock_for_examples();
    ///
    ///     // List devices lists the available devices without opening them.
    ///     // It just lists the device types and the corresponding device id.
    ///     let devices = StreamDeckDevice::list_devices(&hidapi);
    ///
    ///     println!("List of streamdeck devices:\n");
    ///     for device in devices {
    ///         println!("{}", device.0.name());
    ///     }
    /// }
    /// ```
    pub fn list_devices(api: &API) -> Vec<(StreamDeckType, API::DeviceInfo)> {
        let mut result: Vec<(StreamDeckType, API::DeviceInfo)> = Vec::new();

        for device in api.device_list() {
            if let Some(device_type) =
                StreamDeckType::from_vendor_and_product_id(device.vendor_id(), device.product_id())
            {
                result.push((device_type, device));
            }
        }
        result
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
    ///     # let hidapi = streamdeck_hid_rs::hid_api_traits::create_api_mock_for_examples();
    ///
    ///     // List devices lists the available devices without opening them.
    ///     // It just lists the device types and the corresponding device id.
    ///     let devices = StreamDeckDevice::list_devices(&hidapi);
    ///
    ///     println!("List of streamdeck devices:\n");
    ///     for device in devices {
    ///         let device = StreamDeckDevice::open(&hidapi, &device.1);
    ///         // ... do something with device ...
    ///     }
    /// }
    /// ```
    pub fn open(
        api: &API,
        device_info: &API::DeviceInfo,
    ) -> Result<StreamDeckDevice<API>, StreamDeckError> {
        let device_type = StreamDeckType::from_vendor_and_product_id(
            device_info.vendor_id(),
            device_info.product_id(),
        );
        if let Some(device_type) = device_type {
            let hid_device = api
                .open(device_type.get_vendor_id(), device_type.get_product_id())
                .map_err(StreamDeckError::HidError)?;
            Ok(StreamDeckDevice {
                hid_device,
                device_type,
            })
        } else {
            Err(StreamDeckError::NotAStreamDeckDevice)
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
    ///     # let hidapi = streamdeck_hid_rs::hid_api_traits::create_api_mock_for_examples();
    ///
    ///     // List devices lists the available devices without opening them.
    ///     // It just lists the device types and the corresponding device id.
    ///     let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    ///     // ... do something with device ...
    /// }
    /// ```
    pub fn open_first_device(api: &API) -> Result<StreamDeckDevice<API>, StreamDeckError> {
        let mut all_devices = StreamDeckDevice::list_devices(api);
        if !all_devices.is_empty() {
            return StreamDeckDevice::open(api, &all_devices.remove(0).1);
        }
        Err(StreamDeckError::NoDeviceFound)
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
    ///     # let hidapi = streamdeck_hid_rs::hid_api_traits::create_api_mock_for_examples();
    ///     let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    ///     // make it completely dark!
    ///     device.set_brightness(0);
    /// }
    /// ```
    pub fn set_brightness(&self, brightness: u8) -> Result<(), StreamDeckError> {
        self.hid_device
            .send_feature_report(&self.device_type.brightness_packet(brightness))
            .map_err(StreamDeckError::HidError)?;
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
    ///     # let hidapi = streamdeck_hid_rs::hid_api_traits::create_api_mock_for_examples();
    ///     let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    ///     // Ensure communication is reseted
    ///     device.reset();
    ///     // More things with the device
    /// }
    /// ```
    pub fn reset(&self) -> Result<(), StreamDeckError> {
        self.hid_device
            .write(&self.device_type.reset_key_stream_packet())
            .map_err(StreamDeckError::HidError)?;
        self.hid_device
            .send_feature_report(self.device_type.reset_packet())
            .map_err(StreamDeckError::HidError)?;
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
    ///     # let hidapi = streamdeck_hid_rs::hid_api_traits::create_api_mock_for_examples();
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
    pub fn set_button_image(&self, button_id: u8, image: &RgbImage) -> Result<(), StreamDeckError> {
        let image_packages = image_packages(self.device_type.clone(), image, button_id)?;
        for image_package in image_packages {
            let image_package_len = image_package.len();
            let result = self
                .hid_device
                .write(&image_package)
                .map_err(StreamDeckError::HidError)?;
            if result != image_package_len {
                return Err(StreamDeckError::IncorrectWriteLengthError);
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
    ///     # let hidapi = streamdeck_hid_rs::hid_api_traits::create_api_mock_for_examples();
    ///     let device = StreamDeckDevice::open_first_device(&hidapi).unwrap();
    ///
    ///     // device.on_button_events(|event| {
    ///     //    println!("Button {} changed to {:?}", event.button_id, event.state)
    ///     // }).unwrap();
    /// }
    ///
    pub fn on_button_events<F>(&self, cb: F) -> Result<(), StreamDeckError>
    where
        F: Fn(ButtonEvent),
    {
        let length: usize =
            self.device_type.button_read_offset() + self.device_type.total_num_buttons() as usize;
        let mut inbuffer = vec![0; length];

        let mut button_state = vec![ButtonState::Up; self.device_type.total_num_buttons() as usize];

        loop {
            match self.hid_device.read(&mut inbuffer) {
                Result::Ok(_) => {}
                Result::Err(e) => return Err(StreamDeckError::HidError(e)),
            };
            debug!("Streamdeck read: {:?}", inbuffer);
            for button_id in 0..self.device_type.total_num_buttons() {
                if inbuffer[button_id + self.device_type.button_read_offset()] == 0 {
                    if button_state[button_id] == ButtonState::Down {
                        cb(ButtonEvent {
                            button_id: button_id as u32,
                            state: ButtonState::Up,
                        });
                        button_state[button_id] = ButtonState::Up;
                    }
                } else if button_state[button_id] == ButtonState::Up {
                    cb(ButtonEvent {
                        button_id: button_id as u32,
                        state: ButtonState::Down,
                    });
                    button_state[button_id] = ButtonState::Down;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::StreamDeckError::HidError;
    #[allow(unused_imports)]
    use mockall::predicate::*;
    #[allow(unused_imports)]
    use mockall::*;

    #[test]
    fn test_list_devices_empty() {
        // Setup
        let mut api_mock = MockMockHidApi::new();
        api_mock.expect_device_list().times(1).returning(Vec::new);

        // Act
        let devices = StreamDeckDevice::list_devices(&api_mock);

        // Test
        assert_eq!(devices.len(), 0);
    }

    #[test]
    fn test_list_devices_non_streamdeck_empty() {
        // Setup
        let mut api_mock = MockMockHidApi::new();
        api_mock.expect_device_list().times(1).returning(|| {
            let mut info_mock = MockDeviceInfoTrait::new();
            info_mock.expect_vendor_id().returning(|| 1);
            info_mock.expect_product_id().returning(|| 1);
            Vec::from([info_mock])
        });

        // Act
        let devices = StreamDeckDevice::list_devices(&api_mock);

        // Test
        assert_eq!(devices.len(), 0);
    }

    #[test]
    fn test_list_devices_streamdeck_device() {
        // Setup
        let mut api_mock = MockMockHidApi::new();
        api_mock.expect_device_list().times(1).returning(|| {
            let mut wrong_info_mock = MockDeviceInfoTrait::new();
            wrong_info_mock.expect_vendor_id().returning(|| 1);
            wrong_info_mock.expect_product_id().returning(|| 1);
            let mut correct_info_mock = MockDeviceInfoTrait::new();
            correct_info_mock
                .expect_vendor_id()
                .returning(|| StreamDeckType::Xl.get_vendor_id());
            correct_info_mock
                .expect_product_id()
                .returning(|| StreamDeckType::Xl.get_product_id());
            Vec::from([wrong_info_mock, correct_info_mock])
        });

        // Act
        let devices = StreamDeckDevice::list_devices(&api_mock);

        // Test
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].0, StreamDeckType::Xl);
    }
}
