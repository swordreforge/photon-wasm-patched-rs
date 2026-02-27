//! Test program to verify colour space optimizations

use photon_rs::{colour_spaces, multiple, native, PhotonImage, Rgb};
use std::time::Instant;

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

fn benchmark_hsl(name: &str, img: &mut PhotonImage, func: fn(&mut PhotonImage, &str, f32)) {
    let start = Instant::now();
    for _ in 0..10 {
        func(img, "saturate", 0.3);
    }
    let duration = start.elapsed();
    println!("{}: {:?}", name, duration);
}

fn benchmark_hsv(name: &str, img: &mut PhotonImage, func: fn(&mut PhotonImage, &str, f32)) {
    let start = Instant::now();
    for _ in 0..10 {
        func(img, "saturate", 0.3);
    }
    let duration = start.elapsed();
    println!("{}: {:?}", name, duration);
}

fn benchmark_mix(name: &str, img: &mut PhotonImage, func: fn(&mut PhotonImage, Rgb, f32)) {
    let color = Rgb::new(128, 64, 192);
    let start = Instant::now();
    for _ in 0..10 {
        func(img, color, 0.5);
    }
    let duration = start.elapsed();
    println!("{}: {:?}", name, duration);
}

fn main() {
    println!("=== Colour Space Optimization Benchmark ===\n");

    // Test with different image sizes
    let sizes = [(256, 256), (512, 512), (1024, 1024)];

    for (width, height) in sizes {
        println!("--- Image size: {}x{} ---", width, height);

        // HSL benchmarks
        println!("HSL benchmarks:");
        let mut img1 = create_test_image(width, height);
        benchmark_hsl("hsl (original)", &mut img1, colour_spaces::hsl);

        let mut img2 = create_test_image(width, height);
        benchmark_hsl("hsl_fast", &mut img2, colour_spaces::hsl_fast);

        let mut img3 = create_test_image(width, height);
        benchmark_hsl("hsl_simd (NEW)", &mut img3, colour_spaces::hsl_simd);

        // HSV benchmarks
        println!("\nHSV benchmarks:");
        let mut img4 = create_test_image(width, height);
        benchmark_hsv("hsv (original)", &mut img4, colour_spaces::hsv);

        let mut img5 = create_test_image(width, height);
        benchmark_hsv("hsv_fast", &mut img5, colour_spaces::hsv_fast);

        let mut img6 = create_test_image(width, height);
        benchmark_hsv("hsv_simd (NEW)", &mut img6, colour_spaces::hsv_simd);

        // Mix benchmarks
        println!("\nMix benchmarks:");
        let mut img7 = create_test_image(width, height);
        benchmark_mix("mix_with_colour (original)", &mut img7, colour_spaces::mix_with_colour);

        let mut img8 = create_test_image(width, height);
        benchmark_mix("mix_with_colour_simd (NEW)", &mut img8, colour_spaces::mix_with_colour_simd);

        println!();
    }

    // Test specific functions
    println!("=== Specific Function Tests ===\n");

    let mut img = create_test_image(512, 512);

    println!("Testing saturate_hsl (now uses hsl_simd)...");
    let start = Instant::now();
    for _ in 0..10 {
        colour_spaces::saturate_hsl(&mut img, 0.3);
    }
    println!("saturate_hsl: {:?}\n", start.elapsed());

    println!("Testing saturate_hsv (now uses hsv_simd)...");
    let start = Instant::now();
    for _ in 0..10 {
        colour_spaces::saturate_hsv(&mut img, 0.3);
    }
    println!("saturate_hsv: {:?}\n", start.elapsed());

    println!("Testing lighten_hsl (now uses hsl_simd)...");
    let start = Instant::now();
    for _ in 0..10 {
        colour_spaces::lighten_hsl(&mut img, 0.2);
    }
    println!("lighten_hsl: {:?}\n", start.elapsed());

    println!("Testing darken_hsl (now uses hsl_simd)...");
    let start = Instant::now();
    for _ in 0..10 {
        colour_spaces::darken_hsl(&mut img, 0.2);
    }
    println!("darken_hsl: {:?}\n", start.elapsed());

    println!("Testing hue_rotate_hsl (now uses hsl_simd)...");
    let start = Instant::now();
    for _ in 0..10 {
        colour_spaces::hue_rotate_hsl(&mut img, 120.0);
    }
    println!("hue_rotate_hsl: {:?}\n", start.elapsed());

    println!("Testing desaturate_hsl (now uses hsl_simd)...");
    let start = Instant::now();
    for _ in 0..10 {
        colour_spaces::desaturate_hsl(&mut img, 0.5);
    }
    println!("desaturate_hsl: {:?}\n", start.elapsed());

    // Multiple image operations
    println!("=== Multiple Image Operations ===\n");

    let mut base_img = create_test_image(512, 512);
    let watermark_img = create_test_image(128, 128);

    println!("Testing watermark (now uses watermark_fast)...");
    let start = Instant::now();
    for _ in 0..10 {
        multiple::watermark(&mut base_img, &watermark_img, 10, 10);
    }
    println!("watermark: {:?}\n", start.elapsed());

    let mut img1 = create_test_image(512, 512);
    let mut img2 = create_test_image(512, 512);

    println!("Testing blend (now uses blend_fast)...");
    let start = Instant::now();
    for _ in 0..10 {
        multiple::blend(&mut img1, &img2, "multiply");
    }
    println!("blend (multiply): {:?}\n", start.elapsed());

    println!("All tests completed successfully!");
}