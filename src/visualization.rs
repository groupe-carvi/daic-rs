use rerun::{RecordingStream, RecordingStreamBuilder};

/// Visualization module for DepthAI frames using Rerun
/// 
/// This module provides reusable functions to visualize camera frames,
/// neural network outputs, and other DepthAI data using Rerun viewer.

pub struct RerunVisualizer {
    rec: RecordingStream,
    _storage: Option<rerun::MemorySinkStorage>,
}

impl RerunVisualizer {
    /// Create a new Rerun visualizer with memory sink for real-time viewing
    pub fn new(app_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (rec, storage) = RecordingStreamBuilder::new(app_name)
            .memory()?;
        
        Ok(RerunVisualizer {
            rec,
            _storage: Some(storage),
        })
    }
    
    /// Create a new Rerun visualizer that saves to file
    pub fn new_with_file(app_name: &str, file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let rec = RecordingStreamBuilder::new(app_name)
            .save(file_path)?;
        
        Ok(RerunVisualizer {
            rec,
            _storage: None,
        })
    }
    
    /// Log a grayscale camera frame to Rerun
    pub fn log_camera_frame(&self, entity_path: &str, frame_data: &[u8], width: u32, height: u32) -> Result<(), rerun::RecordingStreamError> {
        self.rec.log(
            entity_path,
            &rerun::Image::from_elements(
                frame_data,
                [width, height],
                rerun::ColorModel::L
            )
        )
    }
    
    /// Log an RGB camera frame to Rerun
    pub fn log_rgb_frame(&self, entity_path: &str, frame_data: &[u8], width: u32, height: u32) -> Result<(), rerun::RecordingStreamError> {
        self.rec.log(
            entity_path,
            &rerun::Image::from_elements(
                frame_data,
                [width, height],
                rerun::ColorModel::RGB
            )
        )
    }
    
    /// Log detection results (bounding boxes) to Rerun
    pub fn log_detections(&self, entity_path: &str, detections: &[Detection]) -> Result<(), rerun::RecordingStreamError> {
        let boxes: Vec<rerun::Box2D> = detections.iter().map(|det| {
            rerun::Box2D::from_xywh(det.x, det.y, det.width, det.height)
        }).collect();
        
        self.rec.log(entity_path, &rerun::Boxes2D::from_boxes(boxes))
    }
    
    /// Log text information to Rerun
    pub fn log_text(&self, entity_path: &str, text: &str) -> Result<(), rerun::RecordingStreamError> {
        self.rec.log(entity_path, &rerun::TextDocument::new(text))
    }
    
    /// Log a scalar value (for plotting metrics, FPS, etc.)
    pub fn log_scalar(&self, entity_path: &str, value: f64) -> Result<(), rerun::RecordingStreamError> {
        self.rec.log(entity_path, &rerun::Scalar::new(value))
    }
    
    /// Get the underlying RecordingStream for advanced usage
    pub fn recording_stream(&self) -> &RecordingStream {
        &self.rec
    }
}

/// Simple detection structure for visualization
#[derive(Debug, Clone)]
pub struct Detection {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub confidence: f32,
    pub label: String,
}

impl Detection {
    pub fn new(x: f32, y: f32, width: f32, height: f32, confidence: f32, label: String) -> Self {
        Self { x, y, width, height, confidence, label }
    }
}

/// Helper function to print Rerun setup instructions
pub fn print_rerun_instructions() {
    println!("📌 To view the visualization: run 'rerun' in another terminal");
    println!("   If not installed: pip install rerun-sdk");
    println!("✓ Data will be streamed to Rerun - open viewer to see frames in real-time");
}

/// Helper function to wait for system stabilization (useful for DepthAI)
pub fn wait_for_stabilization(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
}
