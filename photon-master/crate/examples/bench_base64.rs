extern crate image;
extern crate photon_rs;

use instant::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "crate/examples/input_images/daisies_fuji.jpg";
    println!("file name = {}", file_name);

    // Open the image
    let img = photon_rs::native::open_image(file_name)?;
    
    // Benchmark base64 encoding
    println!("Benchmarking base64 encoding...");
    let start = Instant::now();
    let base64_data = img.get_base64();
    let encode_time = start.elapsed();
    println!("Encoding took {} ms", encode_time.as_millis());
    println!("Base64 length: {} bytes", base64_data.len());
    
    // Benchmark base64 decoding
    println!("\nBenchmarking base64 decoding...");
    // Remove data URI prefix before decoding
    let base64_clean = base64_data.strip_prefix("data:image/png;base64,").unwrap();
    let start = Instant::now();
    let img_decoded = photon_rs::base64_to_image(base64_clean);
    let decode_time = start.elapsed();
    println!("Decoding took {} ms", decode_time.as_millis());
    
    // Verify correctness
    assert_eq!(img.get_width(), img_decoded.get_width());
    assert_eq!(img.get_height(), img_decoded.get_height());
    println!("\n✓ Verification passed: decoded image matches original dimensions");
    
    // Total time
    println!("\nTotal base64 (encode + decode) time: {} ms", 
             (encode_time + decode_time).as_millis());

    Ok(())
}