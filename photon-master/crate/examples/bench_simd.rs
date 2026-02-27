extern crate image;
extern crate photon_rs;

use instant::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "crate/examples/input_images/daisies_fuji.jpg";
    println!("file name = {}", file_name);

    // Open the image
    let img = photon_rs::native::open_image(file_name)?;
    println!("Image size: {}x{}", img.get_width(), img.get_height());

    // Test 1: Alter channel (scalar vs SIMD)
    println!("\n=== Testing alter_channel ===");
    let mut img1 = photon_rs::native::open_image(file_name)?;
    let mut img2 = photon_rs::native::open_image(file_name)?;

    let start = Instant::now();
    photon_rs::channels::alter_channel(&mut img1, 0, 50);
    let scalar_time = start.elapsed();
    println!("Scalar version: {} ms", scalar_time.as_millis());

    let start = Instant::now();
    photon_rs::simd::alter_channel_simd(&mut img2, 0, 50);
    let simd_time = start.elapsed();
    println!("SIMD version: {} ms", simd_time.as_millis());

    assert_eq!(img1.get_raw_pixels(), img2.get_raw_pixels(), "Results should match");
    let speedup = scalar_time.as_nanos() as f64 / simd_time.as_nanos() as f64;
    println!("Speedup: {:.2}x", speedup);

    // Test 2: Grayscale (scalar vs SIMD)
    println!("\n=== Testing grayscale ===");
    let mut img1 = photon_rs::native::open_image(file_name)?;
    let mut img2 = photon_rs::native::open_image(file_name)?;

    let start = Instant::now();
    photon_rs::monochrome::grayscale(&mut img1);
    let scalar_time = start.elapsed();
    println!("Scalar version: {} ms", scalar_time.as_millis());

    let start = Instant::now();
    photon_rs::simd::grayscale_simd(&mut img2);
    let simd_time = start.elapsed();
    println!("SIMD version: {} ms", simd_time.as_millis());

    assert_eq!(img1.get_raw_pixels(), img2.get_raw_pixels(), "Results should match");
    let speedup = scalar_time.as_nanos() as f64 / simd_time.as_nanos() as f64;
    println!("Speedup: {:.2}x", speedup);

    // Test 3: Brightness (scalar vs SIMD)
    println!("\n=== Testing adjust_brightness ===");
    let mut img1 = photon_rs::native::open_image(file_name)?;
    let mut img2 = photon_rs::native::open_image(file_name)?;

    let start = Instant::now();
    photon_rs::effects::adjust_brightness(&mut img1, 30);
    let scalar_time = start.elapsed();
    println!("Scalar version: {} ms", scalar_time.as_millis());

    let start = Instant::now();
    photon_rs::simd::adjust_brightness_simd(&mut img2, 30);
    let simd_time = start.elapsed();
    println!("SIMD version: {} ms", simd_time.as_millis());

    assert_eq!(img1.get_raw_pixels(), img2.get_raw_pixels(), "Results should match");
    let speedup = scalar_time.as_nanos() as f64 / simd_time.as_nanos() as f64;
    println!("Speedup: {:.2}x", speedup);

    // Test 4: Invert (scalar vs SIMD)
    println!("\n=== Testing invert ===");
    let mut img1 = photon_rs::native::open_image(file_name)?;
    let mut img2 = photon_rs::native::open_image(file_name)?;

    let start = Instant::now();
    photon_rs::channels::invert(&mut img1);
    let scalar_time = start.elapsed();
    println!("Scalar version: {} ms", scalar_time.as_millis());

    let start = Instant::now();
    photon_rs::simd::invert_simd(&mut img2);
    let simd_time = start.elapsed();
    println!("SIMD version: {} ms", simd_time.as_millis());

    assert_eq!(img1.get_raw_pixels(), img2.get_raw_pixels(), "Results should match");
    let speedup = scalar_time.as_nanos() as f64 / simd_time.as_nanos() as f64;
    println!("Speedup: {:.2}x", speedup);

    println!("\n=== Summary ===");
    println!("All tests passed! SIMD alignment optimizations are working correctly.");
    println!("Note: Actual SIMD speedup depends on CPU architecture and WASM SIMD support in browsers.");

    Ok(())
}