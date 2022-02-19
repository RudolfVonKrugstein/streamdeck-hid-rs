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
    //! The name of the Streamdeck type, as human readable string (english).
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
}

/// Tests are a little stupid in this module, because it contains
/// mostly static data. Still, for now we have these tests.
mod test {
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
}
