//! This module provides types and functions to get streamdeck type specific information.
//!
//! This information include things like the number of buttons but also protocol
//! specific stuff, like the packet used to reset the Streamdeck.
//!
//! The type of the streamdeck is defined in the enum [StreamDeckType]

use std::cmp::min;

/// Type of Streamdeck device.
///
/// This enum defined the types of Streamdeck devices known to this library.
#[derive(PartialEq, Debug)]
pub enum StreamDeckType {
    Xl,
    OrigV2,
    Orig,
    Mini,
}

/// The image formats a Streamdeck can use.
///
/// The streamdecks uses different image formats depending on the
/// type.
#[derive(PartialEq, Debug)]
pub enum StreamDeckImageFormat {
    Jpeg,
    Bmp,
}

/// The transformation an image needs to make to be correctly displayed on the screen.
///
/// This enum contains only those transformations ever needed on streamdecks.
#[derive(PartialEq, Debug)]
pub enum ImageTransformation {
    Rotate180,
    Rotate270,
}

/// The implementation of the [StreamDeckType] provides
/// functions to get information specific to the StreamDeck type.
impl StreamDeckType {
    /// List of ALL possible types
    const ALL: [StreamDeckType; 4] = [
        StreamDeckType::Xl,
        StreamDeckType::OrigV2,
        StreamDeckType::Orig,
        StreamDeckType::Mini,
    ];

    /// The name of the Streamdeck type, as human readable string (english).
    pub fn name(&self) -> &'static str {
        match *self {
            StreamDeckType::Xl => "Streamdeck XL",
            StreamDeckType::OrigV2 => "Streamdeck (original v2)",
            StreamDeckType::Orig => "Streamdeck original",
            StreamDeckType::Mini => "Streamdeck Mini",
        }
    }

    /// The number of buttons found on the streamdeck.
    ///
    /// This function returns a typle, where the first value is the number of rows and
    /// the second the number of columns.
    pub fn num_buttons(&self) -> (u32, u32) {
        match *self {
            StreamDeckType::Xl => (4, 8),
            StreamDeckType::OrigV2 => (3, 5),
            StreamDeckType::Orig => (3, 5),
            StreamDeckType::Mini => (2, 3),
        }
    }

    /// The total number of buttons found on the streamdeck.
    pub fn total_num_buttons(&self) -> usize {
        let (x, y) = self.num_buttons();
        (x * y) as usize
    }

    /// The image format used by the Streamdeck.
    pub fn button_image_format(&self) -> StreamDeckImageFormat {
        match *self {
            StreamDeckType::Xl => StreamDeckImageFormat::Jpeg,
            StreamDeckType::OrigV2 => StreamDeckImageFormat::Jpeg,
            StreamDeckType::Orig => StreamDeckImageFormat::Bmp,
            StreamDeckType::Mini => StreamDeckImageFormat::Bmp,
        }
    }

    /// The expected size of the image when uploading images for the buttons.
    ///
    /// The size is returned as tuple for expected width and height of the image.
    pub fn button_image_size(&self) -> (u32, u32) {
        match *self {
            StreamDeckType::Xl => (96, 96),
            StreamDeckType::OrigV2 => (72, 72),
            StreamDeckType::Orig => (72, 72),
            StreamDeckType::Mini => (80, 80),
        }
    }

    /// Get the product id.
    ///
    /// Get the product id for this Streamdeck device (to compare with the
    /// product_id returned by [HidApi]).
    pub fn get_product_id(&self) -> u16 {
        match *self {
            StreamDeckType::Xl => 0x6c,
            StreamDeckType::OrigV2 => 0x6d,
            StreamDeckType::Orig => 0x60,
            StreamDeckType::Mini => 0x63,
        }
    }

    /// Get the vendor id.
    ///
    /// Get the vendor id for this Streamdeck device (to compare with the
    /// vendor_id returned by [HidApi]).
    pub fn get_vendor_id(&self) -> u16 {
        // For now its always the same
        0x0fd9
    }

    /// Get a type from vendor and product id.
    ///
    /// Returns the Streamdeck type from vendor and product id.
    pub fn from_vendor_and_product_id(vendor_id: u16, product_id: u16) -> Option<StreamDeckType> {
        for t in StreamDeckType::ALL {
            if t.get_vendor_id() == vendor_id && t.get_product_id() == product_id {
                return Some(t);
            }
        }
        None
    }

    /// Returns the byte packet to be used to set the brightness of the device.
    pub(crate) fn brightness_packet(&self, brightness: u8) -> Vec<u8> {
        match *self {
            StreamDeckType::Xl=> {
                let mut cmd = vec![0u8; 32];
                cmd[..3].copy_from_slice(&[0x03, 0x08, brightness]);
                cmd
            }
            StreamDeckType::OrigV2=> {
                let mut cmd = vec![0u8; 32];
                cmd[..3].copy_from_slice(&[0x03, 0x08, brightness]);
                cmd
            }
            StreamDeckType::Orig=> {
                let mut cmd = vec![0u8; 17];
                cmd[..6].copy_from_slice(&[0x05, 0x55, 0xaa, 0xd1, 0x01, brightness]);
                cmd
            }
            StreamDeckType::Mini => {
                let mut cmd = vec![0u8; 17];
                cmd[..6].copy_from_slice(&[0x05, 0x55, 0xaa, 0xd1, 0x01, brightness]);
                cmd
            }
        }
    }

    /// General, reused reset packet
    const RESET_PACKET_17: [u8;17]  = [0x0b, 0x63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    /// General, reused reset packet
    const RESET_PACKET_32: [u8;32] = [0x03, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    /// Returns the byte packet to reset the device.
    pub(crate) fn reset_packet(&self) -> &'static [u8] {
        match *self {
            StreamDeckType::Xl=> &StreamDeckType::RESET_PACKET_32,
            StreamDeckType::OrigV2=> &StreamDeckType::RESET_PACKET_32,
            StreamDeckType::Orig=> &StreamDeckType::RESET_PACKET_17,
            StreamDeckType::Mini => &StreamDeckType::RESET_PACKET_17
        }
    }

    /// Package to reset the key stream communication.
    pub(crate) fn reset_key_stream_packet(&self) -> Vec<u8> {
        let mut r = vec![0; self.image_package_size()];
        r[0] = 2;
        r
    }

    /// How big is an button image package for this device?
    pub(crate) fn image_package_size(&self) -> usize {
        match *self {
            StreamDeckType::Xl=> 1024,
            StreamDeckType::OrigV2=> 1024,
            StreamDeckType::Orig=> 8191,
            StreamDeckType::Mini => 8191
        }
    }

    /// Header for image packages send to set images on buttons.
    pub(crate) fn image_package_header(
        &self, bytes_remaining: usize, btn_index: u8, page_number: u16
    ) -> Vec<u8> {
        match *self {
            StreamDeckType::Xl | StreamDeckType::OrigV2 => {
                let length = min(self.image_package_size(), bytes_remaining);
                vec![
                    0x2,
                    0x7,
                    btn_index,
                    if length == bytes_remaining {0x01} else {0x00},
                    (length & 0xFF) as u8,
                    (length >> 8) as u8,
                    (page_number & 0xFF) as u8,
                    (page_number >> 8) as u8,
                ]
            },
            StreamDeckType::Mini | StreamDeckType::Orig => {
                let _length = min(self.image_package_size(), bytes_remaining);
                vec![
                    0x02,
                    0x01,
                    (page_number + 1) as u8,
                    0,
                    if page_number == 1 {0x01} else {0x00},
                    (btn_index + 1) as u8,
                    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                ]
            }
        }
    }

    /// Tansformation needed to display the image correctly
    pub(crate) fn button_image_transformation(&self) -> ImageTransformation {
        match *self {
            StreamDeckType::Xl => ImageTransformation::Rotate180,
            StreamDeckType::OrigV2 => ImageTransformation::Rotate180,
            StreamDeckType::Orig => ImageTransformation::Rotate180,
            StreamDeckType::Mini => ImageTransformation::Rotate270,
        }
    }

    /// Maximum payload per packet for the device
    pub(crate) fn max_payload_size(&self) -> usize {
        match *self {
            StreamDeckType::Xl => self.image_package_size() - 8,
            StreamDeckType::OrigV2 => self.image_package_size() - 8,
            StreamDeckType::Orig => 7803,
            StreamDeckType::Mini => 7803
        }
    }
}

/// Tests are a little stupid in this module, because it contains
/// mostly static data. Still, for now we have these tests.
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_name() {
        assert!(StreamDeckType::Xl.name().contains("XL"));
        assert!(StreamDeckType::OrigV2.name().contains("(original v2)"));
        assert!(StreamDeckType::Orig.name().contains("original"));
        assert!(StreamDeckType::Mini.name().contains("Mini"));
    }

    #[test]
    fn test_num_buttons() {
        assert_eq!(StreamDeckType::Xl.num_buttons(), (4, 8));
        assert_eq!(StreamDeckType::OrigV2.num_buttons(), (3, 5));
        assert_eq!(StreamDeckType::Orig.num_buttons(), (3, 5));
        assert_eq!(StreamDeckType::Mini.num_buttons(), (2, 3));
    }

    #[test]
    fn test_total_buttons() {
        assert_eq!(StreamDeckType::Xl.total_num_buttons(), 32);
        assert_eq!(StreamDeckType::OrigV2.total_num_buttons(), 15);
        assert_eq!(StreamDeckType::Orig.total_num_buttons(), 15);
        assert_eq!(StreamDeckType::Mini.total_num_buttons(), 6);
    }

    #[test]
    fn test_button_image_format() {
        assert_eq!(
            StreamDeckType::Xl.button_image_format(),
            StreamDeckImageFormat::Jpeg
        );
        assert_eq!(
            StreamDeckType::OrigV2.button_image_format(),
            StreamDeckImageFormat::Jpeg
        );
        assert_eq!(
            StreamDeckType::Orig.button_image_format(),
            StreamDeckImageFormat::Bmp
        );
        assert_eq!(
            StreamDeckType::Mini.button_image_format(),
            StreamDeckImageFormat::Bmp
        );
    }

    #[test]
    fn test_button_image_size() {
        assert_eq!(StreamDeckType::Xl.button_image_size(), (96, 96));
        assert_eq!(StreamDeckType::OrigV2.button_image_size(), (72, 72));
        assert_eq!(StreamDeckType::Orig.button_image_size(), (72, 72));
        assert_eq!(StreamDeckType::Mini.button_image_size(), (80, 80));
    }

    #[test]
    fn test_get_type_correct() {
        assert_eq!(
            StreamDeckType::from_vendor_and_product_id(0x0fd9, 0x63),
            Some(StreamDeckType::Mini)
        );
        assert_eq!(
            StreamDeckType::from_vendor_and_product_id(0x0fd9, 0x60),
            Some(StreamDeckType::Orig)
        );
        assert_eq!(
            StreamDeckType::from_vendor_and_product_id(0x0fd9, 0x6d),
            Some(StreamDeckType::OrigV2)
        );
        assert_eq!(
            StreamDeckType::from_vendor_and_product_id(0x0fd9, 0x6c),
            Some(StreamDeckType::Xl)
        );
    }

    #[test]
    fn test_get_type_incorrect() {
        for t in StreamDeckType::ALL {
            assert_eq!(
                StreamDeckType::from_vendor_and_product_id(0xf334, t.get_product_id()),
                None,
            );
            assert_eq!(
                StreamDeckType::from_vendor_and_product_id( t.get_product_id(), 0xf334),
                None,
            );
        }
    }

    #[test]
    fn test_brightness_packet() {
        // We only test the brightness byte ... the rest is constants
        assert_eq!(StreamDeckType::Xl.brightness_packet(22)[2], 22);
        assert_eq!(StreamDeckType::OrigV2.brightness_packet(23)[2], 23);
        assert_eq!(StreamDeckType::Orig.brightness_packet(34)[5], 34);
        assert_eq!(StreamDeckType::Mini.brightness_packet(35)[5], 35);
    }

    #[test]
    fn test_reset_packet() {
        assert_eq!(StreamDeckType::Xl.reset_packet()[0], 0x03);
        assert_eq!(StreamDeckType::OrigV2.reset_packet()[0], 0x03);
        assert_eq!(StreamDeckType::Orig.reset_packet()[0], 0x0b);
        assert_eq!(StreamDeckType::Mini.reset_packet()[0], 0x0b);
    }

    #[test]
    fn test_reset_keystream_package() {
        assert_eq!(StreamDeckType::Xl.reset_key_stream_packet()[0], 2);
        assert_eq!(StreamDeckType::OrigV2.reset_key_stream_packet()[0], 2);
        assert_eq!(StreamDeckType::Orig.reset_key_stream_packet()[0], 2);
        assert_eq!(StreamDeckType::Mini.reset_key_stream_packet()[0], 2);
    }

    #[test]
    fn test_image_package_size() {
        assert_eq!(StreamDeckType::Xl.image_package_size(), 1024);
        assert_eq!(StreamDeckType::OrigV2.image_package_size(), 1024);
        assert_eq!(StreamDeckType::Orig.image_package_size(), 8191);
        assert_eq!(StreamDeckType::Mini.image_package_size(), 8191);
    }

    #[test]
    fn test_image_package_header_button_index() {
        for btn_index in 0..6 {
            assert_eq!(StreamDeckType::Xl.image_package_header(700, btn_index.clone(), 1)[2], btn_index.clone());
            assert_eq!(StreamDeckType::OrigV2.image_package_header(700, btn_index.clone(), 1)[2], btn_index.clone());
            assert_eq!(StreamDeckType::Orig.image_package_header(700, btn_index.clone(), 1)[5], (btn_index + 1) as u8);
            assert_eq!(StreamDeckType::Mini.image_package_header(700, btn_index.clone(), 1)[5], (btn_index + 1) as u8);
        }
    }

    #[test]
    fn test_image_package_header_page_number() {
        for page_number in 0..300 {
            assert_eq!(StreamDeckType::Xl.image_package_header(700, 1, page_number.clone())[6], (page_number.clone() & 0xFF) as u8);
            assert_eq!(StreamDeckType::Xl.image_package_header(700, 1, page_number.clone())[7], (page_number.clone() >> 8) as u8);

            assert_eq!(StreamDeckType::OrigV2.image_package_header(700, 1, page_number.clone())[6], (page_number.clone() & 0xFF) as u8);
            assert_eq!(StreamDeckType::OrigV2.image_package_header(700, 1, page_number.clone())[7], (page_number.clone() >> 8) as u8);

            assert_eq!(StreamDeckType::Orig.image_package_header(700, 1, page_number.clone())[2], (page_number.clone() + 1) as u8);
            assert_eq!(StreamDeckType::Orig.image_package_header(700, 1, page_number.clone())[4], if page_number == 1 {0x01} else {0x00});

            assert_eq!(StreamDeckType::Mini.image_package_header(700, 1, page_number.clone())[2], (page_number.clone() + 1) as u8);
            assert_eq!(StreamDeckType::Mini.image_package_header(700, 1, page_number.clone())[4], if page_number == 1 {0x01} else {0x00});

        }
    }

    #[test]
    fn test_button_image_transformation() {
        assert_eq!(StreamDeckType::Xl.button_image_transformation(), ImageTransformation::Rotate180);
        assert_eq!(StreamDeckType::OrigV2.button_image_transformation(), ImageTransformation::Rotate180);
        assert_eq!(StreamDeckType::Orig.button_image_transformation(), ImageTransformation::Rotate180);
        assert_eq!(StreamDeckType::Mini.button_image_transformation(), ImageTransformation::Rotate270);
    }

    #[test]
    fn max_payload_size() {
        assert_eq!(StreamDeckType::Xl.max_payload_size(), 1024-8);
        assert_eq!(StreamDeckType::OrigV2.max_payload_size(), 1024-8);
        assert_eq!(StreamDeckType::Orig.max_payload_size(), 7803);
        assert_eq!(StreamDeckType::Mini.max_payload_size(), 7803);
    }
}
