//! Parallelized image processing operations using Rayon.
//!
//! This module provides multi-threaded versions of common image processing operations
//! using Rust's Rayon library. These functions are designed to work with both native
//! and WebAssembly environments (via wasm-bindgen-rayon).
//!
//! Parallel processing is especially beneficial for large images and compute-intensive
//! operations like convolution, blur, and pixel-wise transformations.

use crate::PhotonImage;

#[cfg(all(
    feature = "enable_wasm",
    target_arch = "wasm32",
    not(target_os = "wasi")
))]
use wasm_bindgen::prelude::*;

#[cfg(all(
    feature = "enable_wasm",
    target_arch = "wasm32",
    not(target_os = "wasi")
))]
use wasm_bindgen_rayon::init_thread_pool;

/// Initialize the thread pool for parallel processing.
///
/// This function should be called once at the beginning of your application
/// when using parallel operations in WebAssembly.
///
/// # Example
///
/// ```no_run
/// use photon_rs::parallel::init_parallel;
///
/// // Initialize the thread pool (for WASM)
/// init_parallel();
/// ```
#[cfg(all(
    feature = "enable_wasm",
    target_arch = "wasm32",
    not(target_os = "wasi")
))]
#[wasm_bindgen]
pub fn init_parallel() {
    let _ = init_thread_pool(4); // Default to 4 threads
}

/// Apply a function to each pixel in parallel.
///
/// This is a helper function that processes pixels in parallel chunks,
/// making it easy to parallelize pixel-wise operations.
///
/// # Arguments
/// * `img` - A mutable reference to a PhotonImage.
/// * `f` - A function that takes a mutable reference to a pixel's RGBA values.
///
/// # Example
///
/// ```no_run
/// use photon_rs::parallel::for_each_pixel_parallel;
///
/// fn invert_pixel(pixel: &mut [u8; 4]) {
///     pixel[0] = 255 - pixel[0]; // R
///     pixel[1] = 255 - pixel[1]; // G
///     pixel[2] = 255 - pixel[2]; // B
///     // Alpha remains unchanged
/// }
///
/// // Invert all pixels in parallel
/// for_each_pixel_parallel(&mut img, invert_pixel);
/// ```
pub fn for_each_pixel_parallel<F>(img: &mut PhotonImage, f: F)
where
    F: Fn(&mut [u8; 4]) + Sync + Send,
{
    let pixels = img.raw_pixels.as_mut_slice();
    let len = pixels.len();

    // Minimum image size for parallel processing
    // Too small images benefit less from parallelization
    const MIN_PIXELS_FOR_PARALLEL: usize = 1000;

    if len < MIN_PIXELS_FOR_PARALLEL * 4 {
        // For small images, use sequential processing
        for i in (0..len).step_by(4) {
            unsafe {
                let pixel = [
                    *pixels.get_unchecked(i),
                    *pixels.get_unchecked(i + 1),
                    *pixels.get_unchecked(i + 2),
                    *pixels.get_unchecked(i + 3),
                ];
                let mut pixel_mut = pixel;
                f(&mut pixel_mut);
                *pixels.get_unchecked_mut(i) = pixel_mut[0];
                *pixels.get_unchecked_mut(i + 1) = pixel_mut[1];
                *pixels.get_unchecked_mut(i + 2) = pixel_mut[2];
                *pixels.get_unchecked_mut(i + 3) = pixel_mut[3];
            }
        }
        return;
    }

    // Process pixels in parallel chunks
    #[cfg(all(feature = "enable_wasm", feature = "rayon"))]
    {
        // For WASM, use Rayon via wasm-bindgen-rayon
        use rayon::prelude::*;

        let chunk_size = 4; // Each pixel has 4 bytes (RGBA)
        pixels.par_chunks_exact_mut(chunk_size).for_each(|chunk: &mut [u8]| {
            unsafe {
                let pixel = [
                    *chunk.get_unchecked(0),
                    *chunk.get_unchecked(1),
                    *chunk.get_unchecked(2),
                    *chunk.get_unchecked(3),
                ];
                let mut pixel_mut = pixel;
                f(&mut pixel_mut);
                *chunk.get_unchecked_mut(0) = pixel_mut[0];
                *chunk.get_unchecked_mut(1) = pixel_mut[1];
                *chunk.get_unchecked_mut(2) = pixel_mut[2];
                *chunk.get_unchecked_mut(3) = pixel_mut[3];
            }
        });
    }

    #[cfg(not(feature = "enable_wasm"))]
    {
        // For native, use Rayon directly
        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            
            let chunk_size = 4;
            pixels.par_chunks_exact_mut(chunk_size).for_each(|chunk| {
                unsafe {
                    let pixel = [
                        *chunk.get_unchecked(0),
                        *chunk.get_unchecked(1),
                        *chunk.get_unchecked(2),
                        *chunk.get_unchecked(3),
                    ];
                    let mut pixel_mut = pixel;
                    f(&mut pixel_mut);
                    *chunk.get_unchecked_mut(0) = pixel_mut[0];
                    *chunk.get_unchecked_mut(1) = pixel_mut[1];
                    *chunk.get_unchecked_mut(2) = pixel_mut[2];
                    *chunk.get_unchecked_mut(3) = pixel_mut[3];
                }
            });
        }

        #[cfg(not(feature = "rayon"))]
        {
            // Fallback to sequential if Rayon is not available
            for i in (0..len).step_by(4) {
                unsafe {
                    let pixel = [
                        *pixels.get_unchecked(i),
                        *pixels.get_unchecked(i + 1),
                        *pixels.get_unchecked(i + 2),
                        *pixels.get_unchecked(i + 3),
                    ];
                    let mut pixel_mut = pixel;
                    f(&mut pixel_mut);
                    *pixels.get_unchecked_mut(i) = pixel_mut[0];
                    *pixels.get_unchecked_mut(i + 1) = pixel_mut[1];
                    *pixels.get_unchecked_mut(i + 2) = pixel_mut[2];
                    *pixels.get_unchecked_mut(i + 3) = pixel_mut[3];
                }
            }
        }
    }
}

/// Invert all colors in an image using parallel processing.
///
/// This is the parallel version of the invert operation.
///
/// # Arguments
/// * `photon_image` - A mutable reference to a PhotonImage.
///
/// # Example
///
/// ```no_run
/// use photon_rs::parallel::invert_parallel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// invert_parallel(&mut img);
/// ```
#[cfg_attr(all(feature = "enable_wasm", target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn invert_parallel(photon_image: &mut PhotonImage) {
    for_each_pixel_parallel(photon_image, |pixel| {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
        // Alpha remains unchanged
    });
}

/// Convert an image to grayscale using parallel processing.
///
/// This is the parallel version of the grayscale operation.
/// Uses human-corrected luminance formula: 0.3*R + 0.59*G + 0.11*B
///
/// # Arguments
/// * `photon_image` - A mutable reference to a PhotonImage.
///
/// # Example
///
/// ```no_run
/// use photon_rs::parallel::grayscale_parallel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// grayscale_parallel(&mut img);
/// ```
#[cfg_attr(all(feature = "enable_wasm", target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn grayscale_parallel(photon_image: &mut PhotonImage) {
    for_each_pixel_parallel(photon_image, |pixel| {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let gray = (r * 0.3 + g * 0.59 + b * 0.11) as u8;
        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    });
}

/// Apply brightness adjustment using parallel processing.
///
/// This is the parallel version of the brightness adjustment operation.
///
/// # Arguments
/// * `photon_image` - A mutable reference to a PhotonImage.
/// * `brightness` - The amount to adjust brightness by (-255 to 255).
///
/// # Example
///
/// ```no_run
/// use photon_rs::parallel::adjust_brightness_parallel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// adjust_brightness_parallel(&mut img, 20);
/// ```
#[cfg_attr(all(feature = "enable_wasm", target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn adjust_brightness_parallel(photon_image: &mut PhotonImage, brightness: i16) {
    if brightness > 0 {
        let amt = brightness as u8;
        for_each_pixel_parallel(photon_image, |pixel| {
            pixel[0] = pixel[0].saturating_add(amt);
            pixel[1] = pixel[1].saturating_add(amt);
            pixel[2] = pixel[2].saturating_add(amt);
        });
    } else {
        let amt = brightness.unsigned_abs() as u8;
        for_each_pixel_parallel(photon_image, |pixel| {
            pixel[0] = pixel[0].saturating_sub(amt);
            pixel[1] = pixel[1].saturating_sub(amt);
            pixel[2] = pixel[2].saturating_sub(amt);
        });
    }
}

/// Apply contrast adjustment using parallel processing.
///
/// This is the parallel version of the contrast adjustment operation.
///
/// # Arguments
/// * `photon_image` - A mutable reference to a PhotonImage.
/// * `contrast` - Contrast factor between [-255.0, 255.0].
///
/// # Example
///
/// ```no_run
/// use photon_rs::parallel::adjust_contrast_parallel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// adjust_contrast_parallel(&mut img, 30.0);
/// ```
#[cfg_attr(all(feature = "enable_wasm", target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn adjust_contrast_parallel(photon_image: &mut PhotonImage, contrast: f32) {
    let clamped_contrast = contrast.clamp(-255.0, 255.0);
    let factor = (259.0 * (clamped_contrast + 255.0)) / (255.0 * (259.0 - clamped_contrast));
    let offset = -128.0 * factor + 128.0;

    // Pre-compute lookup table
    let mut lookup_table: [u8; 256] = [0; 256];
    for i in 0..=255_u8 {
        let new_val = i as f32 * factor + offset;
        lookup_table[i as usize] = new_val.clamp(0.0, 255.0) as u8;
    }

    for_each_pixel_parallel(photon_image, |pixel| {
        pixel[0] = lookup_table[pixel[0] as usize];
        pixel[1] = lookup_table[pixel[1] as usize];
        pixel[2] = lookup_table[pixel[2] as usize];
    });
}

/// Apply threshold operation using parallel processing.
///
/// This is the parallel version of the threshold operation.
/// Pixels above threshold become white (255), below become black (0).
///
/// # Arguments
/// * `photon_image` - A mutable reference to a PhotonImage.
/// * `threshold` - The threshold value (0-255).
///
/// # Example
///
/// ```no_run
/// use photon_rs::parallel::threshold_parallel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// threshold_parallel(&mut img, 128);
/// ```
#[cfg_attr(all(feature = "enable_wasm", target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn threshold_parallel(photon_image: &mut PhotonImage, threshold: u32) {
    let threshold_f32 = threshold as f32;
    
    for_each_pixel_parallel(photon_image, |pixel| {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let v = 0.2126 * r + 0.7152 * g + 0.072 * b;
        let result = if v >= threshold_f32 { 255u8 } else { 0u8 };
        pixel[0] = result;
        pixel[1] = result;
        pixel[2] = result;
    });
}

/// Add random noise to an image using parallel processing.
///
/// This is the parallel version of the noise addition operation.
/// Each thread uses its own random number generator to avoid contention.
///
/// # Arguments
/// * `photon_image` - A mutable reference to a PhotonImage.
/// * `strength` - Noise strength. Range: 0.0 to 10.0.
///
/// # Example
///
/// ```no_run
/// use photon_rs::parallel::add_noise_rand_parallel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// add_noise_rand_parallel(&mut img, 2.0);
/// ```
#[cfg_attr(all(feature = "enable_wasm", target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn add_noise_rand_parallel(photon_image: &mut PhotonImage, strength: f32) {
    let strength = strength.clamp(0.0, 10.0);
    let max_offset = (15.0 * strength) as u8;
    
    if max_offset == 0 {
        return;
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
    {
        // For WASM, use JS random
        let pixels = photon_image.raw_pixels.as_mut_slice();
        let _len = pixels.len();

        #[cfg(all(feature = "enable_wasm", feature = "rayon"))]
        {
            use rayon::prelude::*;

            // Process in parallel chunks
            pixels.par_chunks_mut(4).for_each(|chunk| {
                use js_sys::Math::random;
                let offset = (random() * max_offset as f64) as u8;

                for channel in 0..3 {
                    let ch = chunk[channel];
                    chunk[channel] = if ch <= 255 - offset {
                        ch + offset
                    } else {
                        255
                    };
                }
            });
        }
    }

    #[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))]
    {
        use rand::Rng;

        // For native, use thread-local RNG
        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            
            let pixels = photon_image.raw_pixels.as_mut_slice();
            
            pixels.par_chunks_mut(4).for_each(|chunk| {
                let mut rng = rand::thread_rng();
                let offset = rng.gen_range(0..max_offset as i32) as u8;
                
                for channel in 0..3 {
                    let ch = chunk[channel];
                    chunk[channel] = if ch <= 255 - offset {
                        ch + offset
                    } else {
                        255
                    };
                }
            });
        }
        
        #[cfg(not(feature = "rayon"))]
        {
            // Fallback to sequential
            let mut rng = rand::thread_rng();
            for i in (0..photon_image.raw_pixels.len()).step_by(4) {
                let offset = rng.gen_range(0..max_offset as i32) as u8;
                
                for channel in 0..3 {
                    let ch = photon_image.raw_pixels[i + channel];
                    photon_image.raw_pixels[i + channel] = if ch <= 255 - offset {
                        ch + offset
                    } else {
                        255
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_image(width: u32, height: u32) -> PhotonImage {
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        for y in 0..height {
            for x in 0..width {
                let r = ((x * 255) / width.max(1)) as u8;
                let g = ((y * 255) / height.max(1)) as u8;
                let b = (((x + y) * 255) / (width + height).max(1)) as u8;
                let a = 255u8;

                pixels.push(r);
                pixels.push(g);
                pixels.push(b);
                pixels.push(a);
            }
        }

        PhotonImage {
            raw_pixels: pixels,
            width,
            height,
        }
    }

    #[test]
    fn test_invert_parallel() {
        let mut img = create_test_image(100, 100);
        let original = img.clone();
        
        invert_parallel(&mut img);
        
        // Verify inversion
        for i in (0..img.raw_pixels.len()).step_by(4) {
            assert_eq!(img.raw_pixels[i], 255 - original.raw_pixels[i]);
            assert_eq!(img.raw_pixels[i + 1], 255 - original.raw_pixels[i + 1]);
            assert_eq!(img.raw_pixels[i + 2], 255 - original.raw_pixels[i + 2]);
            assert_eq!(img.raw_pixels[i + 3], original.raw_pixels[i + 3]); // Alpha unchanged
        }
    }

    #[test]
    fn test_grayscale_parallel() {
        let mut img = create_test_image(100, 100);
        
        grayscale_parallel(&mut img);
        
        // Verify grayscale conversion
        for i in (0..img.raw_pixels.len()).step_by(4) {
            assert_eq!(img.raw_pixels[i], img.raw_pixels[i + 1]);
            assert_eq!(img.raw_pixels[i + 1], img.raw_pixels[i + 2]);
            assert_eq!(img.raw_pixels[i + 3], 255); // Alpha unchanged
        }
    }

    #[test]
    fn test_adjust_brightness_parallel() {
        let mut img = create_test_image(100, 100);
        
        adjust_brightness_parallel(&mut img, 20);
        
        // Verify brightness increase
        for i in (0..img.raw_pixels.len()).step_by(4) {
            assert!(img.raw_pixels[i] >= 20 || img.raw_pixels[i] == 255);
        }
    }

    #[test]
    fn test_threshold_parallel() {
        let mut img = create_test_image(100, 100);
        
        threshold_parallel(&mut img, 128);
        
        // Verify thresholding
        for i in (0..img.raw_pixels.len()).step_by(4) {
            let val = img.raw_pixels[i];
            assert!(val == 0 || val == 255);
            assert_eq!(img.raw_pixels[i], img.raw_pixels[i + 1]);
            assert_eq!(img.raw_pixels[i + 1], img.raw_pixels[i + 2]);
        }
    }
}