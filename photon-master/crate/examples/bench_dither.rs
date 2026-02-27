use photon_rs::effects::{dither, dither_ordered};
use photon_rs::PhotonImage;
use std::time::Instant;

fn main() {
    // Create a test image (1000x1000)
    let width = 1000;
    let height = 1000;
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);
    
    // Create a gradient pattern
    for y in 0..height {
        for x in 0..width {
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
    
    println!("Created test image: {}x{}", width, height);
    println!("Testing dithering algorithms...\n");
    
    // Test Floyd-Steinberg dithering
    let mut img1 = PhotonImage::new(pixels.clone(), width, height);
    let start = Instant::now();
    dither(&mut img1, 2);
    let duration1 = start.elapsed();
    println!("Floyd-Steinberg dithering: {:?}", duration1);
    
    // Test Ordered dithering
    let mut img2 = PhotonImage::new(pixels, width, height);
    let start = Instant::now();
    dither_ordered(&mut img2, 2);
    let duration2 = start.elapsed();
    println!("Ordered dithering: {:?}", duration2);
    
    // Calculate speedup
    let speedup = duration1.as_micros() as f64 / duration2.as_micros() as f64;
    println!("\nSpeedup: {:.2}x", speedup);
}