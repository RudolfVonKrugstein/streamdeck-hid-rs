//! Module to create packages for images send to streamdeck devices.

use std::cmp::min;
use image::{ColorType, EncodableLayout, imageops, ImageResult, RgbImage};
use image::codecs::bmp::BmpEncoder;
use image::codecs::jpeg::JpegEncoder;
use crate::{Error, StreamDeckImageFormat, StreamDeckType};
use crate::ImageTransformation::{Rotate180, Rotate270};

/// Create an package from an image to send to a streamdeck device.
///
/// # Arguments
///
/// * 'device_type' - The type of Streamdeck device
/// * 'image' - The image as an RGB image. Must be already in correct dimensions!
/// * 'btn_index' - The index of the button for which the image shold be set.
pub fn image_packages(device_type: StreamDeckType, image: &RgbImage, btn_index: u8) -> Result<Vec<Vec<u8>>, Error> {
    // Check image dimensions
    if image.width() != device_type.button_image_size().0 || image.height() != device_type.button_image_size().1 {
        return Err(Error::DimensionMismatch(device_type.button_image_size().0, device_type.button_image_size().1))
    }

    // Transform the image, depending on the deck type
    let image = match device_type.button_image_transformation() {
        Rotate180 => imageops::rotate180(image),
        Rotate270 => imageops::rotate270(image)
    };

    // Encode the image!
    let mut encoded_image = vec![0u8;0];
    let encode_result = match device_type.button_image_format() {
        StreamDeckImageFormat::Bmp => {
            BmpEncoder::new(&mut encoded_image).encode(
                image.as_bytes(),
                device_type.button_image_size().0,
                device_type.button_image_size().1,
                ColorType::Rgb8
            )
        }
        StreamDeckImageFormat::Jpeg => {
            JpegEncoder::new_with_quality(&mut encoded_image, 100).encode(
                image.as_bytes(),
                device_type.button_image_size().0,
                device_type.button_image_size().1,
                ColorType::Rgb8
            )
        }
    };
    if let ImageResult::Err(e) = encode_result { return Err(Error::ImageEncodingError(e)) }
    // The resulting list of packages
    let mut result: Vec<Vec<u8>> = Vec::new();

    let mut bytes_remaining = encoded_image.len();
    let mut page_number = 0;

    while bytes_remaining > 0 {
        // Our current package
        let mut package = vec![0; device_type.image_package_size()];
        let payload_size = min(device_type.max_payload_size(), bytes_remaining);

        let header = device_type.image_package_header(
            payload_size, btn_index, page_number
        );
        // // Add the image header, but only on the first package
        // let img_header = if page_number == 0 { image_header(t) } else { &[] };

        package[..header.len()].copy_from_slice(&header);
        // package[header.len() .. header.len() + img_header.len()].copy_from_slice(
        //     img_header
        // );

        let taken_space = header.len(); // + img_header.len();

        let bytes_send = encoded_image.len() - bytes_remaining;
        package[taken_space..taken_space + payload_size].copy_from_slice(&encoded_image[bytes_send..bytes_send + payload_size]);

        bytes_remaining -= payload_size;
        page_number += 1;

        result.push(package);
    }
    Ok(result)
}

mod tests {
    #[test]
    fn test_image_packages() {
        todo!();
    }

}