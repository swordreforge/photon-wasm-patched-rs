//! Color Picker API Test
//! This example demonstrates the color picker functionality.

use photon_rs::{PhotonImage, Color};
use std::time::Instant;

fn main() {
    println!("=== Color Picker API Test ===\n");

    // Create a test image with gradient
    let width = 300u32;
    let height = 200u32;
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);

    // Create a gradient image
    for y in 0..height {
        for x in 0..width {
            // Gradient: Red increases with x, Green increases with y, Blue varies
            let r = ((x * 255) / width) as u8;
            let g = ((y * 255) / height) as u8;
            let b = (((x + y) * 255) / (width + height)) as u8;
            let a = 255u8;

            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(a);
        }
    }

    let img = PhotonImage::new(pixels, width, height);
    println!("Created test image: {}x{}", width, height);

    // Test 1: Get pixel color
    println!("\n--- Test 1: Get Pixel Color ---");
    let test_points = vec![(0, 0), (100, 100), (200, 150), (299, 199)];

    for (x, y) in test_points {
        if let Some(color) = img.get_pixel_color(x, y) {
            println!("  Pixel at ({}, {}): RGB({}, {}, {}), Alpha={}", x, y, color.r, color.g, color.b, color.a);
        }
        if let Some(hex) = img.get_pixel_color_hex(x, y, false) {
            println!("  Hex (RGB): {}", hex);
        }
        if let Some(brightness) = img.get_pixel_brightness(x, y) {
            println!("  Brightness: {}", brightness);
        }
    }

    // Test 2: Get region average color
    println!("\n--- Test 2: Get Region Average Color ---");
    let regions = vec![
        (0, 0, 50, 50),
        (100, 100, 50, 50),
        (200, 150, 50, 50),
    ];

    for (x, y, w, h) in regions {
        if let Some(color) = img.get_region_average_color(x, y, w, h) {
            println!("  Region ({}, {}, {}x{}) avg: RGB({}, {}, {}), Alpha={}", x, y, w, h, color.r, color.g, color.b, color.a);
        }
        if let Some(brightness) = img.get_region_average_brightness(x, y, w, h) {
            println!("  Region avg brightness: {}", brightness);
        }
    }

    // Test 3: Get dominant color
    println!("\n--- Test 3: Get Dominant Color ---");
    let start = Instant::now();
    let color = img.get_dominant_color();
    let duration = start.elapsed();
    println!("  Dominant color: RGB({}, {}, {}), Alpha={}", color.r, color.g, color.b, color.a);
    println!("  Computed in: {:?}", duration);

    // Test 4: Get region dominant color
    println!("\n--- Test 4: Get Region Dominant Color ---");
    let regions = vec![
        (0, 0, 100, 100),
        (100, 50, 100, 100),
    ];

    for (x, y, w, h) in regions {
        if let Some(color) = img.get_region_dominant_color(x, y, w, h) {
            println!("  Region ({}, {}, {}x{}) dominant: RGB({}, {}, {}), Alpha={}", x, y, w, h, color.r, color.g, color.b, color.a);
        }
    }

    // Test 5: Get color palette
    println!("\n--- Test 5: Get Color Palette ---");
    let palette_sizes = vec![3, 5, 10];

    for num_colors in palette_sizes {
        let start = Instant::now();
        let palette = img.get_color_palette(num_colors);
        let duration = start.elapsed();

        println!("  Top {} colors (computed in {:?}):", num_colors, duration);
        for (i, color) in palette.iter().enumerate() {
            println!("    {}. RGB({}, {}, {}), Alpha={}", i + 1, color.r, color.g, color.b, color.a);
        }
    }

    // Test 6: Out of bounds handling
    println!("\n--- Test 6: Out of Bounds Handling ---");
    let out_of_bounds = vec![(300, 200), (400, 300), (-1, -1)];

    for (x, y) in out_of_bounds {
        let x = x as i32;
        let y = y as i32;

        if x >= 0 && y >= 0 {
            if let Some(color) = img.get_pixel_color(x as u32, y as u32) {
                println!("  Pixel at ({}, {}): {:?}", x, y, color);
            } else {
                println!("  Pixel at ({}, {}): Out of bounds", x, y);
            }
        } else {
            println!("  Pixel at ({}, {}): Invalid coordinates", x, y);
        }
    }

    // Test 7: Performance test for large image
    println!("\n--- Test 7: Performance Test ---");
    let large_width = 1920u32;
    let large_height = 1080u32;
    let mut large_pixels = Vec::with_capacity((large_width * large_height * 4) as usize);

    for y in 0..large_height {
        for x in 0..large_width {
            let r = ((x * 255) / large_width) as u8;
            let g = ((y * 255) / large_height) as u8;
            let b = (((x + y) * 255) / (large_width + large_height)) as u8;
            let a = 255u8;
            large_pixels.push(r);
            large_pixels.push(g);
            large_pixels.push(b);
            large_pixels.push(a);
        }
    }

    let large_img = PhotonImage::new(large_pixels, large_width, large_height);
    println!("  Created large image: {}x{}", large_width, large_height);

    // Test dominant color performance
    let start = Instant::now();
    let _dominant = large_img.get_dominant_color();
    let duration = start.elapsed();
    println!("  Dominant color: {:?}", duration);

    // Test color palette performance
    let start = Instant::now();
    let _palette = large_img.get_color_palette(10);
    let duration = start.elapsed();
    println!("  Color palette (10 colors): {:?}", duration);

    // Test region average color performance
    let start = Instant::now();
    let _avg = large_img.get_region_average_color(500, 300, 500, 300);
    let duration = start.elapsed();
    println!("  Region average color: {:?}", duration);

    println!("\n=== All Tests Completed ===");
    println!("\n=== API Summary ===");
    println!("Available color picker functions:");
    println!("  - get_pixel_color(x, y) -> Option<(R, G, B, A)>");
    println!("  - get_pixel_color_hex(x, y, include_alpha) -> Option<String>");
    println!("  - get_pixel_brightness(x, y) -> Option<u8>");
    println!("  - get_region_average_color(x, y, width, height) -> Option<(R, G, B, A)>");
    println!("  - get_region_average_brightness(x, y, width, height) -> Option<u8>");
    println!("  - get_region_dominant_color(x, y, width, height) -> Option<(R, G, B, A)>");
    println!("  - get_dominant_color() -> (R, G, B, A)");
    println!("  - get_color_palette(num_colors) -> Vec<(R, G, B, A)>");
}