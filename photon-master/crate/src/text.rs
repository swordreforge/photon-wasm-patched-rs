//! Draw text onto an image.
//! For extended graphic design/text-drawing functionality, see [GDL](https://github.com/silvia-odwyer/gdl),
//! which is a graphic design library, compatible with Photon.

use crate::iter::ImageIterator;
use crate::{helpers, PhotonImage};
use image::{DynamicImage, Rgba};
use imageproc::distance_transform::Norm;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use rusttype::{Font, Scale};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// 字体类型枚举
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontType {
    /// Roboto 常规字体（默认）
    RobotoRegular = 0,
    /// 阿里普惠体 常规
    AlibabaRegular = 1,
    /// 鸿雷小纸条青春体
    HongLeiXiaoZhiTiao = 2,
}

/// 根据字体类型加载字体
/// 默认使用 Roboto-Regular.ttf
fn load_font(font_type: FontType) -> Font<'static> {
    let font_vec: Vec<u8> = match font_type {
        FontType::RobotoRegular => include_bytes!("../fonts/Roboto-Regular.ttf").to_vec(),
        FontType::AlibabaRegular => include_bytes!("../fonts/AlibabaPuHuiTi-3-55-Regular.ttf").to_vec(),
        FontType::HongLeiXiaoZhiTiao => include_bytes!("../fonts/鸿雷小纸条青春体.ttf").to_vec(),
    };
    // 使用 Box::leak 将数据泄漏到静态生命周期
    let font_static: &'static [u8] = Box::leak(font_vec.into_boxed_slice());
    Font::try_from_bytes(font_static).unwrap()
}

/// Add bordered-text to an image.
/// The only font available as of now is Roboto.
/// Note: A graphic design/text-drawing library is currently being developed, so stay tuned.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `text` - Text string to be drawn to the image.
/// * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
/// * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
/// * `font_size` - Font size in pixels of the text to be drawn.
///
/// # Example
///
/// ```no_run
/// // For example to draw the string "Welcome to Photon!" at 10, 10:
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text_with_border;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// draw_text_with_border(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_border(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
) {
    draw_text_with_border_with_font(photon_img, text, x, y, font_size, FontType::RobotoRegular);
}

/// Add bordered-text to an image with specified font type.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_border_with_font(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    font_type: FontType,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    let mut image2: DynamicImage =
        DynamicImage::new_luma8(image.width(), image.height());

    let font = load_font(font_type);
    let scale = Scale {
        x: font_size * 1.0,
        y: font_size,
    };
    draw_text_mut(
        &mut image2,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x,
        y,
        scale,
        &font,
        text,
    );

    let mut image2 = image2.to_luma8();
    dilate_mut(&mut image2, Norm::LInf, 4u8);

    // Add a border to the text.
    for (x, y) in ImageIterator::with_dimension(&image2.dimensions()) {
        let pixval = 255 - image2.get_pixel(x, y)[0];
        if pixval != 255 {
            let new_pix = Rgba([pixval, pixval, pixval, 255]);
            image.put_pixel(x, y, new_pix);
        }
    }

    draw_text_mut(
        &mut image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x + 10,
        y - 10,
        scale,
        &font,
        text,
    );
    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}

/// Add text to an image.
/// The only font available as of now is Roboto.
/// Note: A graphic design/text-drawing library is currently being developed, so stay tuned.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `text` - Text string to be drawn to the image.
/// * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
/// * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
/// * `font_size` - Font size in pixels of the text to be drawn.
///
/// # Example
///
/// ```no_run
/// // For example to draw the string "Welcome to Photon!" at 10, 10:
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// draw_text(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
) {
    draw_text_with_font(photon_img, text, x, y, font_size, FontType::RobotoRegular);
}

/// Add text to an image with specified font type.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_font(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    font_type: FontType,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    let font = load_font(font_type);
    let scale = Scale {
        x: font_size * 1.0,
        y: font_size,
    };

    draw_text_mut(
        &mut image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x,
        y,
        scale,
        &font,
        text,
    );
    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}

/// Add text to an image with custom color.
/// The only font available as of now is Roboto.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `text` - Text string to be drawn to the image.
/// * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
/// * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
/// * `font_size` - Font size in pixels of the text to be drawn.
/// * `r` - Red channel (0-255).
/// * `g` - Green channel (0-255).
/// * `b` - Blue channel (0-255).
///
/// # Example
///
/// ```no_run
/// // For example to draw red text at 10, 10:
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text_with_color;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// draw_text_with_color(&mut img, "Hello!", 10_i32, 10_i32, 90_f32, 255u8, 0u8, 0u8);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_color(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    r: u8,
    g: u8,
    b: u8,
) {
    draw_text_with_color_and_font(photon_img, text, x, y, font_size, r, g, b, FontType::RobotoRegular);
}

/// Add text to an image with custom color and specified font type.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_color_and_font(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    r: u8,
    g: u8,
    b: u8,
    font_type: FontType,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    let font = load_font(font_type);
    let scale = Scale {
        x: font_size * 1.0,
        y: font_size,
    };

    draw_text_mut(
        &mut image,
        Rgba([r, g, b, 255u8]),
        x,
        y,
        scale,
        &font,
        text,
    );
    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}

/// Add bordered-text to an image with custom color.
/// The only font available as of now is Roboto.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `text` - Text string to be drawn to the image.
/// * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
/// * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
/// * `font_size` - Font size in pixels of the text to be drawn.
/// * `r` - Red channel (0-255).
/// * `g` - Green channel (0-255).
/// * `b` - Blue channel (0-255).
///
/// # Example
///
/// ```no_run
/// // For example to draw red text with border at 10, 10:
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text_with_border_and_color;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// draw_text_with_border_and_color(&mut img, "Hello!", 10_i32, 10_i32, 90_f32, 255u8, 0u8, 0u8);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_border_and_color(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    r: u8,
    g: u8,
    b: u8,
) {
    draw_text_with_border_and_color_and_font(photon_img, text, x, y, font_size, r, g, b, FontType::RobotoRegular);
}

/// Add bordered-text to an image with custom color and specified font type.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_border_and_color_and_font(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    r: u8,
    g: u8,
    b: u8,
    font_type: FontType,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();
    let mut image2: DynamicImage = DynamicImage::new_luma8(image.width(), image.height());

    let font = load_font(font_type);
    let scale = Scale {
        x: font_size * 1.0,
        y: font_size,
    };

    // 绘制边框
    draw_text_mut(
        &mut image2,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x,
        y,
        scale,
        &font,
        text,
    );

    let mut image2 = image2.to_luma8();
    dilate_mut(&mut image2, Norm::LInf, 4u8);

    // 添加黑色边框
    for (px, py) in ImageIterator::with_dimension(&image2.dimensions()) {
        let pixval = 255 - image2.get_pixel(px, py)[0];
        if pixval != 255 {
            let new_pix = Rgba([0u8, 0u8, 0u8, 255u8]);
            image.put_pixel(px, py, new_pix);
        }
    }

    // 绘制自定义颜色的文本
    draw_text_mut(
        &mut image,
        Rgba([r, g, b, 255u8]),
        x + 10,
        y - 10,
        scale,
        &font,
        text,
    );
    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}
