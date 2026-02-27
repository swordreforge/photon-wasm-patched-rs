#![feature(portable_simd)]

//! A high-performance image processing library, available for use both natively and on the web.
//!
//! #### Functions
//! 96 functions are available, including:
//! - **Transformations**: Resize, crop, and flip images.
//! - **Image correction**: Hue rotation, sharpening, brightness adjustment, adjusting saturation, lightening/darkening all within various colour spaces.
//! - **Convolutions**: Sobel filters, blurs, Laplace effects, edge detection, etc.,
//! - **Channel manipulation**: Increasing/decreasing RGB channel values, swapping channels, removing channels, etc.
//! - **Monochrome effects**: Duotoning, greyscaling of various forms, thresholding, sepia, averaging RGB values
//! - **Colour manipulation**: Work with the image in various colour spaces such as HSL, LCh, and sRGB, and adjust the colours accordingly.
//! - **Filters**: Over 30 pre-set filters available, incorporating various effects and transformations.
//! - **Text**: Apply text to imagery in artistic ways, or to watermark, etc.,
//! - **Watermarking**: Watermark images in multiple formats.
//! - **Blending**: Blend images together using 10 different techniques, change image backgrounds.
//!
//! ## Example
//! ```no_run
//! extern crate photon_rs;
//!
//! use photon_rs::channels::alter_red_channel;
//! use photon_rs::native::{open_image};
//!
//! fn main() {
//!     // Open the image (a PhotonImage is returned)
//!     let mut img = open_image("img.jpg").expect("File should open");
//!     // Apply a filter to the pixels
//!     alter_red_channel(&mut img, 25_i16);
//! }
//! ```
//!
//! This crate contains built-in preset functions, which provide default image processing functionality, as well as functions
//! that allow for direct, low-level access to channel manipulation.
//! To view a full demo of filtered imagery, visit the [official website](https://silvia-odwyer.github.io/photon).
//!
//! ### WebAssembly Use
//! To allow for universal communication between the core Rust library and WebAssembly, the functions have been generalised to allow for both native and in-browser use.
//! [Check out the official guide](https://silvia-odwyer.github.io/photon/guide/) on how to get started with Photon on the web.
//!
//! ### Live Demo
//! View the [official demo of WASM in action](https://silvia-odwyer.github.io/photon).

use base64::{Engine as _, engine::general_purpose};
use image::DynamicImage::ImageRgba8;
use image::GenericImage;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::OnceLock;

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// 初始化默认字体，使用OnceLock确保只初始化一次
static DEFAULT_FONT_INIT: OnceLock<()> = OnceLock::new();

/// 初始化默认字体（在库加载时自动调用）
fn ensure_default_font_initialized() {
    DEFAULT_FONT_INIT.get_or_init(|| {
        text::init_default_font();
    });
}

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::Clamped;

#[cfg(feature = "web-sys")]
use web_sys::{
    Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData,
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Provides the image's height, width, and contains the image's raw pixels.
/// For use when communicating between JS and WASM, and also natively.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[repr(C, align(16))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhotonImage {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32,
}

impl PhotonImage {
    /// Get the PhotonImage's pixels as a slice of u8s.
    pub fn get_raw_pixels_slice(&self) -> &[u8] {
        &self.raw_pixels
    }
}

impl PhotonImage {
    /// Helper function to ensure pixel data is aligned for SIMD operations.
    /// Adjusts the capacity to be a multiple of 16 for better SIMD performance.
    #[inline]
    fn ensure_simd_aligned(pixels: &mut Vec<u8>) {
        let len = pixels.len();
        let padding = (16 - (len % 16)) % 16;
        if padding > 0 {
            pixels.reserve_exact(padding);
        }
    }
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
impl PhotonImage {
    /// Initialize the thread pool for WebAssembly parallel processing.
    /// 
    /// This function must be called from JavaScript before using any parallel processing features.
    /// It sets up the worker threads for rayon parallel execution.
    /// 
    /// # JavaScript Example
    /// ```javascript
    /// import { initThreadPool } from './photon_wasm.js';
    /// 
    /// // Initialize with 4 threads
    /// await initThreadPool(4);
    /// 
    /// // Now you can use parallel processing features
    /// ```
    /// 
    /// # Arguments
    /// * `num_threads` - Number of threads to use for parallel processing.
    ///   If 0, it will use the hardware concurrency (number of logical CPUs).
    #[cfg(all(feature = "enable_wasm", feature = "wasm-bindgen-rayon"))]
    #[wasm_bindgen]
    pub async fn init_thread_pool(num_threads: usize) -> Result<(), JsValue> {
        use wasm_bindgen_rayon::init_thread_pool;
        use wasm_bindgen_futures::JsFuture;
        
        // wasm-bindgen-rayon 的 init_thread_pool 接受一个 usize 参数
        // 0 表示使用默认线程数
        let threads = if num_threads == 0 { 4 } else { num_threads };
        let promise = init_thread_pool(threads);
        
        JsFuture::from(promise).await.map(|_| ()).map_err(|e| e.unchecked_into())
    }

    #[cfg_attr(feature = "enable_wasm", wasm_bindgen(constructor))]
    /// Create a new PhotonImage from a Vec of u8s, which represent raw pixels.
    pub fn new(mut raw_pixels: Vec<u8>, width: u32, height: u32) -> PhotonImage {
        ensure_default_font_initialized();
        Self::ensure_simd_aligned(&mut raw_pixels);
        PhotonImage {
            raw_pixels,
            width,
            height,
        }
    }

    /// Create a new PhotonImage from a base64 string.
    pub fn new_from_base64(base64: &str) -> PhotonImage {
        base64_to_image(base64)
    }

    /// Create a new PhotonImage from a byteslice.
    pub fn new_from_byteslice(vec: Vec<u8>) -> PhotonImage {
        let slice = vec.as_slice();

        let img = image::load_from_memory(slice).unwrap();

        let mut raw_pixels = img.to_rgba8().to_vec();
        Self::ensure_simd_aligned(&mut raw_pixels);

        PhotonImage {
            raw_pixels,
            width: img.width(),
            height: img.height(),
        }
    }

    /// Create a new PhotonImage from a Blob/File.
    #[cfg(feature = "web-sys")]
    pub fn new_from_blob(blob: Blob) -> PhotonImage {
        let bytes: js_sys::Uint8Array = js_sys::Uint8Array::new(&blob);

        let vec = bytes.to_vec();

        PhotonImage::new_from_byteslice(vec)
    }

    /// Create a new PhotonImage from a HTMLImageElement
    #[cfg(feature = "web-sys")]
    pub fn new_from_image(image: HtmlImageElement) -> PhotonImage {
        set_panic_hook();

        let document = web_sys::window().unwrap().document().unwrap();

        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(image.width());
        canvas.set_height(image.height());

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context
            .draw_image_with_html_image_element(&image, 0.0, 0.0)
            .unwrap();

        open_image(canvas, context)
    }

    // pub fn new_from_buffer(buffer: &Buffer, width: u32, height: u32) -> PhotonImage {
    //     // Convert a Node.js Buffer into a Vec<u8>
    //     let raw_pixels: Vec<u8> = Uint8Array::new_with_byte_offset_and_length(
    //         &buffer.buffer(),
    //         buffer.byte_offset(),
    //         buffer.length(),
    //     ).to_vec();

    //     PhotonImage {
    //         raw_pixels,
    //         width,
    //         height,
    //     }
    // }

    /// Get the width of the PhotonImage.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the PhotonImage's pixels as a Vec of u8s.
    /// 
    /// **Note**: This clones the pixel data, which can be expensive for large images.
    /// For read-only access, prefer `get_raw_pixels_slice()` which returns a reference without cloning.
    pub fn get_raw_pixels(&self) -> Vec<u8> {
        self.raw_pixels.clone()
    }

    /// Get the height of the PhotonImage.
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Convert the PhotonImage to base64.
    pub fn get_base64(&self) -> String {
        let mut img = helpers::dyn_image_from_raw(self);
        img = ImageRgba8(img.to_rgba8());

        let mut buffer = vec![];
        img.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png)
            .unwrap();
        let base64 = general_purpose::STANDARD.encode(&buffer);

        let res_base64 = format!("data:image/png;base64,{}", base64);

        res_base64
    }

    /// Convert the PhotonImage to raw bytes. Returns PNG.
    pub fn get_bytes(&self) -> Vec<u8> {
        let mut img = helpers::dyn_image_from_raw(self);
        img = ImageRgba8(img.to_rgba8());
        let mut buffer = vec![];
        img.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png)
            .unwrap();
        buffer
    }

    /// Convert the PhotonImage to raw bytes. Returns a JPEG.
    pub fn get_bytes_jpeg(&self, quality: u8) -> Vec<u8> {
        let mut img = helpers::dyn_image_from_raw(self);
        img = ImageRgba8(img.to_rgba8());
        let mut buffer = vec![];
        let out_format = image::ImageOutputFormat::Jpeg(quality);
        img.write_to(&mut Cursor::new(&mut buffer), out_format)
            .unwrap();
        buffer
    }

    /// Convert the PhotonImage to raw bytes. Returns a WEBP.
    pub fn get_bytes_webp(&self) -> Vec<u8> {
        self.get_bytes_webp_with_quality(75) // 默认质量 75
    }

    /// Convert the PhotonImage to raw bytes. Returns a WEBP with specified quality.
    /// # Arguments
    /// * `quality` - WebP quality (0-100). Higher means better quality but larger file.
    ///   - 0-50: Low quality, small file size
    ///   - 51-75: Medium quality (recommended for web)
    ///   - 76-100: High quality, larger file size
    /// 
    /// Note: The image 0.24.x crate's WebPEncoder currently only supports lossless encoding.
    /// The quality parameter is reserved for future use when the crate adds lossy encoding support.
    /// Currently, all images are encoded in lossless mode regardless of the quality value.
    /// For quality control, consider using JPEG format with `get_bytes_jpeg()` instead.
    pub fn get_bytes_webp_with_quality(&self, _quality: u8) -> Vec<u8> {
        let mut img = helpers::dyn_image_from_raw(self);
        img = ImageRgba8(img.to_rgba8());
        let mut buffer = vec![];
        
        // image 0.24.x 的 WebP 编码器目前只支持无损编码
        // 编码器需要一个 writer 参数
        let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut buffer);
        
        // 编码图像数据
        encoder.encode(img.as_bytes(), img.width(), img.height(), image::ColorType::Rgba8)
            .expect("Failed to encode WebP");
        buffer
    }

    /// Convert the PhotonImage's raw pixels to JS-compatible ImageData.
    #[cfg(all(feature = "web-sys", feature = "wasm-bindgen"))]
    #[allow(clippy::unnecessary_mut_passed)]
    pub fn get_image_data(&mut self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut self.raw_pixels),
            self.width,
            self.height,
        )
        .unwrap()
    }

    /// Convert ImageData to raw pixels, and update the PhotonImage's raw pixels to this.
    #[cfg(feature = "web-sys")]
    pub fn set_imgdata(&mut self, img_data: ImageData) {
        let width = img_data.width();
        let height = img_data.height();
        let mut raw_pixels = to_raw_pixels(img_data);
        Self::ensure_simd_aligned(&mut raw_pixels);
        self.width = width;
        self.height = height;
        self.raw_pixels = raw_pixels;
    }

    /// Calculates estimated filesize and returns number of bytes
    pub fn get_estimated_filesize(&self) -> u64 {
        let base64_data = self.get_base64();
        let padding_count = if base64_data.ends_with("==") {
            2
        } else if base64_data.ends_with('=') {
            1
        } else {
            0
        };

        // Size of original string(in bytes) = ceil(6n/8) – padding
        ((base64_data.len() as f64) * 0.75).ceil() as u64 - padding_count
    }

    /// Get the color of a pixel at the specified coordinates.
    ///
    /// Returns the RGBA color values as a Color struct.
    /// Returns None if the coordinates are out of bounds.
    ///
    /// # Arguments
    /// * `x` - X coordinate (0 to width-1)
    /// * `y` - Y coordinate (0 to height-1)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use photon_rs::PhotonImage;
    ///
    /// let img = PhotonImage::new(vec![255, 0, 0, 255], 1, 1);
    /// if let Some(color) = img.get_pixel_color(0, 0) {
    ///     println!("Pixel color: R={}, G={}, B={}, A={}", color.r, color.g, color.b, color.a);
    /// }
    /// ```
    pub fn get_pixel_color(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = ((y * self.width + x) * 4) as usize;
        let r = self.raw_pixels[idx];
        let g = self.raw_pixels[idx + 1];
        let b = self.raw_pixels[idx + 2];
        let a = self.raw_pixels[idx + 3];

        Some(Color::new(r, g, b, a))
    }

    /// Get the color of a pixel at the specified coordinates as a hex string.
    ///
    /// Returns the color in hex format (#RRGGBB or #RRGGBBAA).
    /// Returns None if the coordinates are out of bounds.
    ///
    /// # Arguments
    /// * `x` - X coordinate (0 to width-1)
    /// * `y` - Y coordinate (0 to height-1)
    /// * `include_alpha` - Whether to include alpha channel in the hex string
    ///
    /// # Example
    ///
    /// ```no_run
    /// use photon_rs::PhotonImage;
    ///
    /// let img = PhotonImage::new(vec![255, 0, 0, 255], 1, 1);
    /// if let Some(hex) = img.get_pixel_color_hex(0, 0, false) {
    ///     println!("Pixel color: {}", hex); // Output: #ff0000
    /// }
    /// ```
    pub fn get_pixel_color_hex(&self, x: u32, y: u32, include_alpha: bool) -> Option<String> {
        if let Some(color) = self.get_pixel_color(x, y) {
            Some(color.to_hex(include_alpha))
        } else {
            None
        }
    }

    /// Get the average color of a rectangular region.
    ///
    /// Returns the average RGBA color values as a Color struct.
    /// Returns None if the region is out of bounds.
    ///
    /// # Arguments
    /// * `x` - X coordinate of the top-left corner
    /// * `y` - Y coordinate of the top-left corner
    /// * `width` - Width of the region
    /// * `height` - Height of the region
    ///
    /// # Example
    ///
    /// ```no_run
    /// use photon_rs::PhotonImage;
    ///
    /// let img = PhotonImage::new(vec![255, 0, 0, 255, 0, 255, 0, 255], 2, 1);
    /// if let Some(color) = img.get_region_average_color(0, 0, 2, 1) {
    ///     println!("Average color: R={}, G={}, B={}, A={}", color.r, color.g, color.b, color.a);
    /// }
    /// ```
    pub fn get_region_average_color(&self, x: u32, y: u32, width: u32, height: u32) -> Option<Color> {
        // Validate bounds
        if x >= self.width || y >= self.height || width == 0 || height == 0 {
            return None;
        }

        let end_x = (x + width).min(self.width);
        let end_y = (y + height).min(self.height);
        let actual_width = end_x - x;
        let actual_height = end_y - y;

        if actual_width == 0 || actual_height == 0 {
            return None;
        }

        let mut sum_r: u64 = 0;
        let mut sum_g: u64 = 0;
        let mut sum_b: u64 = 0;
        let mut sum_a: u64 = 0;
        let mut count: u64 = 0;

        for py in y..end_y {
            for px in x..end_x {
                let idx = ((py * self.width + px) * 4) as usize;
                sum_r += self.raw_pixels[idx] as u64;
                sum_g += self.raw_pixels[idx + 1] as u64;
                sum_b += self.raw_pixels[idx + 2] as u64;
                sum_a += self.raw_pixels[idx + 3] as u64;
                count += 1;
            }
        }

        Some(Color::new(
            (sum_r / count) as u8,
            (sum_g / count) as u8,
            (sum_b / count) as u8,
            (sum_a / count) as u8,
        ))
    }

    /// Get the dominant color of the entire image.
    ///
    /// Uses a color quantization approach to find the most frequent color.
    /// Returns the RGBA color values as a Color struct.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use photon_rs::PhotonImage;
    ///
    /// let img = PhotonImage::new(vec![255, 0, 0, 255], 1, 1);
    /// let color = img.get_dominant_color();
    /// println!("Dominant color: R={}, G={}, B={}, A={}", color.r, color.g, color.b, color.a);
    /// ```
    pub fn get_dominant_color(&self) -> Color {
        use std::collections::HashMap;

        // Sample pixels for performance (max 10000 pixels)
        let total_pixels = self.width * self.height;
        let sample_step = if total_pixels > 10000 {
            (total_pixels / 10000).max(1) as usize
        } else {
            1
        };

        let mut color_counts: HashMap<(u8, u8, u8, u8), u64> = HashMap::new();

        for i in (0..self.raw_pixels.len()).step_by(4 * sample_step) {
            let r = self.raw_pixels[i];
            let g = self.raw_pixels[i + 1];
            let b = self.raw_pixels[i + 2];
            let a = self.raw_pixels[i + 3];

            *color_counts.entry((r, g, b, a)).or_insert(0) += 1;
        }

        // Find the color with the highest count
        let (r, g, b, a) = color_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(color, _)| color)
            .unwrap_or((0, 0, 0, 255));

        Color::new(r, g, b, a)
    }

    /// Get the dominant color of a rectangular region.
    ///
    /// Uses a color quantization approach to find the most frequent color in the region.
    /// Returns None if the region is out of bounds.
    ///
    /// # Arguments
    /// * `x` - X coordinate of the top-left corner
    /// * `y` - Y coordinate of the top-left corner
    /// * `width` - Width of the region
    /// * `height` - Height of the region
    ///
    /// # Example
    ///
    /// ```no_run
    /// use photon_rs::PhotonImage;
    ///
    /// let img = PhotonImage::new(vec![255, 0, 0, 255, 0, 255, 0, 255], 2, 1);
    /// if let Some(color) = img.get_region_dominant_color(0, 0, 2, 1) {
    ///     println!("Dominant color: R={}, G={}, B={}, A={}", color.r, color.g, color.b, color.a);
    /// }
    /// ```
    pub fn get_region_dominant_color(&self, x: u32, y: u32, width: u32, height: u32) -> Option<Color> {
        use std::collections::HashMap;

        // Validate bounds
        if x >= self.width || y >= self.height || width == 0 || height == 0 {
            return None;
        }

        let end_x = (x + width).min(self.width);
        let end_y = (y + height).min(self.height);
        let actual_width = end_x - x;
        let actual_height = end_y - y;

        if actual_width == 0 || actual_height == 0 {
            return None;
        }

        let mut color_counts: HashMap<(u8, u8, u8, u8), u64> = HashMap::new();

        for py in y..end_y {
            for px in x..end_x {
                let idx = ((py * self.width + px) * 4) as usize;
                let r = self.raw_pixels[idx];
                let g = self.raw_pixels[idx + 1];
                let b = self.raw_pixels[idx + 2];
                let a = self.raw_pixels[idx + 3];

                *color_counts.entry((r, g, b, a)).or_insert(0) += 1;
            }
        }

        // Find the color with the highest count
        let (r, g, b, a) = color_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(color, _)| color)?;

        Some(Color::new(r, g, b, a))
    }

    /// Get the color palette of the image.
    ///
    /// Returns a list of the most frequent colors in the image.
    ///
    /// # Arguments
    /// * `num_colors` - Number of colors to extract (default 5)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use photon_rs::PhotonImage;
    ///
    /// let img = PhotonImage::new(vec![255, 0, 0, 255, 0, 255, 0, 255], 2, 1);
    /// let palette = img.get_color_palette(5);
    /// for (i, color) in palette.iter().enumerate() {
    ///     println!("Color {}: #{:02x}{:02x}{:02x}", i + 1, color.r, color.g, color.b);
    /// }
    /// ```
    pub fn get_color_palette(&self, num_colors: usize) -> Vec<Color> {
        use std::collections::HashMap;

        // Sample pixels for performance
        let total_pixels = self.width * self.height;
        let sample_step = if total_pixels > 10000 {
            (total_pixels / 10000).max(1) as usize
        } else {
            1
        };

        let mut color_counts: HashMap<(u8, u8, u8, u8), u64> = HashMap::new();

        for i in (0..self.raw_pixels.len()).step_by(4 * sample_step) {
            let r = self.raw_pixels[i];
            let g = self.raw_pixels[i + 1];
            let b = self.raw_pixels[i + 2];
            let a = self.raw_pixels[i + 3];

            *color_counts.entry((r, g, b, a)).or_insert(0) += 1;
        }

        // Sort by count and take top N
        let mut colors: Vec<_> = color_counts.into_iter().collect();
        colors.sort_by(|a, b| b.1.cmp(&a.1));
        colors.truncate(num_colors);

        colors.into_iter().map(|((r, g, b, a), _)| Color::new(r, g, b, a)).collect()
    }

    /// Get the brightness of a pixel at the specified coordinates.
    ///
    /// Returns the brightness value (0-255) using the human-corrected formula.
    /// Returns None if the coordinates are out of bounds.
    ///
    /// # Arguments
    /// * `x` - X coordinate (0 to width-1)
    /// * `y` - Y coordinate (0 to height-1)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use photon_rs::PhotonImage;
    ///
    /// let img = PhotonImage::new(vec![255, 0, 0, 255], 1, 1);
    /// if let Some(brightness) = img.get_pixel_brightness(0, 0) {
    ///     println!("Pixel brightness: {}", brightness);
    /// }
    /// ```
    pub fn get_pixel_brightness(&self, x: u32, y: u32) -> Option<u8> {
        if let Some(color) = self.get_pixel_color(x, y) {
            Some(color.brightness())
        } else {
            None
        }
    }

    /// Get the average brightness of a rectangular region.
    ///
    /// Returns the average brightness value (0-255).
    /// Returns None if the region is out of bounds.
    ///
    /// # Arguments
    /// * `x` - X coordinate of the top-left corner
    /// * `y` - Y coordinate of the top-left corner
    /// * `width` - Width of the region
    /// * `height` - Height of the region
    ///
    /// # Example
    ///
    /// ```no_run
    /// use photon_rs::PhotonImage;
    ///
    /// let img = PhotonImage::new(vec![255, 0, 0, 255, 0, 255, 0, 255], 2, 1);
    /// if let Some(brightness) = img.get_region_average_brightness(0, 0, 2, 1) {
    ///     println!("Average brightness: {}", brightness);
    /// }
    /// ```
    pub fn get_region_average_brightness(&self, x: u32, y: u32, width: u32, height: u32) -> Option<u8> {
        // Validate bounds
        if x >= self.width || y >= self.height || width == 0 || height == 0 {
            return None;
        }

        let end_x = (x + width).min(self.width);
        let end_y = (y + height).min(self.height);
        let actual_width = end_x - x;
        let actual_height = end_y - y;

        if actual_width == 0 || actual_height == 0 {
            return None;
        }

        let mut sum_brightness: u64 = 0;
        let mut count: u64 = 0;

        for py in y..end_y {
            for px in x..end_x {
                let idx = ((py * self.width + px) * 4) as usize;
                let r = self.raw_pixels[idx] as f32;
                let g = self.raw_pixels[idx + 1] as f32;
                let b = self.raw_pixels[idx + 2] as f32;

                let brightness = (0.299 * r + 0.587 * g + 0.114 * b) as u64;
                sum_brightness += brightness;
                count += 1;
            }
        }

        Some((sum_brightness / count) as u8)
    }
}

/// Create a new PhotonImage from a raw Vec of u8s representing raw image pixels.
#[cfg(feature = "web-sys")]
impl From<ImageData> for PhotonImage {
    fn from(imgdata: ImageData) -> Self {
        let width = imgdata.width();
        let height = imgdata.height();
        let mut raw_pixels = to_raw_pixels(imgdata);
        Self::ensure_simd_aligned(&mut raw_pixels);
        PhotonImage {
            raw_pixels,
            width,
            height,
        }
    }
}

/// Color struct for representing RGBA colors.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn to_hex(&self, include_alpha: bool) -> String {
        if include_alpha {
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        } else {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        }
    }

    pub fn brightness(&self) -> u8 {
        (0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32) as u8
    }
}

/// RGB color type.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
impl Rgb {
    #[cfg_attr(feature = "enable_wasm", wasm_bindgen(constructor))]
    /// Create a new RGB struct.
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }

    /// Set the Red value.
    pub fn set_red(&mut self, r: u8) {
        self.r = r;
    }

    /// Get the Green value.
    pub fn set_green(&mut self, g: u8) {
        self.g = g;
    }

    /// Set the Blue value.
    pub fn set_blue(&mut self, b: u8) {
        self.b = b;
    }

    /// Get the Red value.
    pub fn get_red(&self) -> u8 {
        self.r
    }

    /// Get the Green value.
    pub fn get_green(&self) -> u8 {
        self.g
    }

    /// Get the Blue value.
    pub fn get_blue(&self) -> u8 {
        self.b
    }
}

impl From<Vec<u8>> for Rgb {
    fn from(vec: Vec<u8>) -> Self {
        if vec.len() != 3 {
            panic!("Vec length must be equal to 3.")
        }
        Rgb::new(vec[0], vec[1], vec[2])
    }
}

/// RGBA color type.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
impl Rgba {
    #[cfg_attr(feature = "enable_wasm", wasm_bindgen(constructor))]
    /// Create a new RGBA struct.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }

    /// Set the Red value.
    pub fn set_red(&mut self, r: u8) {
        self.r = r;
    }

    /// Get the Green value.
    pub fn set_green(&mut self, g: u8) {
        self.g = g;
    }

    /// Set the Blue value.
    pub fn set_blue(&mut self, b: u8) {
        self.b = b;
    }

    /// Set the alpha value.
    pub fn set_alpha(&mut self, a: u8) {
        self.a = a;
    }

    /// Get the Red value.
    pub fn get_red(&self) -> u8 {
        self.r
    }

    /// Get the Green value.
    pub fn get_green(&self) -> u8 {
        self.g
    }

    /// Get the Blue value.
    pub fn get_blue(&self) -> u8 {
        self.b
    }

    /// Get the alpha value for this color.
    pub fn get_alpha(&self) -> u8 {
        self.a
    }
}

impl From<Vec<u8>> for Rgba {
    fn from(vec: Vec<u8>) -> Self {
        if vec.len() != 4 {
            panic!("Vec length must be equal to 4.")
        }
        Rgba::new(vec[0], vec[1], vec[2], vec[3])
    }
}

///! [temp] Check if WASM is supported.
#[cfg(feature = "enable_wasm")]
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();
    ensure_default_font_initialized();

    let window = web_sys::window().expect("No Window found, should have a Window");
    let document = window
        .document()
        .expect("No Document found, should have a Document");

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("You're successfully running WASM!"));

    let body = document
        .body()
        .expect("ERR: No body found, should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;
    Ok(())
}

/// Get the ImageData from a 2D canvas context
#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn get_image_data(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
) -> ImageData {
    set_panic_hook();
    let width = canvas.width();
    let height = canvas.height();

    // let data: ImageData = ctx.get_image_data(0.0, 0.0, 100.0, 100.0).unwrap();
    let data = ctx
        .get_image_data(0.0, 0.0, width as f64, height as f64)
        .unwrap();
    let _vec_data = data.data().to_vec();
    data
}

/// Place a PhotonImage onto a 2D canvas.
#[cfg(all(feature = "web-sys", feature = "wasm-bindgen"))]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[allow(non_snake_case)]
#[allow(clippy::unnecessary_mut_passed)]
pub fn putImageData(
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    new_image: PhotonImage,
) {
    // Convert the raw pixels back to an ImageData object.
    let mut raw_pixels = new_image.raw_pixels;
    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut raw_pixels),
        canvas.width(),
        canvas.height(),
    );

    // Place the new imagedata onto the canvas
    ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0)
        .expect("Should put image data on Canvas");
}

/// Convert a HTML5 Canvas Element to a PhotonImage.
///
/// This converts the ImageData found in the canvas context to a PhotonImage,
/// which can then have effects or filters applied to it.
#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[no_mangle]
pub fn open_image(
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
) -> PhotonImage {
    let imgdata = get_image_data(&canvas, &ctx);
    let mut raw_pixels = to_raw_pixels(imgdata);
    PhotonImage::ensure_simd_aligned(&mut raw_pixels);
    PhotonImage {
        raw_pixels,
        width: canvas.width(),
        height: canvas.height(),
    }
}

/// Convert ImageData to a raw pixel vec of u8s.
#[cfg(feature = "web-sys")]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn to_raw_pixels(imgdata: ImageData) -> Vec<u8> {
    imgdata.data().to_vec()
}

/// Convert a base64 string to a PhotonImage.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn base64_to_image(base64: &str) -> PhotonImage {
    let base64_to_vec: Vec<u8> = base64_to_vec(base64);

    let slice = base64_to_vec.as_slice();

    let mut img = image::load_from_memory(slice).unwrap();
    img = ImageRgba8(img.to_rgba8());

    let width = img.width();
    let height = img.height();

    let mut raw_pixels = img.into_bytes();
    PhotonImage::ensure_simd_aligned(&mut raw_pixels);

    PhotonImage {
        raw_pixels,
        width,
        height,
    }
}

/// Convert a base64 string to a Vec of u8s.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn base64_to_vec(base64: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(base64).unwrap()
}

/// Convert a PhotonImage to JS-compatible ImageData.
#[cfg(all(feature = "web-sys", feature = "wasm-bindgen"))]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[allow(clippy::unnecessary_mut_passed)]
pub fn to_image_data(photon_image: PhotonImage) -> ImageData {
    let mut raw_pixels = photon_image.raw_pixels;
    let width = photon_image.width;
    let height = photon_image.height;
    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height)
        .unwrap()
}

#[cfg(not(target_os = "wasi"))]
fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub mod adaptive;
pub mod channels;
pub mod colour_spaces;
pub mod conv;
pub mod effects;
pub mod filters;
pub mod helpers;
pub mod monochrome;
pub mod multiple;
pub mod native;
pub mod noise;
pub mod parallel;
pub mod simd;
mod simd_conv;
mod tests;
pub mod text;
pub mod transform;
pub mod wasm_optimizations;

// Re-export optimized color space functions
pub use colour_spaces::{hsl_fast, hsv_fast, hsl_adaptive, hsv_adaptive};

// Re-export optimized multiple image functions
pub use multiple::{blend_fast, watermark_fast, blend_adaptive, watermark_adaptive};

// Re-export adaptive optimization utilities
pub use adaptive::{ImageSize, get_image_size, get_optimal_batch_size, get_optimal_chunk_size};

// Re-export parallel processing functions
#[cfg(all(feature = "enable_wasm", target_arch = "wasm32"))]
pub use parallel::{
    init_parallel,
    invert_parallel,
    grayscale_parallel,
    adjust_brightness_parallel,
    adjust_contrast_parallel,
    threshold_parallel,
    add_noise_rand_parallel,
};
