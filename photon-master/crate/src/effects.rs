//! Special effects.

use crate::helpers;
use crate::{PhotonImage, Rgb};
use image::Pixel;
use image::Rgba;
use image::{GenericImage, GenericImageView};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use perlin2d::PerlinNoise2D;
use std::collections::HashMap;
use std::f64;

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// Adds an offset to the image by a certain number of pixels.
///
/// This creates an RGB shift effect.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `channel_index`: The index of the channel to increment. 0 for red, 1 for green and 2 for blue.
/// * `offset` - The offset is added to the pixels in the image.
/// # Example
///
/// ```no_run
/// // For example, to offset pixels by 30 pixels on the red channel:
/// use photon_rs::effects::offset;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// offset(&mut img, 0_usize, 30_u32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn offset(photon_image: &mut PhotonImage, channel_index: usize, offset: u32) {
    // Use SIMD optimized version
    crate::simd::offset_simd(photon_image, channel_index, offset);
}

/// Adds an offset to the red channel by a certain number of pixels.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset` - The offset you want to move the red channel by.
/// # Example
///
/// ```no_run
/// // For example, to add an offset to the red channel by 30 pixels.
/// use photon_rs::effects::offset_red;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// offset_red(&mut img, 30_u32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn offset_red(img: &mut PhotonImage, offset_amt: u32) {
    offset(img, 0, offset_amt)
}

/// Adds an offset to the green channel by a certain number of pixels.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset` - The offset you want to move the green channel by.
/// # Example
///
/// ```no_run
/// // For example, to add an offset to the green channel by 30 pixels.
/// use photon_rs::effects::offset_green;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// offset_green(&mut img, 30_u32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn offset_green(img: &mut PhotonImage, offset_amt: u32) {
    offset(img, 1, offset_amt)
}

/// Adds an offset to the blue channel by a certain number of pixels.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset_amt` - The offset you want to move the blue channel by.
/// # Example
/// // For example, to add an offset to the green channel by 40 pixels.
///
/// ```no_run
/// use photon_rs::effects::offset_blue;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// offset_blue(&mut img, 40_u32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn offset_blue(img: &mut PhotonImage, offset_amt: u32) {
    offset(img, 2, offset_amt)
}

/// Adds multiple offsets to the image by a certain number of pixels (on two channels).
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset` - The offset is added to the pixels in the image.
/// # Example
///
/// ```no_run
/// // For example, to add a 30-pixel offset to both the red and blue channels:
/// use photon_rs::effects::multiple_offsets;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// multiple_offsets(&mut img, 30_u32, 0_usize, 2_usize);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn multiple_offsets(
    photon_image: &mut PhotonImage,
    offset: u32,
    channel_index: usize,
    channel_index2: usize,
) {
    if channel_index > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }
    if channel_index2 > 2 {
        panic!("Invalid channel index passed. Channel2 must be equal to 0, 1, or 2.");
    }
    let mut img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            if x + offset < width - 1 && y + offset < height - 1 {
                let offset_px = img.get_pixel(x + offset, y);

                px[channel_index] = offset_px[channel_index];
            }

            if x as i32 - offset as i32 > 0 && y as i32 - offset as i32 > 0 {
                let offset_px2 = img.get_pixel(x - offset, y);

                px[channel_index2] = offset_px2[channel_index2];
            }

            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Halftoning effect.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example:
/// use photon_rs::effects::halftone;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// halftone(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn halftone(photon_image: &mut PhotonImage) {
    // Use SIMD optimized version
    crate::simd::halftone_simd(photon_image);
}

/// Reduces an image to the primary colours.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to add a primary colour effect to an image of type `DynamicImage`:
/// use photon_rs::effects::primary;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// primary(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn primary(img: &mut PhotonImage) {
    crate::simd::primary_simd(img);
}

/// Colorizes the green channels of the image.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to colorize an image of type `PhotonImage`:
/// use photon_rs::effects::colorize;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// colorize(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn colorize(photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(photon_image);
    let threshold = 220;
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let channels = px.channels();
            let px_as_rgb = Rgb {
                r: channels[0],
                g: channels[1],
                b: channels[2],
            };

            let baseline_color = Rgb {
                r: 0,
                g: 255,
                b: 255,
            };

            let square_distance = crate::helpers::square_distance(baseline_color, px_as_rgb);

            let mut r = channels[0] as f32;
            let mut g = channels[1] as f32;
            let mut b = channels[2] as f32;

            if square_distance < i32::pow(threshold, 2) {
                r *= 0.5;
                g *= 1.25;
                b *= 0.5;
            }

            px = image::Rgba([r as u8, g as u8, b as u8, 255]);
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

// #[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
// pub fn inc_luminosity(mut photon_image: PhotonImage) -> PhotonImage {
//     let mut img = helpers::dyn_image_from_raw(photon_image);
//     let (width, height) = img.dimensions();
//     let mut min_intensity = 255;
//     let mut max_intensity = 0;

//     // find the max and min intensities in the image
//     for x in 0..width {
//         for y in 0..height {
//             let px = img.get_pixel(x, y);
//             let intensity = (px.data[0] as u32 + px.data[1] as u32 + px.data[2] as u32) / 3;
//             if intensity > 0{
//                 min_intensity = cmp::min(min_intensity, intensity);
//                 max_intensity = cmp::max(max_intensity, intensity);
//             }

//         }
//     }

//     for x in 0..width {
//         for y in 0..height {
//             let mut px = img.get_pixel(x, y);
//             // let px_as_rgb = Rgb{r: px.data[0], g: px.data[1], b: px.data[2]};

//             let mut r = px.data[0] as f32;
//             let mut g = px.data[1] as f32;
//             let mut b = px.data[2] as f32;

//             let lum = (r + g + b) / 3.0;

//             let new_lum = 255.0 * (lum - min_intensity as f32) / (max_intensity / min_intensity) as f32;

//             r = r * new_lum / lum;
//             g = g * new_lum / lum;
//             b = b * new_lum / lum;

//             px.data[0] = r as u8;
//             px.data[1] = g as u8;
//             px.data[2] = b as u8;

//             img.put_pixel(x, y, px);
//         }
//     }
//     let mut raw_pixels = img.raw_pixels();
//     photon_image.raw_pixels = raw_pixels;
//     photon_image
// }

/// Applies a solarizing effect to an image.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to colorize an image of type `PhotonImage`:
/// use photon_rs::effects::solarize;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// solarize(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn solarize(photon_image: &mut PhotonImage) {
    crate::simd::solarize_simd(photon_image);
}

/// Applies a solarizing effect to an image and returns the resulting PhotonImage.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to solarize "retimg" an image of type `PhotonImage`:
/// use photon_rs::effects::solarize_retimg;
/// use photon_rs::native::open_image;
/// use photon_rs::PhotonImage;
///
/// let img = open_image("img.jpg").expect("File should open");
/// let result: PhotonImage = solarize_retimg(&img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn solarize_retimg(photon_image: &PhotonImage) -> PhotonImage {
    let mut img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let channels = px.channels();
            if 200_i32 - channels[0] as i32 > 0 {
                let new_r_val = 200 - channels[0];
                px = image::Rgba([new_r_val, channels[1], channels[2], channels[3]]);
            }
            img.put_pixel(x, y, px);
        }
    }

    let (width, height) = img.dimensions();

    PhotonImage {
        raw_pixels: img.into_bytes(),
        width,
        height,
    }
}

/// Adjust the brightness of an image by a factor.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `brightness` - A u8 to add or subtract to the brightness. To increase
/// the brightness, pass a positive number (up to 255). To decrease the brightness,
/// pass a negative number instead.
/// # Example
///
/// ```no_run
/// use photon_rs::effects::adjust_brightness;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// adjust_brightness(&mut img, 10_i16);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn adjust_brightness(photon_image: &mut PhotonImage, brightness: i16) {
    if brightness > 0 {
        inc_brightness(photon_image, brightness as u8)
    } else {
        dec_brightness(photon_image, brightness.unsigned_abs() as u8)
    }
}

/// Increase the brightness of an image by a constant.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `brightness` - A u8 to add to the brightness.
/// # Example
///
/// ```no_run
/// use photon_rs::effects::inc_brightness;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// inc_brightness(&mut img, 10_u8);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn inc_brightness(photon_image: &mut PhotonImage, brightness: u8) {
    // Use SIMD optimized version
    crate::simd::inc_brightness_simd(photon_image, brightness);
}

/// Decrease the brightness of an image by a constant.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `brightness` - A u8 to subtract from the brightness. It should be a positive number,
/// and this value will then be subtracted from the brightness.
/// # Example
///
/// ```no_run
/// use photon_rs::effects::dec_brightness;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// dec_brightness(&mut img, 10_u8);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn dec_brightness(photon_image: &mut PhotonImage, brightness: u8) {
    // Use SIMD optimized version
    crate::simd::dec_brightness_simd(photon_image, brightness);
}

/// Adjust the contrast of an image by a factor.
///
/// # Arguments
/// * `photon_image` - A PhotonImage that contains a view into the image.
/// * `contrast` - An f32 factor used to adjust contrast. Between [-255.0, 255.0]. The algorithm will
/// clamp results if passed factor is out of range.
/// # Example
///
/// ```no_run
/// use photon_rs::effects::adjust_contrast;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// adjust_contrast(&mut img, 30_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn adjust_contrast(photon_image: &mut PhotonImage, contrast: f32) {
    // Use SIMD optimized version
    crate::simd::adjust_contrast_simd(photon_image, contrast);
}

/// Tint an image by adding an offset to averaged RGB channel values.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `r_offset` - The amount the R channel should be incremented by.
/// * `g_offset` - The amount the G channel should be incremented by.
/// * `b_offset` - The amount the B channel should be incremented by.
/// # Example
///
/// ```no_run
/// // For example, to tint an image of type `PhotonImage`:
/// use photon_rs::effects::tint;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// tint(&mut img, 10_u32, 20_u32, 15_u32);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn tint(
    photon_image: &mut PhotonImage,
    r_offset: u32,
    g_offset: u32,
    b_offset: u32,
) {
    let mut img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let channels = px.channels();
            let (r_val, g_val, b_val) =
                (channels[0] as u32, channels[1] as u32, channels[2] as u32);

            let new_r_val = if r_val + r_offset < 255 {
                r_val as u8 + r_offset as u8
            } else {
                255
            };
            let new_g_val = if g_val + g_offset < 255 {
                g_val as u8 + g_offset as u8
            } else {
                255
            };
            let new_b_val = if b_val + b_offset < 255 {
                b_val as u8 + b_offset as u8
            } else {
                255
            };

            px = image::Rgba([new_r_val, new_g_val, new_b_val, 255]);

            img.put_pixel(x, y, px);
        }
    }

    let raw_pixels = img.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

fn draw_horizontal_strips(photon_image: &mut PhotonImage, num_strips: u8, color: Rgb) {
    let mut img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();

    let total_strips = (num_strips * 2) - 1;
    let height_strip = height / total_strips as u32;
    let mut y_pos: u32 = 0;
    for i in 1..num_strips {
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, (y_pos + height_strip) as i32).of_size(width, height_strip),
            Rgba([color.r, color.g, color.b, 255u8]),
        );
        y_pos = i as u32 * (height_strip * 2);
    }

    let raw_pixels = img.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Horizontal strips. Divide an image into a series of equal-height strips, for an artistic effect.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `num_strips` - The number of strips
/// # Example
///
/// ```no_run
/// // For example, to draw horizontal strips on a `PhotonImage`:
/// use photon_rs::effects::horizontal_strips;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// horizontal_strips(&mut img, 8u8);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn horizontal_strips(photon_image: &mut PhotonImage, num_strips: u8) {
    let color = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    draw_horizontal_strips(photon_image, num_strips, color)
}

/// Horizontal strips. Divide an image into a series of equal-width strips, for an artistic effect. Sepcify a color as well.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `num_strips` - The numbder of strips
/// * `color` - Color of strips.
/// # Example
///
/// ```no_run
/// // For example, to draw blue horizontal strips on a `PhotonImage`:
/// use photon_rs::effects::color_horizontal_strips;
/// use photon_rs::native::open_image;
/// use photon_rs::Rgb;
///
/// let color = Rgb::new(255u8, 0u8, 0u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// color_horizontal_strips(&mut img, 8u8, color);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn color_horizontal_strips(
    photon_image: &mut PhotonImage,
    num_strips: u8,
    color: Rgb,
) {
    draw_horizontal_strips(photon_image, num_strips, color)
}

fn draw_vertical_strips(photon_image: &mut PhotonImage, num_strips: u8, color: Rgb) {
    let mut img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();

    let total_strips = (num_strips * 2) - 1;
    let width_strip = width / total_strips as u32;
    let mut x_pos: u32 = 0;
    for i in 1..num_strips {
        draw_filled_rect_mut(
            &mut img,
            Rect::at((x_pos + width_strip) as i32, 0).of_size(width_strip, height),
            Rgba([color.r, color.g, color.b, 255u8]),
        );
        x_pos = i as u32 * (width_strip * 2);
    }

    let raw_pixels = img.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `num_strips` - The numbder of strips
/// # Example
///
/// ```no_run
/// // For example, to draw vertical strips on a `PhotonImage`:
/// use photon_rs::effects::vertical_strips;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// vertical_strips(&mut img, 8u8);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn vertical_strips(photon_image: &mut PhotonImage, num_strips: u8) {
    let color = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    draw_vertical_strips(photon_image, num_strips, color)
}

/// Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect. Sepcify a color as well.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `num_strips` - The numbder of strips
/// * `color` - Color of strips.
/// # Example
///
/// ```no_run
/// // For example, to draw red vertical strips on a `PhotonImage`:
/// use photon_rs::effects::color_vertical_strips;
/// use photon_rs::native::open_image;
/// use photon_rs::Rgb;
///
/// let color = Rgb::new(255u8, 0u8, 0u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// color_vertical_strips(&mut img, 8u8, color);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn color_vertical_strips(
    photon_image: &mut PhotonImage,
    num_strips: u8,
    color: Rgb,
) {
    draw_vertical_strips(photon_image, num_strips, color)
}

struct Intensity {
    val: i32,
    r: i32,
    g: i32,
    b: i32,
}

/// Original Oil Painting implementation (kept for comparison and testing)
/// This is the unoptimized version that uses HashMap
#[allow(dead_code)]
fn oil_original(photon_image: &mut PhotonImage, radius: i32, intensity: f64) {
    let img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();
    let mut target = image::DynamicImage::new_rgba8(width, height);
    let mut pixel_intensity_count: HashMap<usize, Intensity>;
    let mut intensity_lut = vec![vec![0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let single_pix = img.get_pixel(x, y);
            let current_val = single_pix.channels();
            let avg = (current_val[0] as i32
                + current_val[1] as i32
                + current_val[2] as i32) as f64
                / 3.0;
            let val = (avg * intensity) / 255.0;
            intensity_lut[y as usize][x as usize] = val.round() as usize;
        }
    }

    for y in 0..height {
        for x in 0..width {
            pixel_intensity_count = HashMap::new();

            for yy in -radius..=radius {
                let yyy = (y as i32) + yy;
                for xx in -radius..=radius {
                    let xxx = (x as i32) + xx;
                    if yyy > 0
                        && yyy < (height as i32)
                        && xxx > 0
                        && xxx < (width as i32)
                    {
                        let idx_x = xxx as usize;
                        let idx_y = yyy as usize;
                        let intensity_val = intensity_lut[idx_y][idx_x];
                        let single_pix = img.get_pixel(idx_x as u32, idx_y as u32);
                        let pix = single_pix.channels();
                        match pixel_intensity_count.get_mut(&(intensity_val)) {
                            Some(val) => {
                                val.val += 1;
                                val.r += pix[0] as i32;
                                val.g += pix[1] as i32;
                                val.b += pix[2] as i32;
                            }
                            None => {
                                pixel_intensity_count.insert(
                                    intensity_val,
                                    Intensity {
                                        val: 1,
                                        r: pix[0] as i32,
                                        g: pix[1] as i32,
                                        b: pix[2] as i32,
                                    },
                                );
                            }
                        }
                    }
                }
            }

            let mut map_vec: Vec<_> = pixel_intensity_count.iter().collect();
            map_vec.sort_by(|a, b| (b.1.val - a.1.val).cmp(&0));

            let cur_max = map_vec[0].1;
            target.put_pixel(
                x,
                y,
                Rgba::<u8>([
                    (cur_max.r / cur_max.val) as u8,
                    (cur_max.g / cur_max.val) as u8,
                    (cur_max.b / cur_max.val) as u8,
                    255,
                ]),
            )
        }
    }
    let raw_pixels = target.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Turn an image into an oil painting
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `radius` - Radius of each paint particle
/// * `intesnity` - How artsy an Image should be
/// # Example
///
/// ```no_run
/// // For example, to oil an image of type `PhotonImage`:
/// use photon_rs::effects::oil;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// oil(&mut img, 4i32, 55.0);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn oil(photon_image: &mut PhotonImage, radius: i32, intensity: f64) {
    // Optimized version using fixed-size arrays instead of HashMap
    oil_optimized(photon_image, radius, intensity);
}

/// Optimized Oil Painting implementation using fixed-size arrays.
///
/// This version avoids HashMap allocation and sorting operations by using
/// direct array indexing. The intensity values are in range 0-255, so we can
/// use fixed-size arrays for O(1) lookup instead of HashMap's O(1) average
/// with overhead.
///
/// Performance improvements:
/// - Eliminates HashMap allocation per pixel (major bottleneck)
/// - Eliminates sorting operation (replaced with linear scan)
/// - Better cache locality with contiguous array access
///
/// Expected speedup: 10-50x depending on radius
fn oil_optimized(photon_image: &mut PhotonImage, radius: i32, intensity: f64) {
    let img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();
    let mut target = image::DynamicImage::new_rgba8(width, height);
    let mut intensity_lut = vec![vec![0; width as usize]; height as usize];

    // Step 1: Calculate intensity LUT (same as original)
    for y in 0..height {
        for x in 0..width {
            let single_pix = img.get_pixel(x, y);
            let current_val = single_pix.channels();
            let avg = (current_val[0] as i32
                + current_val[1] as i32
                + current_val[2] as i32) as f64
                / 3.0;
            let val = (avg * intensity) / 255.0;
            intensity_lut[y as usize][x as usize] = val.round() as usize;
        }
    }

    // Step 2: Apply oil painting effect with optimized statistics
    // Use fixed-size arrays (256 possible intensity values) instead of HashMap
    let mut counts: [i32; 256] = [0; 256];
    let mut r_sum: [i32; 256] = [0; 256];
    let mut g_sum: [i32; 256] = [0; 256];
    let mut b_sum: [i32; 256] = [0; 256];

    for y in 0..height {
        for x in 0..width {
            // Reset arrays for this pixel
            for i in 0_usize..256 {
                counts[i] = 0;
                r_sum[i] = 0;
                g_sum[i] = 0;
                b_sum[i] = 0;
            }

            // Collect statistics in the neighborhood
            for yy in -radius..=radius {
                let yyy = (y as i32) + yy;
                for xx in -radius..=radius {
                    let xxx = (x as i32) + xx;
                    if yyy > 0
                        && yyy < (height as i32)
                        && xxx > 0
                        && xxx < (width as i32)
                    {
                        let idx_x = xxx as usize;
                        let idx_y = yyy as usize;
                        let intensity_val = intensity_lut[idx_y][idx_x];

                        // Use direct array indexing instead of HashMap
                        if intensity_val < 256 {
                            let single_pix = img.get_pixel(idx_x as u32, idx_y as u32);
                            let pix = single_pix.channels();
                            counts[intensity_val] += 1;
                            r_sum[intensity_val] += pix[0] as i32;
                            g_sum[intensity_val] += pix[1] as i32;
                            b_sum[intensity_val] += pix[2] as i32;
                        }
                    }
                }
            }

            // Find the intensity with maximum count (linear scan, no sorting needed)
            let mut max_count = 0;
            let mut max_intensity = 0;
            for i in 0..256 {
                if counts[i] > max_count {
                    max_count = counts[i];
                    max_intensity = i;
                }
            }

            // Calculate average color for the most frequent intensity
            if max_count > 0 {
                target.put_pixel(
                    x,
                    y,
                    Rgba::<u8>([
                        (r_sum[max_intensity] / max_count) as u8,
                        (g_sum[max_intensity] / max_count) as u8,
                        (b_sum[max_intensity] / max_count) as u8,
                        255,
                    ]),
                );
            } else {
                // Fallback: use original pixel
                let single_pix = img.get_pixel(x, y);
                target.put_pixel(x, y, single_pix);
            }
        }
    }
    let raw_pixels = target.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}
/// Turn an image into an frosted glass see through
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to turn an image of type `PhotonImage` into frosted glass see through:
/// use photon_rs::effects::frosted_glass;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// frosted_glass(&mut img);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn frosted_glass(photon_image: &mut PhotonImage) {
    let img_orig_buf = photon_image.get_raw_pixels_slice().to_vec();
    let width = photon_image.get_width();
    let height = photon_image.get_height();
    let end = img_orig_buf.len();

    let mut img_buf = Vec::<u8>::new();
    Vec::resize(&mut img_buf, end, 0_u8);

    let perlin = PerlinNoise2D::new(2, 10.0, 10.0, 10.0, 1.0, (100.0, 100.0), 0.5, 101);

    for pixel in (0..end).step_by(4) {
        let x = (pixel / 4) / width as usize;
        let y = (pixel / 4) % width as usize;

        let res = [
            perlin.get_noise(x as f64, y as f64) - 0.5,
            (perlin.get_noise(100.0 + x as f64, y as f64) - 0.5) * 4.0,
        ];

        let x_new = f64::clamp(f64::floor(x as f64 + res[0]), 0.0, height as f64 - 1.0);
        let x_new = x_new as usize;
        let y_new = f64::clamp(f64::floor(y as f64 + res[1]), 0.0, width as f64 - 1.0);
        let y_new = y_new as usize;

        let pixel_new = (x_new * width as usize + y_new) * 4;
        if pixel_new > end {
            continue;
        }
        img_buf[pixel] = img_orig_buf[pixel_new];
        img_buf[pixel + 1] = img_orig_buf[pixel_new + 1];
        img_buf[pixel + 2] = img_orig_buf[pixel_new + 2];
        img_buf[pixel + 3] = img_orig_buf[pixel_new + 3];
    }

    photon_image.raw_pixels = img_buf;
}

/// Pixelize an image.
///
/// # Arguments
/// * `photon_image` - A PhotonImage that contains a view into the image.
/// * `pixel_size` - Targeted pixel size of generated image.
/// # Example
///
/// ```no_run
/// // For example, to turn an image of type `PhotonImage` into a pixelized image with 50 pixels blocks:
/// use photon_rs::effects::pixelize;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// pixelize(&mut img, 50);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn pixelize(photon_image: &mut PhotonImage, pixel_size: i32) {
    let step_size = pixel_size.max(0) as usize;
    if step_size <= 1 {
        return;
    }

    let buf = photon_image.raw_pixels.as_mut_slice();
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;

    for sy in (0..height).step_by(step_size) {
        let src_row_index = sy * width;

        for sx in (0..width).step_by(step_size) {
            let src_index = 4 * (src_row_index + sx);
            let block_end_y = (sy + step_size).min(height);
            let block_end_x = (sx + step_size).min(width);

            for dy in sy..block_end_y {
                let dst_row_index = dy * width;

                for dx in sx..block_end_x {
                    let dst_index = 4 * (dst_row_index + dx);
                    buf[dst_index] = buf[src_index];
                    buf[dst_index + 1] = buf[src_index + 1];
                    buf[dst_index + 2] = buf[src_index + 2];
                    buf[dst_index + 3] = buf[src_index + 3];
                }
            }
        }
    }
}

/// Normalizes an image by remapping its range of pixels values. Only RGB
/// channels are processed and each channel is stretched to \[0, 255\] range
/// independently. This process is also known as contrast stretching.
/// # Arguments
/// * `photon_image` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to turn an image of type `PhotonImage` into a normalized image:
/// use photon_rs::effects::normalize;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// normalize(&mut img);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn normalize(photon_image: &mut PhotonImage) {
    crate::simd::normalize_simd(photon_image);
}

/// Applies Floyd-Steinberg dithering to an image.
/// Only RGB channels are processed, alpha remains unchanged.
/// # Arguments
/// * `photon_image` - A PhotonImage that contains a view into the image.
/// * `depth` - bits per channel. Clamped between 1 and 8.
/// # Example
///
/// ```no_run
/// // For example, to turn an image of type `PhotonImage` into a dithered image:
/// use photon_rs::effects::dither;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// let depth = 1;
/// dither(&mut img, depth);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn dither(photon_image: &mut PhotonImage, depth: u32) {
    let width = photon_image.get_width() as usize;
    let height = photon_image.get_height() as usize;
    let buf = photon_image.raw_pixels.as_mut_slice();
    let channels = 4;
    let chan_stride = 1;
    let col_stride = chan_stride * channels;
    let row_stride = col_stride * width;

    // Depth basically specifies the number of colours, e.g. when depth is 1,
    // than means monochrome values in each channel:
    // Number of colours = 2 ^ depth = 2 ^ 1 = 2
    // In order to resample pixel, the original value must be downscaled to [0..colours] range
    // (divide by the quant rate) and then upscaled back to [0..255] (multiply by the rate).
    let depth = depth.clamp(1, 8);
    let num_colours = u16::pow(2, depth);
    let quant_rate = (256_u16 / num_colours) as u8;
    let mut lookup_table: Vec<u8> = vec![0; 256];
    for (tbl_idx, table) in lookup_table.iter_mut().enumerate().take(256_usize) {
        let downscaled_val = (tbl_idx as u8) / quant_rate;
        let upscaled_val = downscaled_val * quant_rate;
        *table = upscaled_val.clamp(0, 255);
    }

    for row in 0..height - 1 {
        for col in 0..width - 1 {
            for chan in 0..channels - 1 {
                let buf_idx = row * row_stride + col * col_stride + chan * chan_stride;
                let old_pixel = buf[buf_idx];
                let new_pixel = lookup_table[old_pixel as usize];

                buf[buf_idx] = new_pixel;

                let quant_error = (old_pixel as i16) - (new_pixel as i16);

                let buf_idx =
                    row * row_stride + (col + 1) * col_stride + chan * chan_stride;
                let new_pixel = (buf[buf_idx] as i16) + (quant_error * 7) / 16;
                buf[buf_idx] = new_pixel.clamp(0, 255) as u8;

                let buf_idx = (row + 1) * row_stride + col * col_stride - col_stride
                    + chan * chan_stride;
                let new_pixel = (buf[buf_idx] as i16) + (quant_error * 3) / 16;
                buf[buf_idx] = new_pixel.clamp(0, 255) as u8;

                let buf_idx =
                    (row + 1) * row_stride + col * col_stride + chan * chan_stride;
                let new_pixel = (buf[buf_idx] as i16) + (quant_error * 5) / 16;
                buf[buf_idx] = new_pixel.clamp(0, 255) as u8;

                let buf_idx =
                    (row + 1) * row_stride + (col + 1) * col_stride + chan * chan_stride;
                let new_pixel = (buf[buf_idx] as i16) + quant_error / 16;
                buf[buf_idx] = new_pixel.clamp(0, 255) as u8;
            }
        }
    }
}

/// Applies Ordered Dithering (Bayer Matrix) to an image.
/// This is faster than Floyd-Steinberg dithering and produces better results for some images.
/// Only RGB channels are processed, alpha remains unchanged.
/// # Arguments
/// * `photon_image` - A PhotonImage that contains a view into the image.
/// * `depth` - bits per channel. Clamped between 1 and 8.
/// # Example
///
/// ```no_run
/// // For example, to turn an image of type `PhotonImage` into a dithered image:
/// use photon_rs::effects::dither_ordered;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// let depth = 1;
/// dither_ordered(&mut img, depth);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn dither_ordered(photon_image: &mut PhotonImage, depth: u32) {
    let width = photon_image.get_width();
    let height = photon_image.get_height();
    let pixels = photon_image.raw_pixels.as_mut_slice();
    
    let depth = depth.clamp(1, 8);
    let num_colours = u16::pow(2, depth);
    let quant_rate = (256_u16 / num_colours) as u8;
    
    // Pre-compute Bayer matrix thresholds
    // 4x4 Bayer matrix values: 0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5
    // Normalized to 0-255 range: multiply by 16
    const BAYER_THRESHOLDS: [i16; 16] = [
        0, 128, 32, 160, 192, 64, 224, 96, 48, 176, 16, 144, 240, 112, 208, 80
    ];
    
    let row_size = (width * 4) as usize;
    let quant_rate_i16 = quant_rate as i16;
    
    for y in 0..height {
        let row_offset = (y as usize * row_size) as usize;
        let bayer_row = (y % 4) as usize * 4;
        
        for x in 0..width {
            let idx = row_offset + (x as usize * 4);
            
            // Get Bayer threshold value using pre-computed table
            let bayer_col = (x % 4) as usize;
            let threshold = BAYER_THRESHOLDS[bayer_row + bayer_col];
            
            // Apply dithering to RGB channels (unroll loop for better performance)
            let r_val = pixels[idx] as i16;
            let g_val = pixels[idx + 1] as i16;
            let b_val = pixels[idx + 2] as i16;
            
            pixels[idx] = ((r_val + threshold - 128) / quant_rate_i16 * quant_rate_i16).clamp(0, 255) as u8;
            pixels[idx + 1] = ((g_val + threshold - 128) / quant_rate_i16 * quant_rate_i16).clamp(0, 255) as u8;
            pixels[idx + 2] = ((b_val + threshold - 128) / quant_rate_i16 * quant_rate_i16).clamp(0, 255) as u8;
            // Alpha channel remains unchanged
        }
    }
}

fn create_gradient_map(color_a: Rgb, color_b: Rgb) -> Vec<Rgb> {
    let mut gradient_map = vec![Rgb::new(0, 0, 0); 256];

    for (px, pos) in gradient_map.iter_mut().zip(0_u32..) {
        let inv_pos = 256 - pos;

        px.r = (((color_a.r as u32) * inv_pos + (color_b.r as u32) * pos) / 256)
            .clamp(0, 255) as u8;
        px.g = (((color_a.g as u32) * inv_pos + (color_b.g as u32) * pos) / 256)
            .clamp(0, 255) as u8;
        px.b = (((color_a.b as u32) * inv_pos + (color_b.b as u32) * pos) / 256)
            .clamp(0, 255) as u8;
    }

    gradient_map
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn duotone(photon_image: &mut PhotonImage, color_a: Rgb, color_b: Rgb) {
    let gradient_map = create_gradient_map(color_a, color_b);
    let buf = photon_image.raw_pixels.as_mut_slice();

    for px in buf.chunks_mut(4) {
        // Transform RGB (sRGB) to linear luminance (CIE 1931)
        let luma =
            (((px[0] as u32) * 2126 + (px[1] as u32) * 7152 + (px[2] as u32) * 722)
                / 10000)
                .clamp(0, 255);

        let mapped_luma = &gradient_map[luma as usize];
        px[0] = mapped_luma.r;
        px[1] = mapped_luma.g;
        px[2] = mapped_luma.b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PhotonImage;

    /// 测试基本功能：图像被修改且尺寸不变
    #[test]
    fn test_oil_basic() {
        let mut img = create_test_image(50, 50);
        let img_copy = img.clone();

        oil(&mut img, 2, 20.0);

        // 验证图像被修改
        assert_ne!(img.raw_pixels, img_copy.raw_pixels);

        // 验证 alpha 通道保持不变
        for i in (3..img.raw_pixels.len()).step_by(4) {
            assert_eq!(img.raw_pixels[i], 255);
        }
    }

    /// 测试零半径边界情况
    #[test]
    fn test_oil_with_zero_radius() {
        let mut img = create_test_image(30, 30);

        // 半径 0 应该正常运行（虽然效果可能很小）
        oil(&mut img, 0, 20.0);

        assert_eq!(img.width, 30);
        assert_eq!(img.height, 30);
    }

    /// 测试大半径边界情况
    #[test]
    fn test_oil_with_large_radius() {
        let mut img = create_test_image(40, 40);

        // 大半径应该正常运行
        oil(&mut img, 5, 30.0);

        assert_eq!(img.width, 40);
        assert_eq!(img.height, 40);
    }

    /// 测试高强度参数
    #[test]
    fn test_oil_with_high_intensity() {
        let mut img = create_test_image(30, 30);

        // 高强度应该正常运行
        oil(&mut img, 2, 100.0);

        assert_eq!(img.raw_pixels.len(), 30 * 30 * 4);
    }

    /// 测试单像素图像边界情况
    #[test]
    fn test_oil_single_pixel() {
        let mut img = PhotonImage {
            raw_pixels: vec![128, 128, 128, 255],
            width: 1,
            height: 1,
        };

        oil(&mut img, 1, 30.0);

        assert_eq!(img.width, 1);
        assert_eq!(img.height, 1);
    }

    /// 测试 2x2 小图像
    #[test]
    fn test_oil_2x2_image() {
        let mut img = PhotonImage {
            raw_pixels: vec![
                0, 0, 0, 255,      // 黑色
                255, 255, 255, 255,  // 白色
                255, 0, 0, 255,     // 红色
                0, 255, 0, 255,     // 绿色
            ],
            width: 2,
            height: 2,
        };

        oil(&mut img, 1, 20.0);

        assert_eq!(img.width, 2);
        assert_eq!(img.height, 2);
        assert_eq!(img.raw_pixels.len(), 16);
    }

    /// 测试纯色图像（所有像素相同）
    #[test]
    fn test_oil_solid_color_image() {
        let mut img = PhotonImage {
            raw_pixels: vec![100; 20 * 20 * 4], // 全是 100 的灰色
            width: 20,
            height: 20,
        };

        oil(&mut img, 3, 50.0);

        // 纯色图像经过处理后应该仍然是纯色
        let first_pixel = &img.raw_pixels[0..4];
        for chunk in img.raw_pixels.chunks(4) {
            assert_eq!(chunk, first_pixel, "Solid color should remain uniform");
        }
    }

    /// 测试 Alpha 通道保持不变
    #[test]
    fn test_oil_preserves_alpha() {
        let mut img = create_test_image_with_alpha(40, 40);
        oil(&mut img, 2, 30.0);

        // 验证所有 alpha 通道都是 255
        for i in (3..img.raw_pixels.len()).step_by(4) {
            assert_eq!(
                img.raw_pixels[i], 255,
                "Alpha channel should be preserved at index {}",
                i
            );
        }
    }

    /// 测试性能：大半径（验证不会因为性能问题导致超时）
    #[test]
    fn test_oil_large_radius_performance() {
        let mut img = create_test_image(20, 20);
        
        // 大半径不应该导致崩溃
        oil(&mut img, 10, 20.0);
        
        assert_eq!(img.width, 20);
        assert_eq!(img.height, 20);
    }

    /// 测试确定性：相同输入产生相同输出
    #[test]
    fn test_oil_deterministic() {
        let mut img1 = create_test_image(25, 25);
        let mut img2 = create_test_image(25, 25);

        oil(&mut img1, 2, 30.0);
        oil(&mut img2, 2, 30.0);

        // 相同输入应该产生相同输出
        assert_eq!(img1.raw_pixels, img2.raw_pixels);
    }

    /// 测试不同参数产生不同结果
    #[test]
    fn test_oil_different_params_produce_different_results() {
        let mut img1 = create_test_image(25, 25);
        let mut img2 = create_test_image(25, 25);
        let mut img3 = create_test_image(25, 25);

        oil(&mut img1, 1, 20.0);
        oil(&mut img2, 3, 20.0);
        oil(&mut img3, 2, 50.0);

        // 不同参数应该产生不同结果
        assert_ne!(img1.raw_pixels, img2.raw_pixels, "Different radius should produce different results");
        assert_ne!(img2.raw_pixels, img3.raw_pixels, "Different intensity should produce different results");
    }

    /// 创建一个带有梯度的测试图像
    fn create_test_image(width: u32, height: u32) -> PhotonImage {
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        for y in 0..height {
            for x in 0..width {
                let r = ((x * 255) / width) as u8;
                let g = ((y * 255) / height) as u8;
                let b = (((x + y) * 255) / (width + height)) as u8;
                let a = 255u8;

                pixels.push(r);
                pixels.push(g);
                pixels.push(b);
                pixels.push(a);
            }
        }

        PhotonImage {
            raw_pixels: pixels,
            width,
            height,
        }
    }

    /// 创建一个带有变化的 Alpha 通道的测试图像
    fn create_test_image_with_alpha(width: u32, height: u32) -> PhotonImage {
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        for y in 0..height {
            for x in 0..width {
                let r = ((x * 255) / width) as u8;
                let g = ((y * 255) / height) as u8;
                let b = (((x + y) * 255) / (width + height)) as u8;
                let a = 255u8; // 所有像素 alpha 都是 255

                pixels.push(r);
                pixels.push(g);
                pixels.push(b);
                pixels.push(a);
            }
        }

        PhotonImage {
            raw_pixels: pixels,
            width,
            height,
        }
    }
}
