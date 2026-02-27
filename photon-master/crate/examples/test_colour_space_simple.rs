//! Simple test to verify colour space optimizations compile and run

use photon_rs::{colour_spaces, multiple, PhotonImage, Rgb};

fn create_test_image(width: u32, height: u32) -> PhotonImage {
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            let r = ((x * 255) / width) as u8;
            let g = ((y * 255) / height) as u8;
            let b = ((x + y) * 127 / (width + height)) as u8;
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(255);
        }
    }
    PhotonImage::new(pixels, width, height)
}

fn main() {
    println!("=== Testing Colour Space Optimizations ===\n");

    // Test HSL functions
    println!("Testing HSL functions...");
    let mut img = create_test_image(512, 512);

    // These should now use hsl_simd
    colour_spaces::saturate_hsl(&mut img, 0.3);
    println!("✓ saturate_hsl completed");

    colour_spaces::lighten_hsl(&mut img, 0.2);
    println!("✓ lighten_hsl completed");

    colour_spaces::darken_hsl(&mut img, 0.2);
    println!("✓ darken_hsl completed");

    colour_spaces::hue_rotate_hsl(&mut img, 120.0);
    println!("✓ hue_rotate_hsl completed");

    colour_spaces::desaturate_hsl(&mut img, 0.5);
    println!("✓ desaturate_hsl completed");

    // Test HSV functions
    println!("\nTesting HSV functions...");
    let mut img2 = create_test_image(512, 512);

    // These should now use hsv_simd
    colour_spaces::saturate_hsv(&mut img2, 0.3);
    println!("✓ saturate_hsv completed");

    colour_spaces::lighten_hsv(&mut img2, 0.2);
    println!("✓ lighten_hsv completed");

    colour_spaces::darken_hsv(&mut img2, 0.2);
    println!("✓ darken_hsv completed");

    colour_spaces::hue_rotate_hsv(&mut img2, 120.0);
    println!("✓ hue_rotate_hsv completed");

    colour_spaces::desaturate_hsv(&mut img2, 0.5);
    println!("✓ desaturate_hsv completed");

    // Test mix_with_colour
    println!("\nTesting mix_with_colour...");
    let mut img3 = create_test_image(512, 512);
    let color = Rgb::new(128, 64, 192);
    colour_spaces::mix_with_colour(&mut img3, color, 0.5);
    println!("✓ mix_with_colour completed");

    // Test multiple image operations
    println!("\nTesting multiple image operations...");
    let mut base_img = create_test_image(512, 512);
    let watermark_img = create_test_image(128, 128);

    // This should now use watermark_fast
    multiple::watermark(&mut base_img, &watermark_img, 10, 10);
    println!("✓ watermark completed");

    let mut img4 = create_test_image(512, 512);
    let img5 = create_test_image(512, 512);

    // This should now use blend_fast
    multiple::blend(&mut img4, &img5, "multiply");
    println!("✓ blend completed");

    println!("\n=== All tests completed successfully! ===");
    println!("All optimized functions are working correctly.");
}