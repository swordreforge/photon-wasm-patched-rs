use wasm_bindgen::prelude::*;
use photon_rs::{PhotonImage, filters, native, monochrome};
use base64::{Engine as _, engine::general_purpose};

#[wasm_bindgen]
pub struct ImageProcessor {
    image: PhotonImage,
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
            image,
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
            image,
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

    // 重置到原始图像
    pub fn reset(&mut self) {
        // 使用 open_image_from_bytes 重新从原始字节创建图像
        // 注意: 这里需要从原始字节数组中重建图像，但 original_bytes 是加载时的完整图像数据
        // 由于 PhotonImage 没有直接从字节数组克隆的方法，我们需要创建一个新的 ImageProcessor
        if let Ok(new_image) = native::open_image_from_bytes(&self.original_bytes) {
            self.image = new_image;
        }
    }

    // 可调节参数的滤镜 - 使用简单的滤镜字符串
    pub fn apply_brightness(&mut self, level: i32) {
        filters::filter(&mut self.image, &format!("brightness:{}", level));
    }

    pub fn apply_contrast(&mut self, level: f32) {
        filters::filter(&mut self.image, &format!("contrast:{}", level));
    }

    pub fn apply_saturation(&mut self, level: f32) {
        filters::filter(&mut self.image, &format!("saturation:{}", level));
    }

    pub fn apply_hue(&mut self, level: i32) {
        filters::filter(&mut self.image, &format!("hue:{}", level));
    }

    // 批量应用多个调节（避免重复重置）
    pub fn apply_all_adjustments(&mut self, brightness: i32, contrast: f32, saturation: f32, hue: i32) {
        self.reset();

        if brightness != 0 {
            filters::filter(&mut self.image, &format!("brightness:{}", brightness));
        }

        if contrast != 100.0 {
            filters::filter(&mut self.image, &format!("contrast:{}", contrast));
        }

        if saturation != 100.0 {
            filters::filter(&mut self.image, &format!("saturation:{}", saturation));
        }

        if hue != 0 {
            filters::filter(&mut self.image, &format!("hue:{}", hue));
        }
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}