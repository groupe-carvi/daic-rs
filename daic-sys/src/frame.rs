//! Safe Rust API for DepthAI Frame/Image

pub struct Frame {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Frame {
    /// Create a dummy frame (grayscale 640x480)
    pub fn dummy() -> Self {
        Frame {
            data: vec![0u8; 640 * 480],
            width: 640,
            height: 480,
        }
    }
}
