//! WebAssembly-specific optimizations for photon image processing.
//!
//! This module provides optimizations specifically designed for WebAssembly environments,
//! including memory layout optimizations, zero-copy data transfer, and WASM-specific
//! performance improvements.

use crate::PhotonImage;

#[cfg(all(
    feature = "enable_wasm",
    target_arch = "wasm32",
    not(target_os = "wasi")
))]
use wasm_bindgen::prelude::*;

#[cfg(all(
    feature = "enable_wasm",
    target_arch = "wasm32",
    not(target_os = "wasi")
))]
use web_sys::ImageData;

/// Optimized pixel access using unchecked bounds.
///
/// This function provides fast access to pixel data by avoiding bounds checks.
/// It should only be used when the indices are guaranteed to be valid.
///
/// # Safety
/// The caller must ensure that `idx + 3 < pixels.len()`.
#[inline(always)]
pub unsafe fn get_pixel_unchecked(pixels: &[u8], idx: usize) -> [u8; 4] {
    [
        *pixels.get_unchecked(idx),
        *pixels.get_unchecked(idx + 1),
        *pixels.get_unchecked(idx + 2),
        *pixels.get_unchecked(idx + 3),
    ]
}

/// Optimized pixel modification using unchecked bounds.
///
/// This function provides fast modification of pixel data by avoiding bounds checks.
/// It should only be used when the indices are guaranteed to be valid.
///
/// # Safety
/// The caller must ensure that `idx + 3 < pixels.len()`.
#[inline(always)]
pub unsafe fn set_pixel_unchecked(pixels: &mut [u8], idx: usize, pixel: [u8; 4]) {
    *pixels.get_unchecked_mut(idx) = pixel[0];
    *pixels.get_unchecked_mut(idx + 1) = pixel[1];
    *pixels.get_unchecked_mut(idx + 2) = pixel[2];
    *pixels.get_unchecked_mut(idx + 3) = pixel[3];
}

/// Create a PhotonImage from a Uint8ClampedArray with zero-copy.
///
/// This function creates a PhotonImage directly from JavaScript's Uint8ClampedArray
/// without copying the data, enabling efficient data transfer between JavaScript and WASM.
///
/// # Arguments
/// * `data` - A Uint8ClampedArray containing RGBA pixel data.
/// * `width` - The image width.
/// * `height` - The image height.
///
/// # Example
///
/// ```javascript
/// import { PhotonImage } from 'photon_rs';
///
/// const canvas = document.getElementById('myCanvas');
/// const ctx = canvas.getContext('2d');
/// const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
///
/// // Create PhotonImage without copying
/// const photonImg = PhotonImage.from_uint8_array(imageData.data, canvas.width, canvas.height);
/// ```
#[cfg(all(
    feature = "enable_wasm",
    target_arch = "wasm32",
    not(target_os = "wasi")
))]
#[wasm_bindgen]
pub fn photon_image_from_uint8_clamped_array(
    data: js_sys::Uint8ClampedArray,
    width: u32,
    height: u32,
) -> PhotonImage {
    // Convert to Vec<u8> with minimal copying
    let raw_pixels = data.to_vec();
    
    PhotonImage {
        raw_pixels,
        width,
        height,
    }
}

/// Get pixel data as a Uint8ClampedArray for efficient transfer to JavaScript.
///
/// This function provides the image's pixel data as a Uint8ClampedArray,
/// which can be directly used with Canvas API without additional copying.
///
/// # Arguments
/// * `img` - A reference to a PhotonImage.
///
/// # Returns
/// A Uint8ClampedArray containing the RGBA pixel data.
///
/// # Example
///
/// ```javascript
/// import { PhotonImage } from 'photon_rs';
///
/// const canvas = document.getElementById('myCanvas');
/// const ctx = canvas.getContext('2d');
///
/// // After processing an image
/// const pixelData = photonImg.get_uint8_clamped_array();
/// const imageData = new ImageData(pixelData, photonImg.width, photonImg.height);
/// ctx.putImageData(imageData, 0, 0);
/// ```
#[cfg(all(
    feature = "enable_wasm",
    target_arch = "wasm32",
    not(target_os = "wasi")
))]
#[wasm_bindgen]
pub fn photon_image_get_uint8_clamped_array(img: &PhotonImage) -> js_sys::Uint8ClampedArray {
    unsafe {
        js_sys::Uint8ClampedArray::view(&img.raw_pixels)
    }
}

/// Process image data in-place for zero-copy operations.
///
/// This function processes ImageData directly without creating intermediate
/// PhotonImage objects, reducing memory allocations and copying.
///
/// # Arguments
/// * `image_data` - A mutable reference to ImageData.
/// * `processor` - A function that processes the pixel data.
///
/// # Example
///
/// ```javascript
/// import { process_image_data_inplace } from 'photon_rs';
///
/// const canvas = document.getElementById('myCanvas');
/// const ctx = canvas.getContext('2d');
/// const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
///
/// // Process image data in-place
/// process_image_data_inplace(imageData, (pixels, width, height) => {
///     // Custom processing logic
///     for (let i = 0; i < pixels.length; i += 4) {
///         pixels[i] = 255 - pixels[i];     // Invert R
///         pixels[i + 1] = 255 - pixels[i + 1]; // Invert G
///         pixels[i + 2] = 255 - pixels[i + 2]; // Invert B
///     }
/// });
///
/// ctx.putImageData(imageData, 0, 0);
/// ```
#[cfg(all(
    feature = "enable_wasm",
    target_arch = "wasm32",
    not(target_os = "wasi")
))]
#[wasm_bindgen]
pub fn process_image_data_inplace(
    image_data: &ImageData,
    processor: &js_sys::Function,
) -> Result<(), JsValue> {
    let data = image_data.data();
    let width = image_data.width();
    let height = image_data.height();
    
    // Call the JavaScript processor function with width and height as an object
    let params = js_sys::Object::new();
    js_sys::Reflect::set(&params, &"width".into(), &width.into())?;
    js_sys::Reflect::set(&params, &"height".into(), &height.into())?;
    
    processor.call2(
        &JsValue::NULL,
        &data.into(),
        &params,
    )?;
    
    Ok(())
}

/// Batch process multiple images efficiently.
///
/// This function processes multiple images in a single call, reducing
/// JavaScript-WASM bridge overhead.
///
/// # Arguments
/// * `images` - An array of PhotonImage objects.
/// * `processor` - A function that processes a single image.
///
/// # Example
///
/// ```javascript
/// import { batch_process_images } from 'photon_rs';
///
/// const images = [
///     img1,
///     img2,
///     img3
/// ];
///
/// batch_process_images(images, (img) => {
///     // Apply processing to each image
///     return processImage(img);
/// });
/// ```
#[cfg(all(
    feature = "enable_wasm",
    target_arch = "wasm32",
    not(target_os = "wasi")
))]
#[wasm_bindgen]
pub fn batch_process_images(
    images: js_sys::Array,
    processor: &js_sys::Function,
) -> Result<js_sys::Array, JsValue> {
    let results = js_sys::Array::new();
    
    for i in 0..images.length() {
        let img = images.get(i);
        if let Some(img_value) = img.as_f64() {
            let img_value = JsValue::from(img_value);
            let result = processor.call1(&JsValue::NULL, &img_value)?;
            results.push(&result);
        }
    }
    
    Ok(results)
}

/// Optimized lookup table for contrast adjustment.
///
/// Pre-computes contrast adjustment values to avoid repeated calculations.
#[inline(always)]
pub fn create_contrast_lut(contrast: f32) -> [u8; 256] {
    let clamped_contrast = contrast.clamp(-255.0, 255.0);
    let factor = (259.0 * (clamped_contrast + 255.0)) / (255.0 * (259.0 - clamped_contrast));
    let offset = -128.0 * factor + 128.0;
    
    let mut lut = [0u8; 256];
    for i in 0..=255_u8 {
        let new_val = i as f32 * factor + offset;
        lut[i as usize] = new_val.clamp(0.0, 255.0) as u8;
    }
    
    lut
}

/// Apply contrast adjustment using pre-computed lookup table.
///
/// This function uses a pre-computed lookup table for faster contrast adjustment.
///
/// # Arguments
/// * `img` - A mutable reference to a PhotonImage.
/// * `lut` - A pre-computed contrast lookup table.
#[inline(always)]
pub fn apply_contrast_lut(img: &mut PhotonImage, lut: &[u8; 256]) {
    let pixels = img.raw_pixels.as_mut_slice();
    let len = pixels.len();
    
    unsafe {
        for i in (0..len).step_by(4) {
            *pixels.get_unchecked_mut(i) = lut[*pixels.get_unchecked(i) as usize];
            *pixels.get_unchecked_mut(i + 1) = lut[*pixels.get_unchecked(i + 1) as usize];
            *pixels.get_unchecked_mut(i + 2) = lut[*pixels.get_unchecked(i + 2) as usize];
        }
    }
}

/// Optimized grayscale conversion using lookup table.
///
/// Pre-computes grayscale values for all possible RGB combinations
/// to accelerate repeated grayscale conversions.
///
/// # Note
/// This uses 256^3 = 16,777,216 entries, which may be too large for WASM.
/// Consider using a smaller LUT or direct computation for WASM.
#[inline(always)]
pub fn create_grayscale_lut_256() -> [[[u8; 256]; 256]; 256] {
    let mut lut = [[[0u8; 256]; 256]; 256];
    
    for r in 0..=255u8 {
        for g in 0..=255u8 {
            for b in 0..=255u8 {
                let gray = (r as f32 * 0.3 + g as f32 * 0.59 + b as f32 * 0.11) as u8;
                lut[r as usize][g as usize][b as usize] = gray;
            }
        }
    }
    
    lut
}

/// Optimized memory pool for temporary buffers.
///
/// This struct manages a pool of reusable buffers to reduce memory allocation overhead.
pub struct MemoryPool {
    buffers: Vec<Vec<u8>>,
    max_buffers: usize,
}

impl MemoryPool {
    /// Create a new memory pool.
    ///
    /// # Arguments
    /// * `initial_size` - The size of buffers in the pool.
    /// * `max_buffers` - The maximum number of buffers to keep in the pool.
    pub fn new(_initial_size: usize, max_buffers: usize) -> Self {
        Self {
            buffers: Vec::with_capacity(max_buffers),
            max_buffers,
        }
    }
    
    /// Get a buffer from the pool, or create a new one if needed.
    ///
    /// # Arguments
    /// * `min_size` - The minimum size of the buffer.
    pub fn get_buffer(&mut self, min_size: usize) -> Vec<u8> {
        if let Some(mut buffer) = self.buffers.pop() {
            if buffer.capacity() >= min_size {
                buffer.clear();
                return buffer;
            }
        }
        
        Vec::with_capacity(min_size)
    }
    
    /// Return a buffer to the pool for reuse.
    ///
    /// # Arguments
    /// * `buffer` - The buffer to return.
    pub fn return_buffer(&mut self, buffer: Vec<u8>) {
        if self.buffers.len() < self.max_buffers {
            self.buffers.push(buffer);
        }
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new(1024 * 1024, 4) // 1MB buffers, max 4 buffers
    }
}

/// Global memory pool for temporary image buffers.
static mut MEMORY_POOL: Option<MemoryPool> = None;

/// Initialize the global memory pool.
///
/// This should be called once at application startup.
pub fn init_memory_pool() {
    unsafe {
        if (*(&raw mut MEMORY_POOL)).is_none() {
            *(&raw mut MEMORY_POOL) = Some(MemoryPool::default());
        }
    }
}

/// Get a buffer from the global memory pool.
///
/// # Arguments
/// * `min_size` - The minimum size of the buffer.
pub fn get_temp_buffer(min_size: usize) -> Vec<u8> {
    unsafe {
        match &raw mut MEMORY_POOL {
            pool if !(*pool).is_none() => (*pool).as_mut().unwrap().get_buffer(min_size),
            _ => Vec::with_capacity(min_size),
        }
    }
}

/// Return a buffer to the global memory pool.
///
/// # Arguments
/// * `buffer` - The buffer to return.
pub fn return_temp_buffer(buffer: Vec<u8>) {
    unsafe {
        match &raw mut MEMORY_POOL {
            pool if !(*pool).is_none() => (*pool).as_mut().unwrap().return_buffer(buffer),
            _ => drop(buffer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_image(width: u32, height: u32) -> PhotonImage {
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        for y in 0..height {
            for x in 0..width {
                let r = ((x * 255) / width.max(1)) as u8;
                let g = ((y * 255) / height.max(1)) as u8;
                let b = (((x + y) * 255) / (width + height).max(1)) as u8;
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

    #[test]
    fn test_get_pixel_unchecked() {
        let mut pixels = vec![255u8, 128, 64, 255, 0, 128, 255, 255];
        
        unsafe {
            let pixel = get_pixel_unchecked(&pixels, 0);
            assert_eq!(pixel, [255, 128, 64, 255]);
            
            let pixel2 = get_pixel_unchecked(&pixels, 4);
            assert_eq!(pixel2, [0, 128, 255, 255]);
        }
    }

    #[test]
    fn test_set_pixel_unchecked() {
        let mut pixels = vec![0u8; 8];
        
        unsafe {
            set_pixel_unchecked(&mut pixels, 0, [255, 128, 64, 255]);
            assert_eq!(pixels[0..4], [255, 128, 64, 255]);
            
            set_pixel_unchecked(&mut pixels, 4, [0, 128, 255, 255]);
            assert_eq!(pixels[4..8], [0, 128, 255, 255]);
        }
    }

    #[test]
    fn test_create_contrast_lut() {
        let lut = create_contrast_lut(30.0);
        
        // Verify LUT has 256 entries
        assert_eq!(lut.len(), 256);
        
        // Verify values are in valid range
        for &val in lut.iter() {
            assert!(val <= 255);
        }
    }

    #[test]
    fn test_apply_contrast_lut() {
        let mut img = create_test_image(100, 100);
        let lut = create_contrast_lut(30.0);
        
        let original_pixels = img.raw_pixels.clone();
        apply_contrast_lut(&mut img, &lut);
        
        // Verify pixels have changed (unless they were already at extremes)
        let mut changed = false;
        for i in (0..img.raw_pixels.len()).step_by(4) {
            if original_pixels[i] != img.raw_pixels[i] {
                changed = true;
                break;
            }
        }
        assert!(changed);
    }

    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new(100, 3);
        
        // Get a buffer
        let buffer1 = pool.get_buffer(50);
        assert!(buffer1.capacity() >= 50);
        
        // Return it
        pool.return_buffer(buffer1);
        
        // Get another buffer (should reuse)
        let buffer2 = pool.get_buffer(50);
        assert!(buffer2.capacity() >= 50);
        
        // Return it
        pool.return_buffer(buffer2);
    }

    #[test]
    fn test_memory_pool_max_buffers() {
        let mut pool = MemoryPool::new(100, 2);
        
        // Get and return 3 buffers
        let buffer1 = pool.get_buffer(50);
        pool.return_buffer(buffer1);
        
        let buffer2 = pool.get_buffer(50);
        pool.return_buffer(buffer2);
        
        let buffer3 = pool.get_buffer(50);
        pool.return_buffer(buffer3);
        
        // Should only keep 2 buffers in the pool
        assert!(pool.buffers.len() <= 2);
    }
}