use wasm_bindgen::prelude::*;
use photon_rs::{PhotonImage, filters, native, monochrome};
use base64::{Engine as _, engine::general_purpose};

#[wasm_bindgen]
pub struct ImageProcessor {
    image: PhotonImage,
    original_image: PhotonImage,
}

#[wasm_bindgen]
impl ImageProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, data: &[u8]) -> Result<ImageProcessor, JsValue> {
        let image = PhotonImage::new(data.to_vec(), width, height);
        let original_image = PhotonImage::new(data.to_vec(), width, height);
        Ok(ImageProcessor { image, original_image })
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Result<ImageProcessor, JsValue> {
        let image = native::open_image_from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to open image: {}", e)))?;
        let original_image = native::open_image_from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to open image: {}", e)))?;
        Ok(ImageProcessor { image, original_image })
    }

    pub fn to_base64(&self) -> String {
        let bytes = self.image.get_bytes();
        general_purpose::STANDARD.encode(&bytes)
    }

    pub fn get_width(&self) -> u32 {
        self.image.get_width()
    }

    pub fn get_height(&self) -> u32 {
        self.image.get_height()
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
        let width = self.original_image.get_width();
        let height = self.original_image.get_height();
        let bytes = self.original_image.get_bytes();
        self.image = PhotonImage::new(bytes, width, height);
    }

    // 可调节参数的滤镜 - 使用混合方式
    pub fn apply_brightness(&mut self, level: i32) {
        // 先重置到原始图像
        self.reset();
        // 应用亮度调整
        let _ = filters::filter(&mut self.image, &format!("brightness:{}", level));
    }

    pub fn apply_contrast(&mut self, level: f32) {
        self.reset();
        let _ = filters::filter(&mut self.image, &format!("contrast:{}", level));
    }

    pub fn apply_saturation(&mut self, level: f32) {
        self.reset();
        let _ = filters::filter(&mut self.image, &format!("saturation:{}", level));
    }

    pub fn apply_hue(&mut self, level: i32) {
        self.reset();
        let _ = filters::filter(&mut self.image, &format!("hue:{}", level));
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}