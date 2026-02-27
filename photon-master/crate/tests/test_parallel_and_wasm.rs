//! Tests for parallel and WASM optimization modules.

use photon_rs::{native, parallel, PhotonImage};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen_test::*;

#[cfg(feature = "enable_wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

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
fn test_parallel_invert_produces_inverted_image() {
    let mut img = create_test_image(100, 100);
    let original = img.clone();
    
    parallel::invert_parallel(&mut img);
    
    // Verify inversion
    for i in (0..img.raw_pixels.len()).step_by(4) {
        assert_eq!(img.raw_pixels[i], 255 - original.raw_pixels[i]);
        assert_eq!(img.raw_pixels[i + 1], 255 - original.raw_pixels[i + 1]);
        assert_eq!(img.raw_pixels[i + 2], 255 - original.raw_pixels[i + 2]);
        assert_eq!(img.raw_pixels[i + 3], original.raw_pixels[i + 3]);
    }
}

#[test]
fn test_parallel_grayscale_produces_grayscale() {
    let mut img = create_test_image(100, 100);
    
    parallel::grayscale_parallel(&mut img);
    
    // Verify grayscale
    for i in (0..img.raw_pixels.len()).step_by(4) {
        assert_eq!(img.raw_pixels[i], img.raw_pixels[i + 1]);
        assert_eq!(img.raw_pixels[i + 1], img.raw_pixels[i + 2]);
        assert_eq!(img.raw_pixels[i + 3], 255);
    }
}

#[test]
fn test_parallel_brightness_increase() {
    let mut img = create_test_image(100, 100);
    
    parallel::adjust_brightness_parallel(&mut img, 30);
    
    // Verify brightness increase
    for i in (0..img.raw_pixels.len()).step_by(4) {
        // All RGB channels should be increased or saturated
        assert!(
            img.raw_pixels[i] >= 30 || img.raw_pixels[i] == 255,
            "Red channel not increased properly at index {}",
            i
        );
        assert!(
            img.raw_pixels[i + 1] >= 30 || img.raw_pixels[i + 1] == 255,
            "Green channel not increased properly at index {}",
            i
        );
        assert!(
            img.raw_pixels[i + 2] >= 30 || img.raw_pixels[i + 2] == 255,
            "Blue channel not increased properly at index {}",
            i
        );
    }
}

#[test]
fn test_parallel_brightness_decrease() {
    let mut img = create_test_image(100, 100);
    
    parallel::adjust_brightness_parallel(&mut img, -30);
    
    // Verify brightness decrease
    for i in (0..img.raw_pixels.len()).step_by(4) {
        // All RGB channels should be decreased or 0
        assert!(
            img.raw_pixels[i] <= 225 || img.raw_pixels[i] == 0,
            "Red channel not decreased properly at index {}",
            i
        );
    }
}

#[test]
fn test_parallel_contrast() {
    let mut img1 = create_test_image(100, 100);
    let mut img2 = create_test_image(100, 100);
    
    parallel::adjust_contrast_parallel(&mut img1, 30.0);
    parallel::adjust_contrast_parallel(&mut img2, -30.0);
    
    // Verify contrast adjustment changed the image
    assert_ne!(img1.raw_pixels, img2.raw_pixels);
}

#[test]
fn test_parallel_threshold() {
    let mut img = create_test_image(100, 100);
    
    parallel::threshold_parallel(&mut img, 128);
    
    // Verify thresholding
    for i in (0..img.raw_pixels.len()).step_by(4) {
        let val = img.raw_pixels[i];
        assert!(val == 0 || val == 255, "Pixel at index {} not thresholded properly", i);
        assert_eq!(img.raw_pixels[i], img.raw_pixels[i + 1]);
        assert_eq!(img.raw_pixels[i + 1], img.raw_pixels[i + 2]);
    }
}

#[test]
fn test_parallel_threshold_low() {
    let mut img = create_test_image(100, 100);
    
    parallel::threshold_parallel(&mut img, 10);
    
    // Most pixels should be white (255)
    let white_count = img.raw_pixels.iter()
        .step_by(4)
        .filter(|&&x| x == 255)
        .count();
    
    assert!(white_count > (img.width * img.height) as usize * 8 / 10);
}

#[test]
fn test_parallel_threshold_high() {
    let mut img = create_test_image(100, 100);
    
    parallel::threshold_parallel(&mut img, 240);
    
    // Most pixels should be black (0)
    let black_count = img.raw_pixels.iter()
        .step_by(4)
        .filter(|&&x| x == 0)
        .count();
    
    assert!(black_count > (img.width * img.height) as usize * 8 / 10);
}

#[test]
fn test_parallel_noise_addition() {
    let mut img1 = create_test_image(100, 100);
    let mut img2 = create_test_image(100, 100);
    
    parallel::add_noise_rand_parallel(&mut img1, 0.0); // No noise
    parallel::add_noise_rand_parallel(&mut img2, 5.0); // Add noise
    
    // Verify that noise was added
    assert_ne!(img1.raw_pixels, img2.raw_pixels);
}

#[test]
fn test_parallel_noise_strength_zero() {
    let mut img = create_test_image(100, 100);
    let original = img.clone();
    
    parallel::add_noise_rand_parallel(&mut img, 0.0);
    
    // No change expected
    assert_eq!(img.raw_pixels, original.raw_pixels);
}

#[test]
fn test_parallel_small_image_uses_sequential() {
    // Small images should use sequential processing internally
    let mut img = create_test_image(10, 10);
    
    // This should not crash
    parallel::invert_parallel(&mut img);
    parallel::grayscale_parallel(&mut img);
    parallel::adjust_brightness_parallel(&mut img, 10);
}

#[test]
fn test_parallel_large_image_uses_parallel() {
    // Large images should benefit from parallel processing
    let mut img = create_test_image(500, 500);
    
    parallel::invert_parallel(&mut img);
    parallel::grayscale_parallel(&mut img);
    parallel::adjust_brightness_parallel(&mut img, 20);
    
    // Just verify it runs without error
    assert_eq!(img.raw_pixels.len(), (500 * 500 * 4) as usize);
}

#[test]
fn test_parallel_multiple_operations() {
    let mut img = create_test_image(200, 200);
    
    // Chain multiple operations
    parallel::grayscale_parallel(&mut img);
    parallel::adjust_brightness_parallel(&mut img, 30);
    parallel::adjust_contrast_parallel(&mut img, 20.0);
    parallel::threshold_parallel(&mut img, 100);
    
    // Verify final result is thresholded
    for i in (0..img.raw_pixels.len()).step_by(4) {
        let val = img.raw_pixels[i];
        assert!(val == 0 || val == 255);
    }
}

#[test]
fn test_parallel_preserves_image_dimensions() {
    let mut img = create_test_image(150, 200);
    let original_width = img.width;
    let original_height = img.height;
    
    parallel::invert_parallel(&mut img);
    parallel::grayscale_parallel(&mut img);
    parallel::adjust_brightness_parallel(&mut img, 15);
    parallel::adjust_contrast_parallel(&mut img, 10.0);
    
    assert_eq!(img.width, original_width);
    assert_eq!(img.height, original_height);
}

#[cfg(feature = "enable_wasm")]
#[wasm_bindgen_test]
async fn test_wasm_parallel_invert() {
    let mut img = create_test_image(100, 100);
    let original = img.clone();
    
    parallel::invert_parallel(&mut img);
    
    // Verify inversion
    for i in (0..img.raw_pixels.len()).step_by(4) {
        assert_eq!(img.raw_pixels[i], 255 - original.raw_pixels[i]);
    }
}

#[cfg(feature = "enable_wasm")]
#[wasm_bindgen_test]
async fn test_wasm_parallel_grayscale() {
    let mut img = create_test_image(100, 100);
    
    parallel::grayscale_parallel(&mut img);
    
    // Verify grayscale
    for i in (0..img.raw_pixels.len()).step_by(4) {
        assert_eq!(img.raw_pixels[i], img.raw_pixels[i + 1]);
        assert_eq!(img.raw_pixels[i + 1], img.raw_pixels[i + 2]);
    }
}

#[test]
fn test_wasm_optimizations_get_pixel_unchecked() {
    use photon_rs::wasm_optimizations;
    
    let pixels = vec![255u8, 128, 64, 255, 0, 128, 255, 255];
    
    unsafe {
        let pixel = wasm_optimizations::get_pixel_unchecked(&pixels, 0);
        assert_eq!(pixel, [255, 128, 64, 255]);
        
        let pixel2 = wasm_optimizations::get_pixel_unchecked(&pixels, 4);
        assert_eq!(pixel2, [0, 128, 255, 255]);
    }
}

#[test]
fn test_wasm_optimizations_set_pixel_unchecked() {
    use photon_rs::wasm_optimizations;
    
    let mut pixels = vec![0u8; 8];
    
    unsafe {
        wasm_optimizations::set_pixel_unchecked(&mut pixels, 0, [255, 128, 64, 255]);
        assert_eq!(pixels[0..4], [255, 128, 64, 255]);
        
        wasm_optimizations::set_pixel_unchecked(&mut pixels, 4, [0, 128, 255, 255]);
        assert_eq!(pixels[4..8], [0, 128, 255, 255]);
    }
}

#[test]
fn test_wasm_optimizations_contrast_lut() {
    use photon_rs::wasm_optimizations;
    
    let lut = wasm_optimizations::create_contrast_lut(30.0);
    
    // Verify LUT has 256 entries
    assert_eq!(lut.len(), 256);
    
    // Verify values are in valid range
    for &val in lut.iter() {
        assert!(val <= 255);
    }
}

#[test]
fn test_wasm_optimizations_apply_contrast_lut() {
    use photon_rs::wasm_optimizations;
    
    let mut img = create_test_image(100, 100);
    let lut = wasm_optimizations::create_contrast_lut(30.0);
    
    let original_pixels = img.raw_pixels.clone();
    wasm_optimizations::apply_contrast_lut(&mut img, &lut);
    
    // Verify pixels have changed
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
fn test_wasm_optimizations_memory_pool() {
    use photon_rs::wasm_optimizations;
    
    let mut pool = wasm_optimizations::MemoryPool::new(100, 3);
    
    let buffer1 = pool.get_buffer(50);
    assert!(buffer1.capacity() >= 50);
    
    pool.return_buffer(buffer1);
    
    let buffer2 = pool.get_buffer(50);
    assert!(buffer2.capacity() >= 50);
    
    pool.return_buffer(buffer2);
}

#[test]
fn test_wasm_optimizations_memory_pool_max_buffers() {
    use photon_rs::wasm_optimizations;
    
    let mut pool = wasm_optimizations::MemoryPool::new(100, 2);
    
    let buffer1 = pool.get_buffer(50);
    pool.return_buffer(buffer1);
    
    let buffer2 = pool.get_buffer(50);
    pool.return_buffer(buffer2);
    
    let buffer3 = pool.get_buffer(50);
    pool.return_buffer(buffer3);
    
    // Should only keep 2 buffers in the pool
    assert!(pool.buffers.len() <= 2);
}

#[test]
fn test_wasm_optimizations_temp_buffer() {
    use photon_rs::wasm_optimizations;
    
    wasm_optimizations::init_memory_pool();
    
    let buffer = wasm_optimizations::get_temp_buffer(1024);
    assert!(buffer.capacity() >= 1024);
    
    wasm_optimizations::return_temp_buffer(buffer);
}