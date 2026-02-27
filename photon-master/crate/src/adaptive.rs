//! Adaptive optimization strategies based on image size.
//!
//! This module provides utilities for selecting optimal algorithms based on
//! image dimensions, providing better performance for both small and large images.

use crate::PhotonImage;

/// Thresholds for image size classification
pub const SMALL_IMAGE_THRESHOLD: u32 = 256 * 256;   // 256x256 pixels = 65,536 pixels
pub const MEDIUM_IMAGE_THRESHOLD: u32 = 1024 * 1024; // 1024x1024 pixels = 1,048,576 pixels

/// Image size classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageSize {
    /// Small images (< 256x256 pixels)
    Small,
    /// Medium images (256x256 to 1024x1024 pixels)
    Medium,
    /// Large images (> 1024x1024 pixels)
    Large,
}

impl ImageSize {
    /// Classify an image based on its dimensions
    pub fn classify(width: u32, height: u32) -> Self {
        let pixel_count = width * height;

        if pixel_count < SMALL_IMAGE_THRESHOLD {
            ImageSize::Small
        } else if pixel_count < MEDIUM_IMAGE_THRESHOLD {
            ImageSize::Medium
        } else {
            ImageSize::Large
        }
    }

    /// Get the recommended batch size for processing based on image size
    pub fn recommended_batch_size(&self) -> usize {
        match self {
            ImageSize::Small => 4,   // Process 4 pixels at a time for small images
            ImageSize::Medium => 8,  // Process 8 pixels at a time for medium images
            ImageSize::Large => 16,  // Process 16 pixels at a time for large images
        }
    }

    /// Get the recommended chunk size for parallel processing
    pub fn recommended_chunk_size(&self) -> usize {
        match self {
            ImageSize::Small => 256,      // Small chunks for small images
            ImageSize::Medium => 1024,    // Medium chunks for medium images
            ImageSize::Large => 4096,     // Large chunks for large images
        }
    }

    /// Determine if SIMD optimization should be used
    pub fn use_simd(&self) -> bool {
        // Use SIMD for medium and large images
        matches!(self, ImageSize::Medium | ImageSize::Large)
    }

    /// Determine if parallel processing should be used
    pub fn use_parallel(&self) -> bool {
        // Use parallel processing only for large images
        matches!(self, ImageSize::Large)
    }
}

/// Get image size classification for a PhotonImage
pub fn get_image_size(image: &PhotonImage) -> ImageSize {
    ImageSize::classify(image.width, image.height)
}

/// Adaptive processing helper that selects the optimal batch size based on image size
pub fn get_optimal_batch_size(image: &PhotonImage) -> usize {
    get_image_size(image).recommended_batch_size()
}

/// Adaptive processing helper that selects the optimal chunk size for parallel processing
pub fn get_optimal_chunk_size(image: &PhotonImage) -> usize {
    get_image_size(image).recommended_chunk_size()
}

/// Determine if SIMD optimization should be used for this image
pub fn should_use_simd(image: &PhotonImage) -> bool {
    get_image_size(image).use_simd()
}

/// Determine if parallel processing should be used for this image
pub fn should_use_parallel(image: &PhotonImage) -> bool {
    get_image_size(image).use_parallel()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_size_classification() {
        assert_eq!(ImageSize::classify(100, 100), ImageSize::Small);
        assert_eq!(ImageSize::classify(512, 512), ImageSize::Medium);
        assert_eq!(ImageSize::classify(2048, 2048), ImageSize::Large);
    }

    #[test]
    fn test_batch_size_selection() {
        assert_eq!(ImageSize::Small.recommended_batch_size(), 4);
        assert_eq!(ImageSize::Medium.recommended_batch_size(), 8);
        assert_eq!(ImageSize::Large.recommended_batch_size(), 16);
    }

    #[test]
    fn test_simd_usage() {
        assert!(!ImageSize::Small.use_simd());
        assert!(ImageSize::Medium.use_simd());
        assert!(ImageSize::Large.use_simd());
    }

    #[test]
    fn test_parallel_usage() {
        assert!(!ImageSize::Small.use_parallel());
        assert!(!ImageSize::Medium.use_parallel());
        assert!(ImageSize::Large.use_parallel());
    }
}