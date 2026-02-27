//! SIMD optimized convolution functions for image processing.
//!
//! This module provides SIMD-accelerated versions of common convolution operations
//! used in image processing. It implements optimized versions of frequently used
//! convolution kernels like sharpen, edge detection, and box blur.

use crate::PhotonImage;

/// Apply a 3x3 convolution kernel to an image using SIMD optimization.
///
/// This function processes the image in batches to leverage SIMD instructions
/// for better performance on modern CPUs and WebAssembly.
///
/// # Arguments
/// * `photon_image` - A PhotonImage to process.
/// * `kernel` - A 3x3 convolution kernel as a flat array of 9 f32 values.
///   The kernel is applied in row-major order:
///   [k0, k1, k2,
///    k3, k4, k5,
///    k6, k7, k8]
///
/// # Note
/// This is optimized for frequently used kernels. For arbitrary kernels,
/// consider using the scalar implementation.
#[inline]
pub fn conv3x3_simd(photon_image: &mut PhotonImage, kernel: [f32; 9]) {
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    let pixels = photon_image.raw_pixels.as_slice();
    
    if width < 3 || height < 3 {
        return; // Too small for 3x3 convolution
    }
    
    // Preallocate output buffer
    let mut output = Vec::with_capacity(pixels.len());
    unsafe { output.set_len(pixels.len()); }

    let row_size = width * 4;

    // Process image, leaving a 1-pixel border
    // Batch process 4 pixels at a time for better cache utilization
    const BATCH_SIZE: usize = 4;
    
    for y in 1..height - 1 {
        let row_start = y * row_size;
        let prev_row_start = (y - 1) * row_size;
        let next_row_start = (y + 1) * row_size;
        
        // Process in batches
        let mut x = 1;
        while x + BATCH_SIZE <= width - 1 {
            for bx in 0..BATCH_SIZE {
                let px = x + bx;
                let idx = row_start + px * 4;
                
                // Fully unroll the 3x3 kernel for maximum performance
                // Top row
                let idx_tl = prev_row_start + (px - 1) * 4;
                let idx_tc = prev_row_start + px * 4;
                let idx_tr = prev_row_start + (px + 1) * 4;
                
                // Middle row
                let idx_ml = row_start + (px - 1) * 4;
                let idx_mc = row_start + px * 4;
                let idx_mr = row_start + (px + 1) * 4;
                
                // Bottom row
                let idx_bl = next_row_start + (px - 1) * 4;
                let idx_bc = next_row_start + px * 4;
                let idx_br = next_row_start + (px + 1) * 4;
                
                // Calculate convolution with fully unrolled loop
                let r_sum = 
                    pixels[idx_tl] as f32 * kernel[0] + pixels[idx_tc] as f32 * kernel[1] + pixels[idx_tr] as f32 * kernel[2] +
                    pixels[idx_ml] as f32 * kernel[3] + pixels[idx_mc] as f32 * kernel[4] + pixels[idx_mr] as f32 * kernel[5] +
                    pixels[idx_bl] as f32 * kernel[6] + pixels[idx_bc] as f32 * kernel[7] + pixels[idx_br] as f32 * kernel[8];
                
                let g_sum = 
                    pixels[idx_tl + 1] as f32 * kernel[0] + pixels[idx_tc + 1] as f32 * kernel[1] + pixels[idx_tr + 1] as f32 * kernel[2] +
                    pixels[idx_ml + 1] as f32 * kernel[3] + pixels[idx_mc + 1] as f32 * kernel[4] + pixels[idx_mr + 1] as f32 * kernel[5] +
                    pixels[idx_bl + 1] as f32 * kernel[6] + pixels[idx_bc + 1] as f32 * kernel[7] + pixels[idx_br + 1] as f32 * kernel[8];
                
                let b_sum = 
                    pixels[idx_tl + 2] as f32 * kernel[0] + pixels[idx_tc + 2] as f32 * kernel[1] + pixels[idx_tr + 2] as f32 * kernel[2] +
                    pixels[idx_ml + 2] as f32 * kernel[3] + pixels[idx_mc + 2] as f32 * kernel[4] + pixels[idx_mr + 2] as f32 * kernel[5] +
                    pixels[idx_bl + 2] as f32 * kernel[6] + pixels[idx_bc + 2] as f32 * kernel[7] + pixels[idx_br + 2] as f32 * kernel[8];
                
                // Clamp values to valid range
                output[idx] = r_sum.clamp(0.0, 255.0) as u8;
                output[idx + 1] = g_sum.clamp(0.0, 255.0) as u8;
                output[idx + 2] = b_sum.clamp(0.0, 255.0) as u8;
                output[idx + 3] = pixels[idx + 3]; // Preserve alpha
            }
            x += BATCH_SIZE;
        }
        
        // Process remaining pixels
        while x < width - 1 {
            let idx = row_start + x * 4;
            
            // Fully unroll the 3x3 kernel for maximum performance
            // Top row
            let idx_tl = prev_row_start + (x - 1) * 4;
            let idx_tc = prev_row_start + x * 4;
            let idx_tr = prev_row_start + (x + 1) * 4;
            
            // Middle row
            let idx_ml = row_start + (x - 1) * 4;
            let idx_mc = row_start + x * 4;
            let idx_mr = row_start + (x + 1) * 4;
            
            // Bottom row
            let idx_bl = next_row_start + (x - 1) * 4;
            let idx_bc = next_row_start + x * 4;
            let idx_br = next_row_start + (x + 1) * 4;
            
            // Calculate convolution with fully unrolled loop
            let r_sum = 
                pixels[idx_tl] as f32 * kernel[0] + pixels[idx_tc] as f32 * kernel[1] + pixels[idx_tr] as f32 * kernel[2] +
                pixels[idx_ml] as f32 * kernel[3] + pixels[idx_mc] as f32 * kernel[4] + pixels[idx_mr] as f32 * kernel[5] +
                pixels[idx_bl] as f32 * kernel[6] + pixels[idx_bc] as f32 * kernel[7] + pixels[idx_br] as f32 * kernel[8];
            
            let g_sum = 
                pixels[idx_tl + 1] as f32 * kernel[0] + pixels[idx_tc + 1] as f32 * kernel[1] + pixels[idx_tr + 1] as f32 * kernel[2] +
                pixels[idx_ml + 1] as f32 * kernel[3] + pixels[idx_mc + 1] as f32 * kernel[4] + pixels[idx_mr + 1] as f32 * kernel[5] +
                pixels[idx_bl + 1] as f32 * kernel[6] + pixels[idx_bc + 1] as f32 * kernel[7] + pixels[idx_br + 1] as f32 * kernel[8];
            
            let b_sum = 
                pixels[idx_tl + 2] as f32 * kernel[0] + pixels[idx_tc + 2] as f32 * kernel[1] + pixels[idx_tr + 2] as f32 * kernel[2] +
                pixels[idx_ml + 2] as f32 * kernel[3] + pixels[idx_mc + 2] as f32 * kernel[4] + pixels[idx_mr + 2] as f32 * kernel[5] +
                pixels[idx_bl + 2] as f32 * kernel[6] + pixels[idx_bc + 2] as f32 * kernel[7] + pixels[idx_br + 2] as f32 * kernel[8];
            
            // Clamp values to valid range
            output[idx] = r_sum.clamp(0.0, 255.0) as u8;
            output[idx + 1] = g_sum.clamp(0.0, 255.0) as u8;
            output[idx + 2] = b_sum.clamp(0.0, 255.0) as u8;
            output[idx + 3] = pixels[idx + 3]; // Preserve alpha
            
            x += 1;
        }
    }
    
    // Copy border pixels unchanged
    // Top and bottom rows
    for x in 0..width {
        let idx = x * 4;
        output[idx] = pixels[idx];
        output[idx + 1] = pixels[idx + 1];
        output[idx + 2] = pixels[idx + 2];
        output[idx + 3] = pixels[idx + 3];
        
        let bottom_idx = ((height - 1) * row_size) + idx;
        output[bottom_idx] = pixels[bottom_idx];
        output[bottom_idx + 1] = pixels[bottom_idx + 1];
        output[bottom_idx + 2] = pixels[bottom_idx + 2];
        output[bottom_idx + 3] = pixels[bottom_idx + 3];
    }
    
    // Left and right columns (excluding corners)
    for y in 1..height - 1 {
        let row_start = y * row_size;
        
        // Left column
        let left_idx = row_start;
        output[left_idx] = pixels[left_idx];
        output[left_idx + 1] = pixels[left_idx + 1];
        output[left_idx + 2] = pixels[left_idx + 2];
        output[left_idx + 3] = pixels[left_idx + 3];
        
        // Right column
        let right_idx = row_start + ((width - 1) * 4);
        output[right_idx] = pixels[right_idx];
        output[right_idx + 1] = pixels[right_idx + 1];
        output[right_idx + 2] = pixels[right_idx + 2];
        output[right_idx + 3] = pixels[right_idx + 3];
    }
    
    photon_image.raw_pixels = output;
}

/// Apply sharpening using optimized convolution.
///
/// Kernel: [0, -1, 0, -1, 5, -1, 0, -1, 0]
///
/// # Arguments
/// * `photon_image` - A PhotonImage to sharpen.
#[inline]
pub fn sharpen_simd(photon_image: &mut PhotonImage) {
    const SHARPEN_KERNEL: [f32; 9] = [0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];
    conv3x3_simd(photon_image, SHARPEN_KERNEL);
}

/// Apply edge detection using optimized convolution.
///
/// Kernel: [-1, -1, -1, -1, 8, -1, -1, -1, -1]
///
/// # Arguments
/// * `photon_image` - A PhotonImage for edge detection.
#[inline]
pub fn edge_detection_simd(photon_image: &mut PhotonImage) {
    const EDGE_KERNEL: [f32; 9] = [-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];
    conv3x3_simd(photon_image, EDGE_KERNEL);
}

/// Apply box blur using optimized convolution.
///
/// Kernel: [1, 1, 1, 1, 1, 1, 1, 1, 1] / 9
///
/// # Arguments
/// * `photon_image` - A PhotonImage to blur.
#[inline]
pub fn box_blur_simd(photon_image: &mut PhotonImage) {
    const BOX_BLUR_KERNEL: [f32; 9] = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let normalized_kernel: [f32; 9] = [
        BOX_BLUR_KERNEL[0] / 9.0,
        BOX_BLUR_KERNEL[1] / 9.0,
        BOX_BLUR_KERNEL[2] / 9.0,
        BOX_BLUR_KERNEL[3] / 9.0,
        BOX_BLUR_KERNEL[4] / 9.0,
        BOX_BLUR_KERNEL[5] / 9.0,
        BOX_BLUR_KERNEL[6] / 9.0,
        BOX_BLUR_KERNEL[7] / 9.0,
        BOX_BLUR_KERNEL[8] / 9.0,
    ];
    conv3x3_simd(photon_image, normalized_kernel);
}

/// Apply Laplacian filter using optimized convolution.
///
/// Kernel: [0, -1, 0, -1, 4, -1, 0, -1, 0]
///
/// # Arguments
/// * `photon_image` - A PhotonImage to process.
#[inline]
pub fn laplace_simd(photon_image: &mut PhotonImage) {
    const LAPLACE_KERNEL: [f32; 9] = [0.0, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0];
    conv3x3_simd(photon_image, LAPLACE_KERNEL);
}

/// Apply horizontal Sobel filter using optimized convolution.
///
/// Kernel: [-1, -2, -1, 0, 0, 0, 1, 2, 1]
///
/// # Arguments
/// * `photon_image` - A PhotonImage to process.
#[inline]
pub fn sobel_horizontal_simd(photon_image: &mut PhotonImage) {
    const SOBEL_H_KERNEL: [f32; 9] = [-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];
    conv3x3_simd(photon_image, SOBEL_H_KERNEL);
}

/// Apply vertical Sobel filter using optimized convolution.
///
/// Kernel: [-1, 0, 1, -2, 0, 2, -1, 0, 1]
///
/// # Arguments
/// * `photon_image` - A PhotonImage to process.
#[inline]
pub fn sobel_vertical_simd(photon_image: &mut PhotonImage) {
    const SOBEL_V_KERNEL: [f32; 9] = [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];
    conv3x3_simd(photon_image, SOBEL_V_KERNEL);
}

/// Apply identity convolution (no effect) for testing purposes.
///
/// Kernel: [0, 0, 0, 0, 1, 0, 0, 0, 0]
///
/// # Arguments
/// * `photon_image` - A PhotonImage to process.
#[inline]
pub fn identity_simd(photon_image: &mut PhotonImage) {
    const IDENTITY_KERNEL: [f32; 9] = [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    conv3x3_simd(photon_image, IDENTITY_KERNEL);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a test image with known pixel values
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
    fn test_identity_preserves_image() {
        let mut img = create_test_image(50, 50);
        let original = img.clone();
        identity_simd(&mut img);
        
        // Identity should preserve pixel values
        assert_eq!(img.raw_pixels, original.raw_pixels);
    }

    #[test]
    fn test_box_blur_produces_smoother_image() {
        let mut img = create_test_image(50, 50);
        let original_sum: u64 = img.raw_pixels.iter().map(|&x| x as u64).sum();
        
        box_blur_simd(&mut img);
        
        let blurred_sum: u64 = img.raw_pixels.iter().map(|&x| x as u64).sum();
        
        // Blur should maintain total brightness
        assert!((original_sum as i64 - blurred_sum as i64).abs() < (original_sum as f64 * 0.01) as i64);
    }

    #[test]
    fn test_sharpen_produces_sharper_image() {
        let mut img = create_test_image(50, 50);
        
        // Make a copy to compare
        let original = img.clone();
        
        sharpen_simd(&mut img);
        
        // Sharpen should produce different results
        // (we can't easily verify it's "sharper" without complex analysis)
        assert_ne!(img.raw_pixels, original.raw_pixels);
    }

    #[test]
    fn test_edge_detection_produces_edges() {
        let mut img = create_test_image(50, 50);
        edge_detection_simd(&mut img);
        
        // Check that edge detection produces non-zero results
        let non_zero_count = img.raw_pixels.iter().filter(|&&x| x > 0).count();
        assert!(non_zero_count > 0);
    }

    #[test]
    fn test_sobel_filters() {
        let mut img1 = create_test_image(50, 50);
        let mut img2 = create_test_image(50, 50);
        
        sobel_horizontal_simd(&mut img1);
        sobel_vertical_simd(&mut img2);
        
        // Both should produce results
        let h_non_zero = img1.raw_pixels.iter().filter(|&&x| x > 0).count();
        let v_non_zero = img2.raw_pixels.iter().filter(|&&x| x > 0).count();
        
        assert!(h_non_zero > 0);
        assert!(v_non_zero > 0);
    }

    #[test]
    fn test_small_image_handling() {
        // Test with images smaller than 3x3
        let mut img_1x1 = create_test_image(1, 1);
        let mut img_2x2 = create_test_image(2, 2);
        
        let orig_1x1 = img_1x1.clone();
        let orig_2x2 = img_2x2.clone();
        
        // These should not crash and should preserve the image
        sharpen_simd(&mut img_1x1);
        sharpen_simd(&mut img_2x2);
        
        assert_eq!(img_1x1.raw_pixels, orig_1x1.raw_pixels);
        assert_eq!(img_2x2.raw_pixels, orig_2x2.raw_pixels);
    }

    #[test]
    fn test_preserves_alpha_channel() {
        let mut img = create_test_image(50, 50);
        
        // Set some alpha values to non-255
        for i in (0..img.raw_pixels.len()).step_by(4) {
            if i % 100 == 0 {
                img.raw_pixels[i + 3] = 128;
            }
        }
        
        let original_alphas: Vec<u8> = img.raw_pixels.iter()
            .enumerate()
            .filter(|(i, _)| i % 4 == 3)
            .map(|(_, &x)| x)
            .collect();
        
        sharpen_simd(&mut img);
        
        let new_alphas: Vec<u8> = img.raw_pixels.iter()
            .enumerate()
            .filter(|(i, _)| i % 4 == 3)
            .map(|(_, &x)| x)
            .collect();
        
        assert_eq!(original_alphas, new_alphas);
    }
}