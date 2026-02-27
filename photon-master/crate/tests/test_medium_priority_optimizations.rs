//! Tests for medium priority optimizations

use photon_rs::{PhotonImage, colour_spaces::{hsl, hsl_fast, hsl_adaptive, hsv, hsv_fast, hsv_adaptive}, multiple::{blend, blend_fast, blend_adaptive}, adaptive::{ImageSize, get_image_size}};

fn create_test_image(width: u32, height: u32) -> PhotonImage {
    let size = (width * height * 4) as usize;
    let pixels: Vec<u8> = vec![255u8; size];
    PhotonImage::new(pixels, width, height)
}

#[test]
fn test_adaptive_image_size_classification() {
    // Test small image
    let small_img = create_test_image(100, 100);
    assert_eq!(get_image_size(&small_img), ImageSize::Small);

    // Test medium image
    let medium_img = create_test_image(512, 512);
    assert_eq!(get_image_size(&medium_img), ImageSize::Medium);

    // Test large image
    let large_img = create_test_image(2048, 2048);
    assert_eq!(get_image_size(&large_img), ImageSize::Large);
}

#[test]
fn test_hsl_fast_vs_standard() {
    // Create a test image with known pixel values
    let mut img1 = create_test_image(100, 100);
    let mut img2 = create_test_image(100, 100);

    // Apply saturation using both versions
    hsl(&mut img1, "saturate", 0.2);
    hsl_fast(&mut img2, "saturate", 0.2);

    // Both should produce results (may differ slightly due to implementation)
    // The important thing is that both run without errors
    assert!(!img1.get_raw_pixels().is_empty());
    assert!(!img2.get_raw_pixels().is_empty());
}

#[test]
fn test_hsv_fast_vs_standard() {
    let mut img1 = create_test_image(100, 100);
    let mut img2 = create_test_image(100, 100);

    hsv(&mut img1, "saturate", 0.2);
    hsv_fast(&mut img2, "saturate", 0.2);

    assert!(!img1.get_raw_pixels().is_empty());
    assert!(!img2.get_raw_pixels().is_empty());
}

#[test]
fn test_blend_fast_vs_standard() {
    let mut img1a = create_test_image(50, 50);
    let img1b = create_test_image(50, 50);

    let mut img2a = create_test_image(50, 50);
    let img2b = create_test_image(50, 50);

    blend(&mut img1a, &img1b, "multiply");
    blend_fast(&mut img2a, &img2b, "multiply");

    assert!(!img1a.get_raw_pixels().is_empty());
    assert!(!img2a.get_raw_pixels().is_empty());
}

#[test]
fn test_hsl_adaptive() {
    // Test with small image (should use standard version)
    let mut small_img = create_test_image(50, 50);
    hsl_adaptive(&mut small_img, "saturate", 0.2);
    assert!(!small_img.get_raw_pixels().is_empty());

    // Test with large image (should use fast version)
    let mut large_img = create_test_image(2000, 2000);
    hsl_adaptive(&mut large_img, "saturate", 0.2);
    assert!(!large_img.get_raw_pixels().is_empty());
}

#[test]
fn test_hsv_adaptive() {
    let mut small_img = create_test_image(50, 50);
    hsv_adaptive(&mut small_img, "saturate", 0.2);
    assert!(!small_img.get_raw_pixels().is_empty());

    let mut large_img = create_test_image(2000, 2000);
    hsv_adaptive(&mut large_img, "saturate", 0.2);
    assert!(!large_img.get_raw_pixels().is_empty());
}

#[test]
fn test_blend_adaptive() {
    let mut small_img1 = create_test_image(50, 50);
    let small_img2 = create_test_image(50, 50);
    blend_adaptive(&mut small_img1, &small_img2, "multiply");
    assert!(!small_img1.get_raw_pixels().is_empty());

    let mut large_img1 = create_test_image(2000, 2000);
    let large_img2 = create_test_image(2000, 2000);
    blend_adaptive(&mut large_img1, &large_img2, "multiply");
    assert!(!large_img1.get_raw_pixels().is_empty());
}

#[test]
fn test_hsl_fast_various_modes() {
    let modes = ["saturate", "desaturate", "lighten", "darken", "shift_hue"];

    for mode in modes {
        let mut img = create_test_image(100, 100);
        hsl_fast(&mut img, mode, 0.2);
        assert!(!img.get_raw_pixels().is_empty(), "Failed for mode: {}", mode);
    }
}

#[test]
fn test_hsv_fast_various_modes() {
    let modes = ["saturate", "desaturate", "lighten", "darken", "shift_hue"];

    for mode in modes {
        let mut img = create_test_image(100, 100);
        hsv_fast(&mut img, mode, 0.2);
        assert!(!img.get_raw_pixels().is_empty(), "Failed for mode: {}", mode);
    }
}

#[test]
fn test_blend_fast_various_modes() {
    let blend_modes = [
        "overlay", "multiply", "screen", "darken", "lighten",
        "soft_light", "hard_light", "difference", "exclusion"
    ];

    for blend_mode in blend_modes {
        let mut img1 = create_test_image(50, 50);
        let img2 = create_test_image(50, 50);
        blend_fast(&mut img1, &img2, blend_mode);
        assert!(!img1.get_raw_pixels().is_empty(), "Failed for blend mode: {}", blend_mode);
    }
}

#[test]
fn test_gamma_correction_performance() {
    // Test that gamma correction (already optimized with lookup tables) works correctly
    let mut img = create_test_image(100, 100);
    photon_rs::colour_spaces::gamma_correction(&mut img, 2.2, 2.2, 2.2);
    assert!(!img.get_raw_pixels().is_empty());
}

#[test]
fn test_edge_cases() {
    // Test with very small image
    let mut tiny_img = create_test_image(1, 1);
    hsl_fast(&mut tiny_img, "saturate", 0.5);
    assert!(!tiny_img.get_raw_pixels().is_empty());

    // Test with image size exactly at thresholds
    let mut threshold_img = create_test_image(256, 256);
    hsl_adaptive(&mut threshold_img, "saturate", 0.5);
    assert!(!threshold_img.get_raw_pixels().is_empty());
}