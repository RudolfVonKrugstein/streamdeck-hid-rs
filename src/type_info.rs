//! This module provides types and functions to get streamdeck type specific information.
//!
//! This information include things like the number of buttons but also protocol
//! specific stuff, like the packet used to reset the Streamdeck.
//!
//! The type of the streamdeck is defined in the enum [StreamDeckType]

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
}
