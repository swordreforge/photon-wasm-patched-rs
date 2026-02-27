// 测试noise模块的SIMD优化
use photon_rs::noise::{add_noise_rand, add_noise_rand_with_strength};
use photon_rs::native::open_image;
use photon_rs::PhotonImage;

fn main() {
    // 创建一个测试图像
    let mut img = PhotonImage::new(vec![255u8; 400], 10, 10);
    
    // 测试SIMD优化的add_noise_rand
    println!("Testing add_noise_rand (SIMD optimized)...");
    add_noise_rand(&mut img);
    println!("✓ add_noise_rand completed successfully");
    
    // 测试SIMD优化的add_noise_rand_with_strength
    println!("\nTesting add_noise_rand_with_strength (SIMD optimized)...");
    let mut img2 = PhotonImage::new(vec![255u8; 400], 10, 10);
    add_noise_rand_with_strength(&mut img2, 5.0);
    println!("✓ add_noise_rand_with_strength completed successfully");
    
    println!("\n✓ All noise optimization tests passed!");
}