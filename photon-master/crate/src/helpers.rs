//! Helper functions for converting between various formats

use crate::{PhotonImage, Rgb};
use image::DynamicImage::ImageRgba8;
use image::{DynamicImage, ImageBuffer};
use std::borrow::Cow;

#[cfg(feature = "enable_wasm")]
extern crate wasm_bindgen;

/// Gets the square distance between two colours
pub fn square_distance(color1: Rgb, color2: Rgb) -> i32 {
    let (r1, g1, b1) = (color1.r as i32, color1.g as i32, color1.b as i32);
    let (r2, g2, b2) = (color2.r as i32, color2.g as i32, color2.b as i32);
    i32::pow(r1 - r2, 2) + i32::pow(g1 - g2, 2) + i32::pow(b1 - b2, 2)
}

// Read a DynamicImage from a given path.
pub fn open_dyn_image(img_path: &'static str) -> DynamicImage {
    image::open(img_path).unwrap()
}

/// Save a DynamicImage to a path.
pub fn save_dyn_image(img: DynamicImage, filtered_img_path: &str) {
    // let raw_pixels = img.raw_pixels;
    // let width = img.width;
    // let height = img.height;

    // let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    // let dynimage = image::ImageRgba8(img_buffer);

    img.save(filtered_img_path).unwrap();
}

/// Get raw pixels (as a vec of u8s) from a DynamicImage
pub fn get_pixels(img: DynamicImage) -> Vec<u8> {
    // get an image's raw pixels, and return as a vec of u8s
    img.into_bytes()
}

/// Convert a PhotonImage to a DynamicImage type (struct used by the `image` crate)
/// 
/// This version clones the pixel data, which is necessary when the image needs to be modified.
/// For read-only operations, consider using `dyn_image_from_raw_borrowed` to avoid cloning.
pub fn dyn_image_from_raw(photon_image: &PhotonImage) -> DynamicImage {
    // convert a vec of raw pixels (as u8s) to a DynamicImage type
    let raw_pixels = &photon_image.raw_pixels;
    let img_buffer = ImageBuffer::from_vec(
        photon_image.width,
        photon_image.height,
        raw_pixels.clone(),
    )
    .unwrap();
    ImageRgba8(img_buffer)
}

/// Convert a PhotonImage to a DynamicImage type without cloning pixel data.
/// 
/// This version uses Cow to avoid unnecessary cloning when the image is only used for reading.
/// The returned DynamicImage will take ownership of the pixel data.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage that will be consumed.
/// 
/// # Returns
/// A DynamicImage with the pixel data moved from the PhotonImage.
pub fn dyn_image_from_raw_owned(photon_image: PhotonImage) -> DynamicImage {
    let img_buffer = ImageBuffer::from_vec(
        photon_image.width,
        photon_image.height,
        photon_image.raw_pixels,
    )
    .unwrap();
    ImageRgba8(img_buffer)
}

/// Get a borrowed view of the pixel data from a PhotonImage.
/// 
/// This function returns a Cow that borrows the pixel data when possible,
/// avoiding unnecessary allocations for read-only operations.
/// 
/// # Arguments
/// * `photon_image` - A reference to a PhotonImage.
/// 
/// # Returns
/// A Cow containing either borrowed or owned pixel data.
pub fn get_pixels_cow(photon_image: &PhotonImage) -> Cow<'_, [u8]> {
    Cow::Borrowed(&photon_image.raw_pixels)
}
