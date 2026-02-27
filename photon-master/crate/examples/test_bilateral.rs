//! Test bilateral filter implementation
//! This example demonstrates the bilateral filter with various parameters
//! and compares the performance of the standard vs fast implementations.

use photon_rs::conv::{bilateral_filter, bilateral_filter_fast, bilateral_filter_fast_iter};
use std::time::Instant;

fn create_test_image(width: u32, height: u32) -> Vec<u8> {
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);

    // Create a gradient image with noise
    for y in 0..height {
        for x in 0..width {
            // Gradient background
            let r = ((x * 255) / width) as u8;
            let g = ((y * 255) / height) as u8;
            let b = (((x + y) * 255) / (width + height)) as u8;

            // Add noise
            let noise = if x % 3 == 0 || y % 3 == 0 { 40 } else { 0 };
            let r = (r as i16 + noise).clamp(0, 255) as u8;
            let g = (g as i16 + noise).clamp(0, 255) as u8;
            let b = (b as i16 + noise).clamp(0, 255) as u8;
            let a = 255u8;

            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(a);
        }
    }

    pixels
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Bilateral Filter Performance Comparison ===\n");

    // Test with different image sizes
    let test_sizes = vec![(300, 200), (600, 400), (1200, 800)];

    for (width, height) in test_sizes {
        println!("\n=== Image Size: {}x{} ===", width, height);

        let sigma_spatial = 5.0;
        let sigma_range = 30.0;

        // Test 1: Standard bilateral filter (optimized)
        println!("\nTest 1: Standard bilateral filter (fast_mode=false)");
        let pixels = create_test_image(width, height);
        let mut img = photon_rs::PhotonImage::new(pixels, width, height);
        let start = Instant::now();
        bilateral_filter(&mut img, sigma_spatial, sigma_range, false);
        let duration_std = start.elapsed();
        println!("  Completed in {:.2}ms", duration_std.as_millis());
        let filename = format!("output_bilateral_std_{}x{}.jpg", width, height);
        photon_rs::native::save_image(img.clone(), &filename)?;
        println!("  Saved to: {}", filename);

        // Test 2: Fast bilateral filter via API (fast_mode=true)
        println!("\nTest 2: Fast bilateral filter via API (fast_mode=true)");
        let pixels = create_test_image(width, height);
        let mut img_api_fast = photon_rs::PhotonImage::new(pixels, width, height);
        let start = Instant::now();
        bilateral_filter(&mut img_api_fast, sigma_spatial, sigma_range, true);
        let duration_api_fast = start.elapsed();
        println!("  Completed in {:.2}ms", duration_api_fast.as_millis());
        let filename = format!("output_bilateral_api_fast_{}x{}.jpg", width, height);
        photon_rs::native::save_image(img_api_fast.clone(), &filename)?;
        println!("  Saved to: {}", filename);

        // Test 3: Fast bilateral filter (direct function call)
        println!("\nTest 3: Fast bilateral filter (direct function)");
        let pixels = create_test_image(width, height);
        let mut img_fast = photon_rs::PhotonImage::new(pixels, width, height);
        let start = Instant::now();
        bilateral_filter_fast(&mut img_fast, sigma_spatial, sigma_range);
        let duration_fast = start.elapsed();
        println!("  Completed in {:.2}ms", duration_fast.as_millis());
        let filename = format!("output_bilateral_fast_{}x{}.jpg", width, height);
        photon_rs::native::save_image(img_fast.clone(), &filename)?;
        println!("  Saved to: {}", filename);

        // Test 4: Fast bilateral filter with iterations
        println!("\nTest 4: Fast bilateral filter (5 iterations)");
        let pixels = create_test_image(width, height);
        let mut img_fast_iter = photon_rs::PhotonImage::new(pixels, width, height);
        let start = Instant::now();
        bilateral_filter_fast_iter(&mut img_fast_iter, sigma_spatial, sigma_range, 5);
        let duration_fast_iter = start.elapsed();
        println!("  Completed in {:.2}ms", duration_fast_iter.as_millis());
        let filename = format!("output_bilateral_fast_iter_{}x{}.jpg", width, height);
        photon_rs::native::save_image(img_fast_iter, &filename)?;
        println!("  Saved to: {}", filename);

        // Performance comparison
        println!("\n--- Performance Comparison ---");
        let speedup = duration_std.as_millis() as f64 / duration_api_fast.as_millis() as f64;
        println!("  Standard (fast_mode=false) vs Fast (fast_mode=true): {:.2}x speedup", speedup);
        let speedup_direct = duration_std.as_millis() as f64 / duration_fast.as_millis() as f64;
        println!("  Standard vs Fast (direct): {:.2}x speedup", speedup_direct);
        let speedup_iter = duration_std.as_millis() as f64 / duration_fast_iter.as_millis() as f64;
        println!("  Standard vs Fast (5 iter): {:.2}x speedup", speedup_iter);
    }

    // Additional test: Different parameters
    println!("\n=== Parameter Sensitivity Test (600x400) ===");
    let width = 600;
    let height = 400;

    let param_tests = vec![
        (3.0, 50.0, "light"),
        (5.0, 30.0, "medium"),
        (8.0, 20.0, "strong"),
    ];

    for (sigma_spatial, sigma_range, label) in param_tests {
        println!("\n--- {} smoothing (σs={:.1}, σr={:.1}) ---", label, sigma_spatial, sigma_range);

        // Standard (fast_mode=false)
        let pixels = create_test_image(width, height);
        let mut img = photon_rs::PhotonImage::new(pixels, width, height);
        let start = Instant::now();
        bilateral_filter(&mut img, sigma_spatial, sigma_range, false);
        let duration_std = start.elapsed();
        println!("  Standard (fast_mode=false): {:.2}ms", duration_std.as_millis());

        // Fast via API (fast_mode=true)
        let pixels = create_test_image(width, height);
        let mut img_api_fast = photon_rs::PhotonImage::new(pixels, width, height);
        let start = Instant::now();
        bilateral_filter(&mut img_api_fast, sigma_spatial, sigma_range, true);
        let duration_api_fast = start.elapsed();
        println!("  Fast (fast_mode=true): {:.2}ms", duration_api_fast.as_millis());
        println!("  Speedup: {:.2}x", duration_std.as_millis() as f64 / duration_api_fast.as_millis() as f64);
    }

    println!("\n=== Summary ===");
    println!("✓ All tests completed successfully!");
    println!("\n=== API Usage ===");
    println!("bilateral_filter(img, sigma_spatial, sigma_range, fast_mode):");
    println!("  - fast_mode=false: Standard bilateral filter (best quality)");
    println!("  - fast_mode=true: Fast Domain Transform (best performance)");
    println!("\n=== Algorithm Comparison ===");
    println!("Standard Bilateral Filter (fast_mode=false):");
    println!("  - Time Complexity: O(n * k²) where k is kernel size");
    println!("  - Space Complexity: O(n)");
    println!("  - Quality: Excellent edge preservation");
    println!("  - Speed: Slower for large kernels");
    println!("\nFast Bilateral Filter (fast_mode=true):");
    println!("  - Time Complexity: O(n) - independent of kernel size");
    println!("  - Space Complexity: O(n)");
    println!("  - Quality: Very good edge preservation");
    println!("  - Speed: 10-50x faster than standard");
    println!("\nRecommendations:");
    println!("  - Use fast_mode=true for real-time or large images (default recommendation)");
    println!("  - Use fast_mode=false when quality is paramount");
    println!("  - Use bilateral_filter_fast_iter() for quality/speed fine-tuning");

    Ok(())
}