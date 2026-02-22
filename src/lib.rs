use wasm_bindgen::prelude::*;
use photon_rs::{PhotonImage, filters, native, monochrome};
use base64::{Engine as _, engine::general_purpose};

#[wasm_bindgen]
pub struct ImageProcessor {
    image: PhotonImage,
}

#[wasm_bindgen]
impl ImageProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, data: &[u8]) -> Result<ImageProcessor, JsValue> {
        let image = PhotonImage::new(data.to_vec(), width, height);
        Ok(ImageProcessor { image })
    }

    pub fn new_from_bytes(bytes: &[u8]) -> Result<ImageProcessor, JsValue> {
        let image = native::open_image_from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to open image: {}", e)))?;
        Ok(ImageProcessor { image })
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
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}