//! Draw text onto an image.
//! For extended graphic design/text-drawing functionality, see [GDL](https://github.com/silvia-odwyer/gdl),
//! which is a graphic design/text-drawing library, compatible with Photon.

use crate::{helpers, PhotonImage};
use fontdue::{Font, FontSettings};
use image::{ImageBuffer, Luma, Rgba};
use imageproc::distance_transform::Norm;
use imageproc::morphology::dilate_mut;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// 默认字体名称常量
pub const DEFAULT_FONT_NAME: &str = "default";

/// 默认字体数据（Minikin-1.ttf）
const DEFAULT_FONT_DATA: &[u8] = include_bytes!("../fonts/Minikin-1.ttf");

/// 字体注册表
/// 存储字体名称到 Font 的映射
/// Fontdue 不需要 'static 生命周期，因此避免了 Box::leak 的内存泄漏问题
static FONT_REGISTRY: LazyLock<Mutex<HashMap<String, Font>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

/// 注册字体数据
///
/// # 参数
/// * `font_name` - 字体名称，用于后续引用
/// * `font_data` - 字体文件的二进制数据
pub fn register_font(font_name: &str, font_data: Vec<u8>) {
    // 检查字体是否已注册，避免重复加载
    {
        let registry = FONT_REGISTRY.lock().unwrap();
        if registry.contains_key(font_name) {
            return; // 字体已存在，直接返回
        }
    }

    // Fontdue 的 Font 是拥有自己数据的结构体，不需要 'static 生命周期
    // 这避免了 rusttype 中需要的 Box::leak 内存泄漏问题
    let font = Font::from_bytes(font_data, FontSettings::default())
        .expect("Invalid font data");

    let mut registry = FONT_REGISTRY.lock().unwrap();
    registry.insert(font_name.to_string(), font);
}

/// 检查字体是否已注册
pub fn is_font_registered(font_name: &str) -> bool {
    let registry = FONT_REGISTRY.lock().unwrap();
    registry.contains_key(font_name)
}

/// 获取已注册字体列表
pub fn get_registered_fonts() -> Vec<String> {
    let registry = FONT_REGISTRY.lock().unwrap();
    registry.keys().cloned().collect()
}

/// 移除已注册的字体
/// 注意：这只会从注册表中移除引用，无法释放之前通过 Box::leak 分配的内存
pub fn unregister_font(font_name: &str) -> bool {
    let mut registry = FONT_REGISTRY.lock().unwrap();
    registry.remove(font_name).is_some()
}

/// 根据字体名称加载字体
///
/// # 参数
/// * `font_name` - 字体名称
///
/// # 返回
/// 返回已注册的 Font 引用
fn load_font(font_name: &str) -> Font {
    let registry = FONT_REGISTRY.lock().unwrap();

    match registry.get(font_name) {
        Some(font) => font.clone(),
        None => {
            // 如果字体未注册，使用 panic 提示用户
            panic!(
                "Font '{}' not registered. Please register it first using register_font(). Available fonts: {:?}",
                font_name,
                registry.keys().collect::<Vec<_>>()
            )
        }
    }
}

/// 在 WASM 环境中注册字体
/// 
/// # 参数
/// * `font_name` - 字体名称，用于后续引用
/// * `font_data` - 字体文件的二进制数据（Uint8Array）
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn wasm_register_font(font_name: String, font_data: Vec<u8>) {
    register_font(&font_name, font_data);
}

/// 在 WASM 环境中检查字体是否已注册
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn wasm_is_font_registered(font_name: String) -> bool {
    is_font_registered(&font_name)
}

/// 在 WASM 环境中获取已注册字体列表
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn wasm_get_registered_fonts() -> Vec<String> {
    get_registered_fonts()
}

/// 在 WASM 环境中移除已注册的字体
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn wasm_unregister_font(font_name: String) -> bool {
    unregister_font(&font_name)
}

/// 在 WASM 环境中清空所有已注册的字体
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn wasm_clear_fonts() {
    let mut registry = FONT_REGISTRY.lock().unwrap();
    registry.clear();
}

/// 初始化默认字体（Minikin-1.ttf）
/// 在库初始化时调用此函数，将内置的Minikin-1.ttf字体注册为默认字体
pub fn init_default_font() {
    register_font(DEFAULT_FONT_NAME, DEFAULT_FONT_DATA.to_vec());
}

/// 获取默认字体名称
/// 
/// # 返回
/// 返回默认字体的名称字符串
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn get_default_font_name() -> String {
    DEFAULT_FONT_NAME.to_string()
}

/// 检查默认字体是否已初始化
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn is_default_font_initialized() -> bool {
    is_font_registered(DEFAULT_FONT_NAME)
}

/// Add bordered-text to an image.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `text` - Text string to be drawn to the image.
/// * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
/// * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
/// * `font_size` - Font size in pixels of the text to be drawn.
/// * `font_name` - Name of the registered font to use.
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
/// // Make sure to register a font first
/// photon_rs::text::register_font("my-font", font_data);
/// draw_text_with_border(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32, "my-font");
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_border(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    font_name: &str,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();
    let (img_width, img_height) = image.dimensions();

    // Create a grayscale image for border
    let mut border_image: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(img_width, img_height);

    let font = load_font(font_name);

    // Rasterize text for border (white on black)
    let mut cursor_x = x as f32;
    let cursor_y = y as f32;

    for c in text.chars() {
        let (metrics, bitmap) = font.rasterize(c, font_size);
        let bitmap_width = metrics.width;
        let bitmap_height = metrics.height;

        // Draw white pixels for border
        for py in 0..bitmap_height {
            for px in 0..bitmap_width {
                let alpha = bitmap[py * bitmap_width + px];
                if alpha > 0 {
                    let dest_x = (cursor_x + px as f32 + metrics.xmin as f32) as i32;
                    let dest_y = (cursor_y + py as f32 - metrics.ymin as f32) as i32;
                    if dest_x >= 0 && dest_x < img_width as i32 && dest_y >= 0 && dest_y < img_height as i32 {
                        border_image.put_pixel(dest_x as u32, dest_y as u32, Luma([255]));
                    }
                }
            }
        }

        cursor_x += metrics.advance_width;
    }

    // Dilate the border image
    let mut border_image_vec = border_image.clone();
    dilate_mut(&mut border_image_vec, Norm::LInf, 4u8);

    // Apply border to the main image
    for px in 0..img_width {
        for py in 0..img_height {
            let pixval = 255 - border_image_vec.get_pixel(px, py)[0];
            if pixval != 255 {
                let pixel = image.get_pixel(px, py);
                image.put_pixel(px, py, Rgba([pixval, pixval, pixval, pixel[3]]));
            }
        }
    }

    // Draw the main text
    cursor_x = x as f32;

    for c in text.chars() {
        let (metrics, bitmap) = font.rasterize(c, font_size);
        let bitmap_width = metrics.width;
        let bitmap_height = metrics.height;

        for py in 0..bitmap_height {
            for px in 0..bitmap_width {
                let alpha = bitmap[py * bitmap_width + px];
                if alpha > 0 {
                    let dest_x = (cursor_x + px as f32 + metrics.xmin as f32) as i32;
                    let dest_y = (cursor_y + py as f32 - metrics.ymin as f32) as i32;
                    if dest_x >= 0 && dest_x < img_width as i32 && dest_y >= 0 && dest_y < img_height as i32 {
                        let pixel = image.get_pixel(dest_x as u32, dest_y as u32);
                        // Blend white text with background
                        let blend = alpha as f32 / 255.0;
                        let r = (pixel[0] as f32 * (1.0 - blend) + 255.0 * blend) as u8;
                        let g = (pixel[1] as f32 * (1.0 - blend) + 255.0 * blend) as u8;
                        let b = (pixel[2] as f32 * (1.0 - blend) + 255.0 * blend) as u8;
                        image.put_pixel(dest_x as u32, dest_y as u32, Rgba([r, g, b, 255]));
                    }
                }
            }
        }

        cursor_x += metrics.advance_width;
    }

    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}

/// Add text to an image.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `text` - Text string to be drawn to the image.
/// * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
/// * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
/// * `font_size` - Font size in pixels of the text to be drawn.
/// * `font_name` - Name of the registered font to use.
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
/// // Make sure to register a font first
/// photon_rs::text::register_font("my-font", font_data);
/// draw_text(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32, "my-font");
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    font_name: &str,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();
    let (img_width, img_height) = image.dimensions();

    let font = load_font(font_name);

    // Rasterize text
    let mut cursor_x = x as f32;
    let cursor_y = y as f32;

    for c in text.chars() {
        let (metrics, bitmap) = font.rasterize(c, font_size);
        let bitmap_width = metrics.width;
        let bitmap_height = metrics.height;

        for py in 0..bitmap_height {
            for px in 0..bitmap_width {
                let alpha = bitmap[py * bitmap_width + px];
                if alpha > 0 {
                    let dest_x = (cursor_x + px as f32 + metrics.xmin as f32) as i32;
                    let dest_y = (cursor_y + py as f32 - metrics.ymin as f32) as i32;
                    if dest_x >= 0 && dest_x < img_width as i32 && dest_y >= 0 && dest_y < img_height as i32 {
                        let pixel = image.get_pixel(dest_x as u32, dest_y as u32);
                        // Blend white text with background
                        let blend = alpha as f32 / 255.0;
                        let r = (pixel[0] as f32 * (1.0 - blend) + 255.0 * blend) as u8;
                        let g = (pixel[1] as f32 * (1.0 - blend) + 255.0 * blend) as u8;
                        let b = (pixel[2] as f32 * (1.0 - blend) + 255.0 * blend) as u8;
                        image.put_pixel(dest_x as u32, dest_y as u32, Rgba([r, g, b, 255]));
                    }
                }
            }
        }

        cursor_x += metrics.advance_width;
    }

    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}

/// Add text to an image with custom color.
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
/// * `font_name` - Name of the registered font to use.
///
/// # Example
///
/// ```no_run
/// // For example to draw red text at 10, 10:
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text_with_color;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// // Make sure to register a font first
/// photon_rs::text::register_font("my-font", font_data);
/// draw_text_with_color(&mut img, "Hello!", 10_i32, 10_i32, 90_f32, 255u8, 0u8, 0u8, "my-font");
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
    font_name: &str,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();
    let (img_width, img_height) = image.dimensions();

    let font = load_font(font_name);

    // Rasterize text
    let mut cursor_x = x as f32;
    let cursor_y = y as f32;

    for c in text.chars() {
        let (metrics, bitmap) = font.rasterize(c, font_size);
        let bitmap_width = metrics.width;
        let bitmap_height = metrics.height;

        for py in 0..bitmap_height {
            for px in 0..bitmap_width {
                let alpha = bitmap[py * bitmap_width + px];
                if alpha > 0 {
                    let dest_x = (cursor_x + px as f32 + metrics.xmin as f32) as i32;
                    let dest_y = (cursor_y + py as f32 - metrics.ymin as f32) as i32;
                    if dest_x >= 0 && dest_x < img_width as i32 && dest_y >= 0 && dest_y < img_height as i32 {
                        let pixel = image.get_pixel(dest_x as u32, dest_y as u32);
                        // Blend custom color text with background
                        let blend = alpha as f32 / 255.0;
                        let new_r = (pixel[0] as f32 * (1.0 - blend) + r as f32 * blend) as u8;
                        let new_g = (pixel[1] as f32 * (1.0 - blend) + g as f32 * blend) as u8;
                        let new_b = (pixel[2] as f32 * (1.0 - blend) + b as f32 * blend) as u8;
                        image.put_pixel(dest_x as u32, dest_y as u32, Rgba([new_r, new_g, new_b, 255]));
                    }
                }
            }
        }

        cursor_x += metrics.advance_width;
    }

    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}

/// Add bordered-text to an image with custom color.
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
/// * `font_name` - Name of the registered font to use.
///
/// # Example
///
/// ```no_run
/// // For example to draw red text with border at 10, 10:
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text_with_border_and_color;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// // Make sure to register a font first
/// photon_rs::text::register_font("my-font", font_data);
/// draw_text_with_border_and_color(&mut img, "Hello!", 10_i32, 10_i32, 90_f32, 255u8, 0u8, 0u8, "my-font");
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
    font_name: &str,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();
    let (img_width, img_height) = image.dimensions();

    // Create a grayscale image for border
    let mut border_image: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(img_width, img_height);

    let font = load_font(font_name);

    // Rasterize text for border (white on black)
    let mut cursor_x = x as f32;
    let cursor_y = y as f32;

    for c in text.chars() {
        let (metrics, bitmap) = font.rasterize(c, font_size);
        let bitmap_width = metrics.width;
        let bitmap_height = metrics.height;

        // Draw white pixels for border
        for py in 0..bitmap_height {
            for px in 0..bitmap_width {
                let alpha = bitmap[py * bitmap_width + px];
                if alpha > 0 {
                    let dest_x = (cursor_x + px as f32 + metrics.xmin as f32) as i32;
                    let dest_y = (cursor_y + py as f32 - metrics.ymin as f32) as i32;
                    if dest_x >= 0 && dest_x < img_width as i32 && dest_y >= 0 && dest_y < img_height as i32 {
                        border_image.put_pixel(dest_x as u32, dest_y as u32, Luma([255]));
                    }
                }
            }
        }

        cursor_x += metrics.advance_width;
    }

    // Dilate the border image
    let mut border_image_vec = border_image.clone();
    dilate_mut(&mut border_image_vec, Norm::LInf, 4u8);

    // Apply black border to the main image
    for px in 0..img_width {
        for py in 0..img_height {
            let pixval = 255 - border_image_vec.get_pixel(px, py)[0];
            if pixval != 255 {
                let pixel = image.get_pixel(px, py);
                image.put_pixel(px, py, Rgba([0, 0, 0, pixel[3]]));
            }
        }
    }

    // Draw the main text with custom color
    cursor_x = x as f32;

    for c in text.chars() {
        let (metrics, bitmap) = font.rasterize(c, font_size);
        let bitmap_width = metrics.width;
        let bitmap_height = metrics.height;

        for py in 0..bitmap_height {
            for px in 0..bitmap_width {
                let alpha = bitmap[py * bitmap_width + px];
                if alpha > 0 {
                    let dest_x = (cursor_x + px as f32 + metrics.xmin as f32) as i32;
                    let dest_y = (cursor_y + py as f32 - metrics.ymin as f32) as i32;
                    if dest_x >= 0 && dest_x < img_width as i32 && dest_y >= 0 && dest_y < img_height as i32 {
                        let pixel = image.get_pixel(dest_x as u32, dest_y as u32);
                        // Blend custom color text with background
                        let blend = alpha as f32 / 255.0;
                        let new_r = (pixel[0] as f32 * (1.0 - blend) + r as f32 * blend) as u8;
                        let new_g = (pixel[1] as f32 * (1.0 - blend) + g as f32 * blend) as u8;
                        let new_b = (pixel[2] as f32 * (1.0 - blend) + b as f32 * blend) as u8;
                        image.put_pixel(dest_x as u32, dest_y as u32, Rgba([new_r, new_g, new_b, 255]));
                    }
                }
            }
        }

        cursor_x += metrics.advance_width;
    }

    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}