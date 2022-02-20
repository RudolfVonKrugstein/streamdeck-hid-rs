//! Traits for mocking the hid api. Used for tests.
use crate::StreamDeckType;
use mockall::predicate::*;
use mockall::*;

/// Trait to make HidApi testable for us!
#[automock]
pub trait DeviceInfoTrait {
    fn vendor_id(&self) -> u16;
    fn product_id(&self) -> u16;
}

impl DeviceInfoTrait for hidapi::DeviceInfo {
    fn vendor_id(&self) -> u16 {
        self.vendor_id()
    }

    fn product_id(&self) -> u16 {
        self.product_id()
    }
}

#[automock]
pub trait HidDeviceTrait {
    fn send_feature_report(&self, data: &[u8]) -> hidapi::HidResult<()>;
    fn write(&self, data: &[u8]) -> hidapi::HidResult<usize>;
    fn read(&self, buf: &mut [u8]) -> hidapi::HidResult<usize>;
}

impl HidDeviceTrait for hidapi::HidDevice {
    fn send_feature_report(&self, data: &[u8]) -> hidapi::HidResult<()> {
        self.send_feature_report(data)
    }

    fn write(&self, data: &[u8]) -> hidapi::HidResult<usize> {
        self.write(data)
    }

    fn read(&self, buf: &mut [u8]) -> hidapi::HidResult<usize> {
        self.read(buf)
    }
}

pub trait HidApiTrait {
    type DeviceInfo: DeviceInfoTrait;
    type HidDevice: HidDeviceTrait;
    fn device_list(&self) -> Vec<Self::DeviceInfo>;
    fn open(&self, vid: u16, pid: u16) -> hidapi::HidResult<Self::HidDevice>;
}

impl HidApiTrait for hidapi::HidApi {
    type DeviceInfo = hidapi::DeviceInfo;
    type HidDevice = hidapi::HidDevice;

    fn device_list(&self) -> Vec<Self::DeviceInfo> {
        let mut result = Vec::new();
        for device in self.device_list() {
            result.push(device.clone());
        }
        result
    }
    fn open(&self, vid: u16, pid: u16) -> hidapi::HidResult<Self::HidDevice> {
        self.open(vid, pid)
    }
}

mock! {
    pub MockHidApi {
    }

    impl HidApiTrait for MockHidApi {
        type DeviceInfo = MockDeviceInfoTrait;
        type HidDevice = MockHidDeviceTrait;
        fn device_list(&self) -> Vec<MockDeviceInfoTrait>;
        fn open(&self, vid: u16, pid: u16) -> hidapi::HidResult<MockHidDeviceTrait>;
    }
}

/// Create a mocked hid api to use in examples!
pub fn create_api_mock_for_examples() -> MockMockHidApi {
    let mut result = MockMockHidApi::new();
    result.expect_device_list().returning(|| {
        let mut di = MockDeviceInfoTrait::new();
        di.expect_vendor_id()
            .returning(|| StreamDeckType::Xl.get_vendor_id());
        di.expect_product_id()
            .returning(|| StreamDeckType::Xl.get_product_id());
        Vec::from([di])
    });
    result.expect_open().returning(|_vid: u16, _pid: u16| {
        let mut hd = MockHidDeviceTrait::new();
        hd.expect_send_feature_report()
            .returning(|_data: &[u8]| Ok(()));
        hd.expect_write().returning(|data: &[u8]| Ok(data.len()));
        hd.expect_read().returning(|data: &mut [u8]| Ok(data.len()));
        Ok(hd)
    });
    result
}
