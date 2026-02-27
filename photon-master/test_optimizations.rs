//! Test file for performance optimizations

use photon_rs::conv::{apply_filter_pipeline, apply_convolution_pipeline, FilterOperation, gaussian_blur_tiled, gaussian_blur_fast};
use photon_rs::native::open_image;

fn main() {
    // Load a test image
    let mut img = open_image("crate/examples/input_images/demon.jpg")
        .expect("Failed to open test image");
    
    println!("Original image size: {}x{}", img.width, img.height);
    
    // Test 1: Filter pipeline with multiple operations
    println!("\n=== Test 1: Filter Pipeline ===");
    let mut img1 = img.clone();
    let start = std::time::Instant::now();
    apply_filter_pipeline(&mut img1, &[
        FilterOperation::Brightness(10),
        FilterOperation::Contrast(20.0),
        FilterOperation::Grayscale,
    ]);
    let duration = start.elapsed();
    println!("Filter pipeline took: {:?}", duration);
    
    // Test 2: Convolution pipeline
    println!("\n=== Test 2: Convolution Pipeline ===");
    let mut img2 = img.clone();
    let start = std::time::Instant::now();
    apply_convolution_pipeline(&mut img2, &[
        [0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0], // Sharpen
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],     // Box blur
    ]);
    let duration = start.elapsed();
    println!("Convolution pipeline took: {:?}", duration);
    
    // Test 3: Fast Gaussian blur
    println!("\n=== Test 3: Fast Gaussian Blur ===");
    let mut img3 = img.clone();
    let start = std::time::Instant::now();
    gaussian_blur_fast(&mut img3, 5);
    let duration = start.elapsed();
    println!("Fast Gaussian blur (radius=5) took: {:?}", duration);
    
    // Test 4: Tiled Gaussian blur
    println!("\n=== Test 4: Tiled Gaussian Blur ===");
    let mut img4 = img.clone();
    let start = std::time::Instant::now();
    gaussian_blur_tiled(&mut img4, 5, 256);
    let duration = start.elapsed();
    println!("Tiled Gaussian blur (radius=5, tile_size=256) took: {:?}", duration);
    
    // Test 5: Compare tiled vs non-tiled for large blur
    println!("\n=== Test 5: Large Blur Comparison ===");
    let mut img5 = img.clone();
    let mut img6 = img.clone();
    
    let start = std::time::Instant::now();
    gaussian_blur_fast(&mut img5, 10);
    let duration_fast = start.elapsed();
    
    let start = std::time::Instant::now();
    gaussian_blur_tiled(&mut img6, 10, 256);
    let duration_tiled = start.elapsed();
    
    println!("Fast Gaussian blur (radius=10) took: {:?}", duration_fast);
    println!("Tiled Gaussian blur (radius=10) took: {:?}", duration_tiled);
    println!("Speedup: {:.2}x", duration_fast.as_secs_f64() / duration_tiled.as_secs_f64());
    
    println!("\n=== All tests completed successfully! ===");
}