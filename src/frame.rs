//! Safe Rust API for DepthAI Frame/Image

use std::time::Instant;

pub struct Frame {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub timestamp: Instant,
    pub frame_id: u32,
}

impl Frame {
    /// Create a dummy frame (grayscale 640x480)
    pub fn dummy() -> Self {
        Frame {
            data: vec![0u8; 640 * 480],
            width: 640,
            height: 480,
            timestamp: Instant::now(),
            frame_id: 0,
        }
    }

    /// Create a frame with specific data and dimensions
    pub fn new_with_data(width: usize, height: usize, data: Vec<u8>) -> Self {
        Frame {
            data,
            width,
            height,
            timestamp: Instant::now(),
            frame_id: 0,
        }
    }

    /// Create a test pattern frame (useful for testing visualization)
    pub fn test_pattern(width: usize, height: usize, pattern_type: TestPattern) -> Self {
        let data = match pattern_type {
            TestPattern::Gradient => {
                (0..height * width)
                    .map(|i| {
                        let x = i % width;
                        let y = i / width;
                        ((x + y) % 256) as u8
                    })
                    .collect()
            }
            TestPattern::Checkerboard => {
                (0..height * width)
                    .map(|i| {
                        let x = i % width;
                        let y = i / width;
                        if (x / 32 + y / 32) % 2 == 0 { 255 } else { 0 }
                    })
                    .collect()
            }
            TestPattern::Noise => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                
                (0..height * width)
                    .map(|i| {
                        let mut hasher = DefaultHasher::new();
                        i.hash(&mut hasher);
                        (hasher.finish() % 256) as u8
                    })
                    .collect()
            }
        };

        Frame {
            data,
            width,
            height,
            timestamp: Instant::now(),
            frame_id: 0,
        }
    }

    /// Set frame ID for tracking
    pub fn with_frame_id(mut self, frame_id: u32) -> Self {
        self.frame_id = frame_id;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TestPattern {
    Gradient,
    Checkerboard,
    Noise,
}
