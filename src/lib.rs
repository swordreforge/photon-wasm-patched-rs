mod types;
mod brush;
mod edge_refinement;
mod image_operations;

use wasm_bindgen::prelude::*;
use std::collections::VecDeque;

// ==================== 重新导出类型 ====================

pub use types::{ColorSpace, BrushType, BlendMode, BrushConfig, StrokePoint};
pub use brush::{BrushStroke, DataConverter, StrokeGenerator, BrushRenderer};

// ==================== 重新导出边缘优化模块 ====================

pub use edge_refinement::{
    create_polygon_mask,
    create_circular_mask,
    apply_mask_to_image,
    refine_mask_edges,
    auto_crop_by_color,
};

// ==================== 图像处理器 ====================

/// 图像处理器结构体
/// 这是公共 API，封装了所有图像处理功能
#[wasm_bindgen]
pub struct ImageProcessor {
    image: photon_rs::PhotonImage,
    original_image: photon_rs::PhotonImage,
    original_bytes: Vec<u8>,
    width: u32,
    height: u32,
    // 笔刷相关字段
    current_stroke: Option<BrushStroke>,
    strokes_history: VecDeque<BrushStroke>,
    max_history_size: usize,
}

impl ImageProcessor {
    /// 创建内部处理器实例
    fn create_internal(&self) -> image_operations::ImageProcessor {
        image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        }
    }

    /// 更新图像数据
    fn update_from_internal(&mut self, internal: image_operations::ImageProcessor) {
        self.image = internal.image;
        self.original_image = internal.original_image;
        self.width = internal.width;
        self.height = internal.height;
    }
}

#[wasm_bindgen]
impl ImageProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, data: &[u8]) -> Result<ImageProcessor, JsValue> {
        let processor = image_operations::ImageProcessor::new(width, height, data)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(ImageProcessor {
            image: processor.image,
            original_image: processor.original_image,
            original_bytes: processor.original_bytes,
            width: processor.width,
            height: processor.height,
            current_stroke: None,
            strokes_history: VecDeque::new(),
            max_history_size: 50,
        })
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Result<ImageProcessor, JsValue> {
        let processor = image_operations::ImageProcessor::new_from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(ImageProcessor {
            image: processor.image,
            original_image: processor.original_image,
            original_bytes: processor.original_bytes,
            width: processor.width,
            height: processor.height,
            current_stroke: None,
            strokes_history: VecDeque::new(),
            max_history_size: 50,
        })
    }

    /// 创建指定大小的白色画布
    #[allow(unused)]
    #[wasm_bindgen(static_method_of = ImageProcessor)]
    pub fn new_white_canvas(width: u32, height: u32) -> Result<ImageProcessor, JsValue> {
        let pixel_count = (width * height) as usize;
        let data = vec![255u8; pixel_count * 4];

        let image = photon_rs::PhotonImage::new(data, width, height);
        let original_bytes = vec![255u8; pixel_count * 4];

        Ok(ImageProcessor {
            image: image.clone(),
            original_image: image,
            original_bytes,
            width,
            height,
            current_stroke: None,
            strokes_history: VecDeque::new(),
            max_history_size: 50,
        })
    }

    // ==================== 图像基本信息 ====================

    pub fn to_base64(&self) -> String {
        image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        }.to_base64()
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        }.get_bytes()
    }

    pub fn get_raw_pixels(&self) -> Vec<u8> {
        image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        }.get_raw_pixels()
    }

    pub fn get_estimated_filesize(&self) -> u64 {
        image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        }.get_estimated_filesize()
    }

    // ==================== 图像格式转换 ====================

    pub fn to_jpeg(&mut self, quality: u8) -> String {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        let result = processor.to_jpeg(quality);
        self.image = processor.image;
        result
    }

    pub fn to_png(&mut self) -> String {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        let result = processor.to_png();
        self.image = processor.image;
        result
    }

    pub fn to_webp(&mut self, quality: u8) -> String {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        let result = processor.to_webp(quality);
        self.image = processor.image;
        result
    }

    // ==================== 重置功能 ====================

    pub fn reset(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.reset();
        self.image = processor.image;
        self.original_image = processor.original_image;
    }

    // ==================== 文本绘制功能 ====================

    pub fn draw_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font_size: f32,
        font_type: Option<u8>,
        has_shadow: Option<bool>,
        color_r: Option<u8>,
        color_g: Option<u8>,
        color_b: Option<u8>,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.draw_text(text, x, y, font_size, font_type, has_shadow, color_r, color_g, color_b);
        self.image = processor.image;
    }

    pub fn draw_text_with_color_and_font(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font_size: f32,
        r: u8,
        g: u8,
        b: u8,
        font_type: u8,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.draw_text_with_color_and_font(text, x, y, font_size, r, g, b, font_type);
        self.image = processor.image;
    }

    pub fn draw_text_with_shadow_and_color_and_font(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font_size: f32,
        r: u8,
        g: u8,
        b: u8,
        font_type: u8,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.draw_text_with_shadow_and_color_and_font(text, x, y, font_size, r, g, b, font_type);
        self.image = processor.image;
    }

    pub fn draw_text_with_font_name(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font_size: f32,
        font_name: &str,
        has_shadow: Option<bool>,
        color_r: Option<u8>,
        color_g: Option<u8>,
        color_b: Option<u8>,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.draw_text_with_font_name(text, x, y, font_size, font_name, has_shadow, color_r, color_g, color_b);
        self.image = processor.image;
    }

    // ==================== 图像滤镜 ====================

    pub fn apply_grayscale(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_grayscale();
        self.image = processor.image;
    }

    pub fn apply_sepia(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_sepia();
        self.image = processor.image;
    }

    pub fn apply_invert(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_invert();
        self.image = processor.image;
    }

    pub fn apply_threshold(&mut self, threshold: u32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_threshold(threshold);
        self.image = processor.image;
    }

    pub fn apply_preset_filter(&mut self, filter_name: &str) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_preset_filter(filter_name);
        self.image = processor.image;
    }

    pub fn apply_lix(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_lix();
        self.image = processor.image;
    }

    pub fn apply_neue(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_neue();
        self.image = processor.image;
    }

    pub fn apply_ryo(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_ryo();
        self.image = processor.image;
    }

    // ==================== 特殊效果 ====================

    pub fn apply_pixelate(&mut self, pixel_size: i32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_pixelate(pixel_size);
        self.image = processor.image;
    }

    pub fn apply_halftone(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_halftone();
        self.image = processor.image;
    }

    pub fn apply_oil(&mut self, radius: i32, intensity: f64) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_oil(radius, intensity);
        self.image = processor.image;
    }

    pub fn apply_solarize(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_solarize();
        self.image = processor.image;
    }

    pub fn apply_dither(&mut self, depth: u32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_dither(depth);
        self.image = processor.image;
    }

    pub fn apply_duotone(&mut self, r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_duotone(r1, g1, b1, r2, g2, b2);
        self.image = processor.image;
    }

    pub fn apply_inc_brightness(&mut self, brightness: u8) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_inc_brightness(brightness);
        self.image = processor.image;
    }

    pub fn apply_gradient(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_gradient();
        self.image = processor.image;
    }

    pub fn apply_primary(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_primary();
        self.image = processor.image;
    }

    pub fn apply_colorize(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_colorize();
        self.image = processor.image;
    }

    pub fn apply_frosted_glass(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_frosted_glass();
        self.image = processor.image;
    }

    pub fn apply_tint(&mut self, r: u32, g: u32, b: u32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_tint(r, g, b);
        self.image = processor.image;
    }

    pub fn apply_normalize(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_normalize();
        self.image = processor.image;
    }

    pub fn offset_red(&mut self, offset_amt: u32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.offset_red(offset_amt);
        self.image = processor.image;
    }

    pub fn offset_green(&mut self, offset_amt: u32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.offset_green(offset_amt);
        self.image = processor.image;
    }

    pub fn offset_blue(&mut self, offset_amt: u32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.offset_blue(offset_amt);
        self.image = processor.image;
    }

    pub fn apply_strips(
        &mut self,
        num_strips: u8,
        horizontal: bool,
        color_r: Option<u8>,
        color_g: Option<u8>,
        color_b: Option<u8>,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_strips(num_strips, horizontal, color_r, color_g, color_b);
        self.image = processor.image;
    }

    // ==================== 可调节参数的滤镜 ====================

    pub fn apply_brightness(&mut self, level: i32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_brightness(level);
        self.image = processor.image;
    }

    pub fn apply_contrast(&mut self, level: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_contrast(level);
        self.image = processor.image;
    }

    pub fn apply_saturation(&mut self, level: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_saturation(level);
        self.image = processor.image;
    }

    pub fn apply_hue(&mut self, level: i32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_hue(level);
        self.image = processor.image;
    }

    pub fn apply_lightness(&mut self, level: f32, color_space: ColorSpace) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_lightness(level, color_space);
        self.image = processor.image;
    }

    pub fn apply_gamma(&mut self, red: f32, green: f32, blue: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_gamma(red, green, blue);
        self.image = processor.image;
    }

    pub fn apply_sharpen(&mut self, strength: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_sharpen(strength);
        self.image = processor.image;
    }

    pub fn apply_noise_reduction(&mut self, strength: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_noise_reduction(strength);
        self.image = processor.image;
    }

    pub fn apply_bilateral_filter(&mut self, sigma_spatial: f32, sigma_range: f32, fast_mode: bool) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_bilateral_filter(sigma_spatial, sigma_range, fast_mode);
        self.image = processor.image;
    }

    // ==================== 噪点效果 ====================

    pub fn apply_noise(&mut self, strength: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_noise(strength);
        self.image = processor.image;
    }

    pub fn apply_color_noise_with_strength(&mut self, r_factor: f32, g_factor: f32, b_factor: f32, strength: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_color_noise_with_strength(r_factor, g_factor, b_factor, strength);
        self.image = processor.image;
    }

    pub fn apply_pink_noise(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_pink_noise();
        self.image = processor.image;
    }

    // ==================== 批量应用多个调节 ====================

    pub fn apply_all_adjustments(
        &mut self,
        brightness: i32,
        contrast: f32,
        saturation: f32,
        hue: i32,
        lightness: f32,
        lightness_color_space: ColorSpace,
        gamma_red: f32,
        gamma_green: f32,
        gamma_blue: f32,
        sharpen_strength: f32,
        noise_reduction_strength: f32,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_all_adjustments(
            brightness, contrast, saturation, hue, lightness, lightness_color_space,
            gamma_red, gamma_green, gamma_blue, sharpen_strength, noise_reduction_strength,
        );
        self.image = processor.image;
    }

    // ==================== 变换操作 ====================

    pub fn rotate_90(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.rotate_90();
        self.image = processor.image;
        self.width = processor.width;
        self.height = processor.height;
    }

    pub fn rotate_any(&mut self, angle: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.rotate_any(angle);
        self.image = processor.image;
        self.width = processor.width;
        self.height = processor.height;
    }

    pub fn flip_horizontal(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.flip_horizontal();
        self.image = processor.image;
    }

    pub fn flip_vertical(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.flip_vertical();
        self.image = processor.image;
    }

    pub fn crop(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.crop(x1, y1, x2, y2);
        self.image = processor.image;
        self.width = processor.width;
        self.height = processor.height;
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.resize(new_width, new_height);
        self.image = processor.image;
        self.width = processor.width;
        self.height = processor.height;
    }

    // ==================== 单色效果 ====================

    pub fn apply_b_grayscale(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_b_grayscale();
        self.image = processor.image;
    }

    pub fn apply_desaturate(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_desaturate();
        self.image = processor.image;
    }

    pub fn apply_decompose_max(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_decompose_max();
        self.image = processor.image;
    }

    pub fn apply_decompose_min(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_decompose_min();
        self.image = processor.image;
    }

    pub fn apply_grayscale_human_corrected(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_grayscale_human_corrected();
        self.image = processor.image;
    }

    pub fn apply_grayscale_shades(&mut self, num_shades: u8) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_grayscale_shades(num_shades);
        self.image = processor.image;
    }

    // ==================== 模糊效果 ====================

    pub fn apply_box_blur(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_box_blur();
        self.image = processor.image;
    }

    pub fn apply_gaussian_blur(&mut self, radius: i32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_gaussian_blur(radius);
        self.image = processor.image;
    }

    // ==================== 边缘检测 ====================

    pub fn apply_sobel_horizontal(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_sobel_horizontal();
        self.image = processor.image;
    }

    pub fn apply_sobel_vertical(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_sobel_vertical();
        self.image = processor.image;
    }

    pub fn apply_sobel_global(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_sobel_global();
        self.image = processor.image;
    }

    pub fn apply_prewitt_horizontal(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_prewitt_horizontal();
        self.image = processor.image;
    }

    // ==================== 卷积效果 ====================

    pub fn apply_laplace(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_laplace();
        self.image = processor.image;
    }

    pub fn apply_emboss(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_emboss();
        self.image = processor.image;
    }

    pub fn apply_identity(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_identity();
        self.image = processor.image;
    }

    pub fn apply_edge_one(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_edge_one();
        self.image = processor.image;
    }

    pub fn apply_edge_detection(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_edge_detection();
        self.image = processor.image;
    }

    // ==================== 线条检测 ====================

    pub fn apply_detect_horizontal_lines(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_detect_horizontal_lines();
        self.image = processor.image;
    }

    pub fn apply_detect_vertical_lines(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_detect_vertical_lines();
        self.image = processor.image;
    }

    pub fn apply_detect_45_deg_lines(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_detect_45_deg_lines();
        self.image = processor.image;
    }

    pub fn apply_detect_135_deg_lines(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_detect_135_deg_lines();
        self.image = processor.image;
    }

    // ==================== 通道操作 ====================

    pub fn alter_red_channel(&mut self, amt: i16) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.alter_red_channel(amt);
        self.image = processor.image;
    }

    pub fn alter_green_channel(&mut self, amt: i16) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.alter_green_channel(amt);
        self.image = processor.image;
    }

    pub fn alter_blue_channel(&mut self, amt: i16) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.alter_blue_channel(amt);
        self.image = processor.image;
    }

    pub fn remove_red_channel(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.remove_red_channel();
        self.image = processor.image;
    }

    pub fn remove_green_channel(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.remove_green_channel();
        self.image = processor.image;
    }

    pub fn remove_blue_channel(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.remove_blue_channel();
        self.image = processor.image;
    }

    pub fn swap_rg_channels(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.swap_rg_channels();
        self.image = processor.image;
    }

    pub fn swap_gb_channels(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.swap_gb_channels();
        self.image = processor.image;
    }

    pub fn swap_rb_channels(&mut self) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.swap_rb_channels();
        self.image = processor.image;
    }

    // ==================== 色彩空间 ====================

    pub fn hue_rotate(&mut self, degrees: f32, color_space: u8) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.hue_rotate(degrees, color_space);
        self.image = processor.image;
    }

    pub fn adjust_lightness(&mut self, level: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.adjust_lightness(level);
        self.image = processor.image;
    }

    pub fn adjust_saturation(&mut self, level: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.adjust_saturation(level);
        self.image = processor.image;
    }

    pub fn hue_rotate_hsl(&mut self, degrees: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.hue_rotate_hsl(degrees);
        self.image = processor.image;
    }

    pub fn lighten_hsl(&mut self, level: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.lighten_hsl(level);
        self.image = processor.image;
    }

    pub fn darken_hsl(&mut self, level: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.darken_hsl(level);
        self.image = processor.image;
    }

    pub fn saturate_hsl(&mut self, level: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.saturate_hsl(level);
        self.image = processor.image;
    }

    pub fn desaturate_hsl(&mut self, level: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.desaturate_hsl(level);
        self.image = processor.image;
    }

    // ==================== 水印功能 ====================

    pub fn apply_watermark(
        &mut self,
        watermark_bytes: &[u8],
        x: i64,
        y: i64,
        scale: Option<f32>,
        opacity: Option<f32>,
        rotation: Option<f32>,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_watermark(watermark_bytes, x, y, scale, opacity, rotation);
        self.image = processor.image;
    }

    pub fn apply_watermark_with_blend(
        &mut self,
        watermark_bytes: &[u8],
        x: i64,
        y: i64,
        scale: f32,
        blend_mode: &str,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_watermark_with_blend(watermark_bytes, x, y, scale, blend_mode);
        self.image = processor.image;
    }

    // ==================== 多图混合功能 ====================

    pub fn blend_images(
        &mut self,
        overlay_bytes: &[u8],
        blend_mode: &str,
        scale: Option<f32>,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.blend_images(overlay_bytes, blend_mode, scale);
        self.image = processor.image;
    }

    pub fn blend_images_with_scale(
        &mut self,
        overlay_bytes: &[u8],
        scale: f32,
        blend_mode: &str,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.blend_images_with_scale(overlay_bytes, scale, blend_mode);
        self.image = processor.image;
    }

    // ==================== 边缘优化功能 ====================

    pub fn apply_polygon_mask(
        &mut self,
        vertices: Vec<f32>,
        anti_aliased: bool,
        smooth_edges: bool,
        smoothing_radius: u32,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_polygon_mask(vertices, anti_aliased, smooth_edges, smoothing_radius);
        self.image = processor.image;
    }

    pub fn apply_circular_mask(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        feather_radius: f32,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.apply_circular_mask(center_x, center_y, radius, feather_radius);
        self.image = processor.image;
    }

    pub fn auto_crop_by_color(
        &mut self,
        target_r: u8,
        target_g: u8,
        target_b: u8,
        tolerance: u8,
        feather_radius: f32,
    ) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.auto_crop_by_color(target_r, target_g, target_b, tolerance, feather_radius);
        self.image = processor.image;
    }

    pub fn refine_edges(&mut self, smoothing_radius: u32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.refine_edges(smoothing_radius);
        self.image = processor.image;
    }

    pub fn smart_crop(&mut self, threshold: u8, feather_radius: f32) {
        let mut processor = image_operations::ImageProcessor {
            image: self.image.clone(),
            original_image: self.original_image.clone(),
            original_bytes: self.original_bytes.clone(),
            width: self.width,
            height: self.height,
        };
        processor.smart_crop(threshold, feather_radius);
        self.image = processor.image;
    }

    // ==================== 笔刷功能 ====================

    /// 开始一笔新画
    pub fn begin_stroke(&mut self, config: BrushConfig) {
        self.current_stroke = Some(BrushStroke::new(config));
    }

    /// 添加点到当前笔划
    pub fn add_stroke_point(&mut self, x: f32, y: f32, pressure: f32) {
        if let Some(ref mut stroke) = self.current_stroke {
            let timestamp = js_sys::Date::now() as u64;
            let point = StrokePoint::new(x, y, pressure, timestamp);
            stroke.add_point(point);
            stroke.invalidate_path_cache();
        }
    }

    /// 结束当前笔划并渲染
    pub fn end_stroke(&mut self) {
        if let Some(mut stroke) = self.current_stroke.take() {
            if stroke.get_points_count() < 2 {
                return;
            }

            if stroke.cached_path.is_none() {
                let generator = StrokeGenerator::new(stroke.config);
                stroke.cached_path = generator.generate_path(stroke.get_points_ref());
            }

            if let Err(e) = BrushRenderer::render_stroke(&mut self.image, &stroke) {
                web_sys::console::error_1(&format!("Failed to render stroke: {}", e).into());
            } else {
                self.strokes_history.push_back(stroke);

                if self.strokes_history.len() > self.max_history_size {
                    self.strokes_history.pop_front();
                }
            }
        }
    }

    /// 绘制一笔（高性能版本，使用 Float32Array）
    #[wasm_bindgen]
    pub fn draw_stroke_array(
        &mut self,
        points_array: js_sys::Float32Array,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        width: f32,
    ) {
        let len = points_array.length() as usize;
        if len < 2 || len % 2 != 0 {
            return;
        }

        let mut points = Vec::with_capacity(len / 2);
        let timestamp = js_sys::Date::now() as u64;

        for i in (0..len).step_by(2) {
            let x = points_array.get_index(i as u32);
            let y = points_array.get_index(i as u32 + 1);
            points.push(StrokePoint::new(x, y, 1.0, timestamp));
        }

        if points.is_empty() {
            return;
        }

        let config = BrushConfig {
            brush_type: BrushType::Basic,
            base_width: width,
            color_r: r,
            color_g: g,
            color_b: b,
            color_a: a,
            blend_mode: BlendMode::Normal,
            smoothness: 0.5,
            pressure_sensitivity: 0.5,
        };

        let mut stroke = BrushStroke::new(config);
        *stroke.get_points_mut() = points;

        if stroke.cached_path.is_none() {
            let generator = StrokeGenerator::new(stroke.config);
            stroke.cached_path = generator.generate_path(stroke.get_points_ref());
        }

        if let Err(e) = BrushRenderer::render_stroke(&mut self.image, &stroke) {
            web_sys::console::error_1(&format!("Failed to render stroke: {}", e).into());
        } else {
            self.strokes_history.push_back(stroke);

            if self.strokes_history.len() > self.max_history_size {
                self.strokes_history.pop_front();
            }
        }
    }

    /// 直接绘制一笔（简化接口，向后兼容）
    pub fn draw_stroke(
        &mut self,
        points_js: JsValue,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        width: f32,
    ) {
        let points = if points_js.is_instance_of::<js_sys::Float32Array>() {
            let float_array = js_sys::Float32Array::from(points_js);
            let len = float_array.length() as usize;
            if len < 2 || len % 2 != 0 {
                return;
            }

            let mut result = Vec::with_capacity(len / 2);
            let timestamp = js_sys::Date::now() as u64;

            for i in (0..len).step_by(2) {
                let x = float_array.get_index(i as u32);
                let y = float_array.get_index(i as u32 + 1);
                result.push(StrokePoint::new(x, y, 1.0, timestamp));
            }
            result
        } else if points_js.is_instance_of::<js_sys::Array>() {
            let array: js_sys::Array = points_js.into();
            let mut result = Vec::with_capacity(array.length() as usize);
            let timestamp = js_sys::Date::now() as u64;

            for i in 0..array.length() {
                let point_obj = array.get(i);
                if let Some(obj) = point_obj.dyn_ref::<js_sys::Object>() {
                    let x_val = js_sys::Reflect::get(obj, &"x".into()).unwrap();
                    let y_val = js_sys::Reflect::get(obj, &"y".into()).unwrap();
                    let x = x_val.as_f64().unwrap() as f32;
                    let y = y_val.as_f64().unwrap() as f32;

                    result.push(StrokePoint::new(x, y, 1.0, timestamp));
                }
            }
            result
        } else {
            return;
        };

        if points.is_empty() {
            return;
        }

        let config = BrushConfig {
            brush_type: BrushType::Basic,
            base_width: width,
            color_r: r,
            color_g: g,
            color_b: b,
            color_a: a,
            blend_mode: BlendMode::Normal,
            smoothness: 0.5,
            pressure_sensitivity: 0.5,
        };

        let mut stroke = BrushStroke::new(config);
        *stroke.get_points_mut() = points;

        if stroke.cached_path.is_none() {
            let generator = StrokeGenerator::new(stroke.config);
            stroke.cached_path = generator.generate_path(stroke.get_points_ref());
        }

        if let Err(e) = BrushRenderer::render_stroke(&mut self.image, &stroke) {
            web_sys::console::error_1(&format!("Failed to render stroke: {}", e).into());
        } else {
            self.strokes_history.push_back(stroke);

            if self.strokes_history.len() > self.max_history_size {
                self.strokes_history.pop_front();
            }
        }
    }

    /// 撤销最后一笔
    pub fn undo_stroke(&mut self) -> bool {
        if self.strokes_history.pop_back().is_some() {
            if let Ok(new_image) = photon_rs::native::open_image_from_bytes(&self.original_bytes) {
                self.image = new_image;

                for stroke in &self.strokes_history {
                    let _ = BrushRenderer::render_stroke(&mut self.image, stroke);
                }
            }
            true
        } else {
            false
        }
    }

    /// 清除所有笔划
    pub fn clear_strokes(&mut self) {
        self.strokes_history.clear();
        if let Ok(new_image) = photon_rs::native::open_image_from_bytes(&self.original_bytes) {
            self.original_image = new_image.clone();
            self.image = new_image;
        }
    }

    /// 获取历史笔划数量
    pub fn get_stroke_count(&self) -> usize {
        self.strokes_history.len()
    }

    // ==================== 取色器API ====================

    /// 获取指定坐标的像素颜色
    ///
    /// # 参数
    /// * `x` - X 坐标 (0 到 width-1)
    /// * `y` - Y 坐标 (0 到 height-1)
    ///
    /// # 返回值
    /// 返回包含 RGBA 值的 JsValue (Uint8Array)，如果坐标超出范围则返回 null
    pub fn get_pixel_color(&self, x: u32, y: u32) -> JsValue {
        if let Some(color) = photon_rs::PhotonImage::get_pixel_color(&self.image, x, y) {
            let rgba = vec![color.r, color.g, color.b, color.a];
            unsafe { JsValue::from(js_sys::Uint8Array::from(rgba.as_slice())) }
        } else {
            JsValue::null()
        }
    }

    /// 获取指定坐标的像素颜色的十六进制表示
    ///
    /// # 参数
    /// * `x` - X 坐标 (0 到 width-1)
    /// * `y` - Y 坐标 (0 到 height-1)
    /// * `include_alpha` - 是否包含 alpha 通道
    ///
    /// # 返回值
    /// 返回十六进制颜色字符串，如果坐标超出范围则返回 null
    pub fn get_pixel_color_hex(&self, x: u32, y: u32, include_alpha: bool) -> Option<String> {
        self.image.get_pixel_color_hex(x, y, include_alpha)
    }

    /// 获取指定坐标的像素亮度
    ///
    /// # 参数
    /// * `x` - X 坐标 (0 到 width-1)
    /// * `y` - Y 坐标 (0 到 height-1)
    ///
    /// # 返回值
    /// 返回亮度值 (0-255)，如果坐标超出范围则返回 null
    pub fn get_pixel_brightness(&self, x: u32, y: u32) -> Option<u8> {
        self.image.get_pixel_brightness(x, y)
    }

    /// 获取指定区域的平均颜色
    ///
    /// # 参数
    /// * `x` - 区域左上角 X 坐标
    /// * `y` - 区域左上角 Y 坐标
    /// * `width` - 区域宽度
    /// * `height` - 区域高度
    ///
    /// # 返回值
    /// 返回包含 RGBA 平均值的 JsValue (Uint8Array)，如果区域超出范围则返回 null
    pub fn get_region_average_color(&self, x: u32, y: u32, width: u32, height: u32) -> JsValue {
        if let Some(color) = self.image.get_region_average_color(x, y, width, height) {
            let rgba = vec![color.r, color.g, color.b, color.a];
            unsafe { JsValue::from(js_sys::Uint8Array::from(rgba.as_slice())) }
        } else {
            JsValue::null()
        }
    }

    /// 获取指定区域的平均亮度
    ///
    /// # 参数
    /// * `x` - 区域左上角 X 坐标
    /// * `y` - 区域左上角 Y 坐标
    /// * `width` - 区域宽度
    /// * `height` - 区域高度
    ///
    /// # 返回值
    /// 返回平均亮度值 (0-255)，如果区域超出范围则返回 null
    pub fn get_region_average_brightness(&self, x: u32, y: u32, width: u32, height: u32) -> Option<u8> {
        self.image.get_region_average_brightness(x, y, width, height)
    }

    /// 获取整个图像的主色调
    ///
    /// # 返回值
    /// 返回包含 RGBA 值的 JsValue (Uint8Array)
    pub fn get_dominant_color(&self) -> JsValue {
        let color = self.image.get_dominant_color();
        let rgba = vec![color.r, color.g, color.b, color.a];
        unsafe { JsValue::from(js_sys::Uint8Array::from(rgba.as_slice())) }
    }

    /// 获取指定区域的主色调
    ///
    /// # 参数
    /// * `x` - 区域左上角 X 坐标
    /// * `y` - 区域左上角 Y 坐标
    /// * `width` - 区域宽度
    /// * `height` - 区域高度
    ///
    /// # 返回值
    /// 返回包含 RGBA 值的 JsValue (Uint8Array)，如果区域超出范围则返回 null
    pub fn get_region_dominant_color(&self, x: u32, y: u32, width: u32, height: u32) -> JsValue {
        if let Some(color) = self.image.get_region_dominant_color(x, y, width, height) {
            let rgba = vec![color.r, color.g, color.b, color.a];
            unsafe { JsValue::from(js_sys::Uint8Array::from(rgba.as_slice())) }
        } else {
            JsValue::null()
        }
    }

    /// 获取图像的调色板
    ///
    /// # 参数
    /// * `num_colors` - 要提取的颜色数量
    ///
    /// # 返回值
    /// 返回包含颜色数组的 JsValue (Array<Uint8Array>)，每个颜色是 [r, g, b, a] 格式
    pub fn get_color_palette(&self, num_colors: usize) -> JsValue {
        let palette = self.image.get_color_palette(num_colors);
        let array = js_sys::Array::new();
        for color in palette {
            let rgba = vec![color.r, color.g, color.b, color.a];
            let uint8_array = js_sys::Uint8Array::from(rgba.as_slice());
            array.push(&uint8_array.into());
        }
        array.into()
    }
}

// ==================== 字体注册功能（重新导出 photon_rs 的函数）====================

pub use photon_rs::text::{
    wasm_register_font,
    wasm_is_font_registered,
    wasm_get_registered_fonts,
    wasm_unregister_font,
    wasm_clear_fonts,
    get_default_font_name,
    is_default_font_initialized,
};

// ==================== 线程池初始化 =====================

#[cfg(feature = "wasm-bindgen-rayon")]
#[wasm_bindgen]
pub async fn init_thread_pool(num_threads: usize) -> Result<(), JsValue> {
    use wasm_bindgen_rayon::init_thread_pool;
    use wasm_bindgen_futures::JsFuture;

    let threads = if num_threads == 0 { 4 } else { num_threads };
    let promise = init_thread_pool(threads);

    JsFuture::from(promise).await.map(|_| ()).map_err(|e| e.unchecked_into())
}

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
}