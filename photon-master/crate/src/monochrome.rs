//! Monochrome-related effects and greyscaling/duotoning.

use crate::PhotonImage;

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// Apply a monochrome effect of a certain colour.
///
/// It does so by averaging the R, G, and B values of a pixel, and then adding a
/// separate value to that averaged value for each channel to produce a tint.
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `r_offset` - The value to add to the Red channel per pixel.
/// * `g_offset` - The value to add to the Green channel per pixel.
/// * `b_offset` - The value to add to the Blue channel per pixel.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a monochrome effect to an image:
/// use photon_rs::monochrome::monochrome;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// monochrome(&mut img, 40_u32, 50_u32, 100_u32);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn monochrome(img: &mut PhotonImage, r_offset: u32, g_offset: u32, b_offset: u32) {
    crate::simd::monochrome_simd(img, r_offset, g_offset, b_offset);
}

/// Convert an image to sepia.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// # Example
///
/// ```no_run
/// // For example, to sepia an image of type `PhotonImage`:
/// use photon_rs::monochrome::sepia;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// sepia(&mut img);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn sepia(img: &mut PhotonImage) {
    crate::simd::sepia_simd(img);
}

/// Convert an image to grayscale using the conventional averaging algorithm.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// # Example
///
/// ```no_run
/// // For example, to convert an image of type `PhotonImage` to grayscale:
/// use photon_rs::monochrome::grayscale;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// grayscale(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn grayscale(img: &mut PhotonImage) {
    // Use SIMD optimized version
    crate::simd::grayscale_simd(img);
}

/// Convert an image to grayscale with a human corrected factor, to account for human vision.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// # Example
///
/// ```no_run
/// // For example, to convert an image of type `PhotonImage` to grayscale with a human corrected factor:
/// use photon_rs::monochrome::grayscale_human_corrected;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// grayscale_human_corrected(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn grayscale_human_corrected(img: &mut PhotonImage) {
    // Use SIMD optimized version
    crate::simd::grayscale_human_corrected_simd(img);
}

/// Desaturate an image by getting the min/max of each pixel's RGB values.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// # Example
///
/// ```no_run
/// // For example, to desaturate an image:
/// use photon_rs::monochrome::desaturate;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// desaturate(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn desaturate(img: &mut PhotonImage) {
    crate::simd::desaturate_simd(img);
}

/// Uses a min. decomposition algorithm to convert an image to greyscale.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to decompose an image with min decomposition:
/// use photon_rs::monochrome::decompose_min;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// decompose_min(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn decompose_min(img: &mut PhotonImage) {
    let end = img.get_raw_pixels_slice().len();

    for i in (0..end).step_by(4) {
        let r_val = img.raw_pixels[i] as u32;
        let g_val = img.raw_pixels[i + 1] as u32;
        let b_val = img.raw_pixels[i + 2] as u32;

        // get the max and min vals of a pixel's 3 rgb values by sorting a vec of these
        let mut rgb_vals = [r_val, g_val, b_val];
        rgb_vals.sort_unstable();

        let gray: u8 = rgb_vals[0] as u8;

        img.raw_pixels[i] = gray;
        img.raw_pixels[i + 1] = gray;
        img.raw_pixels[i + 2] = gray;
    }
}

/// Uses a max. decomposition algorithm to convert an image to greyscale.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to decompose an image with max decomposition:
/// use photon_rs::monochrome::decompose_max;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// decompose_max(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn decompose_max(img: &mut PhotonImage) {
    let end = img.get_raw_pixels_slice().len();

    for i in (0..end).step_by(4) {
        let r_val = img.raw_pixels[i] as u32;
        let g_val = img.raw_pixels[i + 1] as u32;
        let b_val = img.raw_pixels[i + 2] as u32;

        // get the max and min vals of a pixel's 3 rgb values by sorting a vec of these
        let mut rgb_vals = [r_val, g_val, b_val];
        rgb_vals.sort_unstable();

        let gray: u8 = rgb_vals[2] as u8;

        img.raw_pixels[i] = gray;
        img.raw_pixels[i + 1] = gray;
        img.raw_pixels[i + 2] = gray;
    }
}

/// Employ only a limited number of gray shades in an image.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `num_shades` - The number of grayscale shades to be displayed in the image.
///
/// # Example
///
/// ```no_run
/// // For example, to limit an image to four shades of gray only:
/// use photon_rs::monochrome::grayscale_shades;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// grayscale_shades(&mut img, 4_u8);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn grayscale_shades(photon_image: &mut PhotonImage, num_shades: u8) {
    let conversion: f32 = 255.0 / (num_shades as f32 - 1.0);
    let pixels = photon_image.raw_pixels.as_mut_slice();
    let len = pixels.len();

    for i in (0..len).step_by(4) {
        let r_val = pixels[i] as u32;
        let g_val = pixels[i + 1] as u32;
        let b_val = pixels[i + 2] as u32;

        let avg: f32 = (r_val + g_val + b_val) as f32 / 3.0;
        let dividend = avg / conversion;
        let gray = ((dividend + 0.5) * conversion) as u8;

        pixels[i] = gray;
        pixels[i + 1] = gray;
        pixels[i + 2] = gray;
        // Alpha channel remains unchanged
    }
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to the Red channel's value.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// use photon_rs::monochrome::r_grayscale;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// r_grayscale(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn r_grayscale(photon_image: &mut PhotonImage) {
    single_channel_grayscale(photon_image, 0)
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to the Green channel's value.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// use photon_rs::monochrome::g_grayscale;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// g_grayscale(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn g_grayscale(photon_image: &mut PhotonImage) {
    single_channel_grayscale(photon_image, 1)
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to the Blue channel's value.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// use photon_rs::monochrome::b_grayscale;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// b_grayscale(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn b_grayscale(photon_image: &mut PhotonImage) {
    single_channel_grayscale(photon_image, 2)
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to a chosen channel's value.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `channel` - A usize representing the channel from 0 to 2. O represents the Red channel, 1 the Green channel, and 2 the Blue channel.
///
/// # Example
/// To grayscale using only values from the Red channel:
/// ```no_run
/// use photon_rs::monochrome::single_channel_grayscale;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// single_channel_grayscale(&mut img, 0_usize);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn single_channel_grayscale(photon_image: &mut PhotonImage, channel: usize) {
    let pixels = photon_image.raw_pixels.as_mut_slice();
    let len = pixels.len();

    for i in (0..len).step_by(4) {
        let channel_data = pixels[i + channel];

        pixels[i] = channel_data;
        pixels[i + 1] = channel_data;
        pixels[i + 2] = channel_data;
        // Alpha channel remains unchanged
    }
}

/// Threshold an image using a standard thresholding algorithm.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `threshold` - The amount the image should be thresholded by from 0 to 255.
/// # Example
///
/// ```no_run
/// // For example, to threshold an image of type `PhotonImage`:
/// use photon_rs::monochrome::threshold;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// threshold(&mut img, 30_u32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn threshold(img: &mut PhotonImage, threshold: u32) {
    // Use SIMD optimized version
    crate::simd::threshold_simd(img, threshold);
}
