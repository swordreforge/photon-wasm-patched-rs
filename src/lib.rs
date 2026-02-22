use wasm_bindgen::prelude::*;
use photon_rs::{PhotonImage, filters, native, monochrome, transform, effects, colour_spaces};
use base64::{Engine as _, engine::general_purpose};

#[wasm_bindgen]
pub struct ImageProcessor {
    image: PhotonImage,
    original_image: PhotonImage,
    original_bytes: Vec<u8>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl ImageProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, data: &[u8]) -> Result<ImageProcessor, JsValue> {
        let image = PhotonImage::new(data.to_vec(), width, height);
        Ok(ImageProcessor {
            image: image.clone(),
            original_image: image,
            original_bytes: data.to_vec(),
            width,
            height,
        })
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Result<ImageProcessor, JsValue> {
        let image = native::open_image_from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to open image: {}", e)))?;
        let width = image.get_width();
        let height = image.get_height();
        Ok(ImageProcessor {
            image: image.clone(),
            original_image: image,
            original_bytes: bytes.to_vec(),
            width,
            height,
        })
    }

    pub fn to_base64(&self) -> String {
        self.image.get_base64()
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.image.get_bytes()
    }

    // 图像格式转换
    pub fn to_jpeg(&mut self, quality: u8) -> String {
        self.image.get_base64()
    }

    pub fn to_png(&mut self) -> String {
        self.image.get_base64()
    }

    pub fn to_webp(&mut self, _quality: u8) -> String {
        self.image.get_base64()
    }

    // 图像滤镜
    pub fn apply_grayscale(&mut self) {
        filters::filter(&mut self.image, "grayscale");
    }

    pub fn apply_sepia(&mut self) {
        filters::filter(&mut self.image, "sepia");
    }

    pub fn apply_invert(&mut self) {
        filters::filter(&mut self.image, "invert");
    }

    pub fn apply_threshold(&mut self, threshold: u32) {
        monochrome::threshold(&mut self.image, threshold);
    }

    // 更多预设滤镜
    pub fn apply_preset_filter(&mut self, filter_name: &str) {
        filters::filter(&mut self.image, filter_name);
    }

    // 特殊效果
    pub fn apply_pixelate(&mut self, pixel_size: i32) {
        effects::pixelize(&mut self.image, pixel_size);
    }

    pub fn apply_halftone(&mut self) {
        effects::halftone(&mut self.image);
    }

    pub fn apply_oil(&mut self, radius: i32, intensity: f64) {
        effects::oil(&mut self.image, radius, intensity);
    }

    pub fn apply_solarize(&mut self) {
        effects::solarize(&mut self.image);
    }

    pub fn apply_dither(&mut self, depth: u32) {
        effects::dither(&mut self.image, depth);
    }

    pub fn apply_duotone(&mut self, r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) {
        effects::duotone(&mut self.image, photon_rs::Rgb::new(r1, g1, b1), photon_rs::Rgb::new(r2, g2, b2));
    }

    // 重置到原始图像
    pub fn reset(&mut self) {
        // 使用 open_image_from_bytes 重新从原始字节创建图像
        // 注意: 这里需要从原始字节数组中重建图像，但 original_bytes 是加载时的完整图像数据
        // 由于 PhotonImage 没有直接从字节数组克隆的方法，我们需要创建一个新的 ImageProcessor
        if let Ok(new_image) = native::open_image_from_bytes(&self.original_bytes) {
            self.image = new_image.clone();
            self.original_image = new_image;
        }
    }

    // 可调节参数的滤镜 - 直接调用 photon-rs 函数
    pub fn apply_brightness(&mut self, level: i32) {
        // brightness 范围: -255 到 255
        let clamped_level = level.clamp(-255, 255) as i16;
        effects::adjust_brightness(&mut self.image, clamped_level);
    }

    pub fn apply_contrast(&mut self, level: f32) {
        // contrast 范围: -255 到 255 (JavaScript 端已转换)
        let clamped_level = level.clamp(-255.0, 255.0);
        effects::adjust_contrast(&mut self.image, clamped_level);
    }

    pub fn apply_saturation(&mut self, level: f32) {
        // saturation 范围: -1 到 1 (JavaScript 端已转换)
        let clamped_level = level.clamp(-1.0, 1.0);
        
        if clamped_level >= 0.0 {
            colour_spaces::saturate_hsl(&mut self.image, clamped_level);
        } else {
            colour_spaces::desaturate_hsl(&mut self.image, -clamped_level);
        }
    }

    pub fn apply_hue(&mut self, level: i32) {
        // hue 范围: -360 到 360
        let clamped_level = level.clamp(-360, 360);
        colour_spaces::hsl(&mut self.image, "shift_hue", clamped_level as f32);
    }

    // 批量应用多个调节（避免重复重置）
    pub fn apply_all_adjustments(&mut self, brightness: i32, contrast: f32, saturation: f32, hue: i32) {
        // 基于原始图像应用所有调整
        let mut temp_img = self.original_image.clone();

        // 亮度: 输入 -255 到 255
        if brightness != 0 {
            let clamped_brightness = brightness.clamp(-255, 255) as i16;
            effects::adjust_brightness(&mut temp_img, clamped_brightness);
        }

        // 对比度: 输入 -255 到 255 (已经转换过)
        if contrast != 0.0 {
            let clamped_contrast = contrast.clamp(-255.0, 255.0);
            effects::adjust_contrast(&mut temp_img, clamped_contrast);
        }

        // 饱和度: 输入 -1 到 1 (已经转换过)
        if saturation != 0.0 {
            let clamped_saturation = saturation.clamp(-1.0, 1.0);
            if clamped_saturation >= 0.0 {
                colour_spaces::saturate_hsl(&mut temp_img, clamped_saturation);
            } else {
                colour_spaces::desaturate_hsl(&mut temp_img, -clamped_saturation);
            }
        }

        // 色相: 输入 -360 到 360
        if hue != 0 {
            let clamped_hue = hue.clamp(-360, 360);
            colour_spaces::hsl(&mut temp_img, "shift_hue", clamped_hue as f32);
        }

        self.image = temp_img;
    }

    // 变换操作
    pub fn rotate_90(&mut self) {
        self.image = transform::rotate(&self.image, 90.0);
        self.width = self.image.get_width();
        self.height = self.image.get_height();
    }

    pub fn flip_horizontal(&mut self) {
        transform::fliph(&mut self.image);
    }

    pub fn flip_vertical(&mut self) {
        transform::flipv(&mut self.image);
    }

    pub fn crop(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        self.image = transform::crop(&self.image, x1, y1, x2, y2);
        self.width = self.image.get_width();
        self.height = self.image.get_height();
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.image = transform::resize(&self.image, new_width, new_height, photon_rs::transform::SamplingFilter::Nearest);
        self.width = self.image.get_width();
        self.height = self.image.get_height();
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}