//! Module to create packages for images send to streamdeck devices.

use crate::ImageTransformation::{Rotate180, Rotate270};
use crate::{Error, StreamDeckImageFormat, StreamDeckType};
use image::codecs::bmp::BmpEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::{imageops, ColorType, EncodableLayout, ImageResult, RgbImage};
use std::cmp::min;

/// Create an package from an image to send to a streamdeck device.
///
/// # Arguments
///
/// * 'device_type' - The type of Streamdeck device
/// * 'image' - The image as an RGB image. Must be already in correct dimensions!
/// * 'btn_index' - The index of the button for which the image shold be set.
pub fn image_packages(
    device_type: StreamDeckType,
    image: &RgbImage,
    btn_index: u8,
) -> Result<Vec<Vec<u8>>, Error> {
    // Check image dimensions
    if image.width() != device_type.button_image_size().0
        || image.height() != device_type.button_image_size().1
    {
        return Err(Error::DimensionMismatch(
            device_type.button_image_size().0,
            device_type.button_image_size().1,
        ));
    }

    // Transform the image, depending on the deck type
    let image = match device_type.button_image_transformation() {
        Rotate180 => imageops::rotate180(image),
        Rotate270 => imageops::rotate270(image),
    };

    // Encode the image!
    let mut encoded_image = vec![0u8; 0];
    let encode_result = match device_type.button_image_format() {
        StreamDeckImageFormat::Bmp => BmpEncoder::new(&mut encoded_image).encode(
            image.as_bytes(),
            device_type.button_image_size().0,
            device_type.button_image_size().1,
            ColorType::Rgb8,
        ),
        StreamDeckImageFormat::Jpeg => JpegEncoder::new_with_quality(&mut encoded_image, 100)
            .encode(
                image.as_bytes(),
                device_type.button_image_size().0,
                device_type.button_image_size().1,
                ColorType::Rgb8,
            ),
    };
    if let ImageResult::Err(e) = encode_result {
        return Err(Error::ImageEncodingError(e));
    }
    // The resulting list of packages
    let mut result: Vec<Vec<u8>> = Vec::new();

    let mut bytes_remaining = encoded_image.len();
    let mut page_number = 0;

    while bytes_remaining > 0 {
        // Our current package
        let mut package = vec![0; device_type.image_package_size()];
        let payload_size = min(device_type.max_payload_size(), bytes_remaining);

        let header = device_type.image_package_header(payload_size, btn_index, page_number);
        // // Add the image header, but only on the first package
        // let img_header = if page_number == 0 { image_header(t) } else { &[] };

        package[..header.len()].copy_from_slice(&header);
        // package[header.len() .. header.len() + img_header.len()].copy_from_slice(
        //     img_header
        // );

        let taken_space = header.len(); // + img_header.len();

        let bytes_send = encoded_image.len() - bytes_remaining;
        package[taken_space..taken_space + payload_size]
            .copy_from_slice(&encoded_image[bytes_send..bytes_send + payload_size]);

        bytes_remaining -= payload_size;
        page_number += 1;

        result.push(package);
    }
    Ok(result)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::StreamDeckType;

    #[test]
    fn test_image_packer_accept_correct_dimensions() {
        for device_type in StreamDeckType::ALL {
            let image = image::RgbImage::new(
                device_type.button_image_size().0,
                device_type.button_image_size().1,
            );
            assert!(image_packages(device_type, &image, 1).is_ok());
        }
    }

    #[test]
    fn test_image_packer_fail_incorrect_dimensions() {
        for device_type in StreamDeckType::ALL {
            let image = image::RgbImage::new(
                device_type.button_image_size().0 + 1,
                device_type.button_image_size().1 + 1,
            );
            assert!(image_packages(device_type, &image, 1).is_err());
        }
    }

    #[test]
    fn test_image_packer_header() {
        for device_type in StreamDeckType::ALL {
            let image = image::RgbImage::new(
                device_type.button_image_size().0,
                device_type.button_image_size().1,
            );
            let correct_header = device_type.image_package_header(0, 0, 0);
            let packages = image_packages(device_type, &image, 1).unwrap();
            assert_eq!(packages[0][0], correct_header[0]);
            assert_eq!(packages[0][0], correct_header[0]);
        }
    }

    #[test]
    fn test_image_packer_encoding() {
        for device_type in StreamDeckType::ALL {
            let image = image::RgbImage::new(
                device_type.button_image_size().0,
                device_type.button_image_size().1,
            );
            let correct_header = device_type.image_package_header(0, 0, 0);
            let packages = image_packages(device_type.clone(), &image, 1).unwrap();

            // We just test if the first bytes are correctly set
            match &device_type.button_image_format() {
                StreamDeckImageFormat::Bmp => {
                    assert_eq!(packages[0][correct_header.len()], 66);
                    assert_eq!(packages[0][correct_header.len() + 1], 77);
                }
                StreamDeckImageFormat::Jpeg => {
                    assert_eq!(packages[0][correct_header.len()], 255);
                    assert_eq!(packages[0][correct_header.len() + 1], 216);
                }
            }
        }
    }

    #[test]
    fn test_image_packer_num_pages() {
        for device_type in StreamDeckType::ALL {
            let image = image::RgbImage::new(
                device_type.button_image_size().0,
                device_type.button_image_size().1,
            );

            let packages = image_packages(device_type.clone(), &image, 1).unwrap();

            match &device_type {
                StreamDeckType::Xl => {
                    assert_eq!(packages.len(), 1)
                }
                StreamDeckType::MK2 => {
                    assert_eq!(packages.len(), 1)
                }
                StreamDeckType::OrigV2 => {
                    assert_eq!(packages.len(), 1)
                }
                StreamDeckType::Orig => {
                    assert_eq!(packages.len(), 2)
                }
                StreamDeckType::Mini => {
                    assert_eq!(packages.len(), 3)
                }
            }
        }
    }
}
