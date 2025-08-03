//! Pipeline builder for fluent pipeline construction
//!
//! This module provides a builder pattern for constructing DepthAI pipelines.

use crate::pipeline::{Pipeline, PipelineConfig};
use crate::pipeline::nodes::*;
use crate::error::{DaiResult};
use crate::pipeline::nodes::camera::{CameraBoardSocket, CameraResolution, ColorOrder, CameraConfig};
use crate::pipeline::nodes::mono_camera::{MonoCameraConfig, MonoCameraResolution};
use crate::pipeline::nodes::neural_network::NeuralNetworkConfig;
use crate::pipeline::nodes::depth::{DepthConfig, DepthPreset, MedianFilter}; 
use crate::pipeline::nodes::manip::{ImageManipConfig, ResizeMode, ResizeConfig, CropConfig};
use crate::pipeline::nodes::output::XLinkOutConfig;
use crate::pipeline::nodes::input::XLinkInConfig;

/// Builder for constructing DepthAI pipelines
pub struct PipelineBuilder {
    config: PipelineConfig,
    cameras: Vec<(String, Camera)>,
    mono_cameras: Vec<(String, MonoCamera)>,
    neural_networks: Vec<(String, NeuralNetwork)>,
    depth_nodes: Vec<(String, Depth)>,
    manip_nodes: Vec<(String, ImageManip)>,
    output_nodes: Vec<(String, XLinkOut)>,
    input_nodes: Vec<(String, XLinkIn)>,
    connections: Vec<Connection>,
}

/// Represents a connection between two nodes
#[derive(Debug, Clone)]
pub struct Connection {
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}

impl PipelineBuilder {
    /// Create a new pipeline builder
    pub fn new() -> Self {
        Self {
            config: PipelineConfig::default(),
            cameras: Vec::new(),
            mono_cameras: Vec::new(),
            neural_networks: Vec::new(),
            depth_nodes: Vec::new(),
            manip_nodes: Vec::new(),
            output_nodes: Vec::new(),
            input_nodes: Vec::new(),
            connections: Vec::new(),
        }
    }
    
    /// Set pipeline configuration
    pub fn with_config(mut self, config: PipelineConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Add a camera node
    pub fn add_camera(mut self, socket: CameraBoardSocket) -> CameraBuilder<Self> {
        CameraBuilder::new(self, socket)
    }
    
    /// Add a mono camera node
    pub fn add_mono_camera(mut self, socket: CameraBoardSocket) -> MonoCameraBuilder<Self> {
        MonoCameraBuilder::new(self, socket)
    }
    
    /// Add a neural network node
    pub fn add_neural_network(mut self, blob_path: impl Into<String>) -> NeuralNetworkBuilder<Self> {
        NeuralNetworkBuilder::new(self, blob_path)
    }
    
    /// Add a stereo depth node
    pub fn add_stereo_depth(mut self) -> DepthBuilder<Self> {
        DepthBuilder::new(self)
    }
    
    /// Add an image manipulation node
    pub fn add_image_manip(mut self) -> ImageManipBuilder<Self> {
        ImageManipBuilder::new(self)
    }
    
    /// Add an output stream
    pub fn add_output(mut self, stream_name: impl Into<String>) -> OutputBuilder<Self> {
        OutputBuilder::new(self, stream_name)
    }
    
    /// Add an input stream
    pub fn add_input(mut self, stream_name: impl Into<String>) -> InputBuilder<Self> {
        InputBuilder::new(self, stream_name)
    }
    
    /// Connect two nodes
    pub fn connect(mut self, 
                   from_node: impl Into<String>,
                   from_port: impl Into<String>,
                   to_node: impl Into<String>, 
                   to_port: impl Into<String>) -> Self {
        self.connections.push(Connection {
            from_node: from_node.into(),
            from_port: from_port.into(),
            to_node: to_node.into(),
            to_port: to_port.into(),
        });
        self
    }
    
    /// Build the pipeline
    pub fn build(mut self) -> DaiResult<Pipeline> {
        let mut pipeline = Pipeline::with_config(self.config)?;
        
        // Add all nodes to the pipeline
        for (_, camera) in self.cameras {
            pipeline.add_node(camera)?;
        }
        
        for (_, mono_camera) in self.mono_cameras {
            pipeline.add_node(mono_camera)?;
        }
        
        for (_, neural_network) in self.neural_networks {
            pipeline.add_node(neural_network)?;
        }
        
        for (_, depth) in self.depth_nodes {
            pipeline.add_node(depth)?;
        }
        
        for (_, manip) in self.manip_nodes {
            pipeline.add_node(manip)?;
        }
        
        for (_, output) in self.output_nodes {
            pipeline.add_node(output)?;
        }
        
        for (_, input) in self.input_nodes {
            pipeline.add_node(input)?;
        }
        
        // TODO: Apply connections between nodes
        for connection in &self.connections {
            // Connect nodes in the C++ pipeline
            // This requires access to the underlying C++ objects
        }
        
        Ok(pipeline)
    }
    
    // Internal methods for builders
    fn add_camera_internal(&mut self, id: String, camera: Camera) {
        self.cameras.push((id, camera));
    }
    
    fn add_mono_camera_internal(&mut self, id: String, mono_camera: MonoCamera) {
        self.mono_cameras.push((id, mono_camera));
    }
    
    fn add_neural_network_internal(&mut self, id: String, neural_network: NeuralNetwork) {
        self.neural_networks.push((id, neural_network));
    }
    
    fn add_depth_internal(&mut self, id: String, depth: Depth) {
        self.depth_nodes.push((id, depth));
    }
    
    fn add_manip_internal(&mut self, id: String, manip: ImageManip) {
        self.manip_nodes.push((id, manip));
    }
    
    fn add_output_internal(&mut self, id: String, output: XLinkOut) {
        self.output_nodes.push((id, output));
    }
    
    fn add_input_internal(&mut self, id: String, input: XLinkIn) {
        self.input_nodes.push((id, input));
    }
}

impl Default for PipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Camera builder
pub struct CameraBuilder<T> {
    parent: T,
    socket: CameraBoardSocket,
    config: CameraConfig,
    id: String,
}

impl<T> CameraBuilder<T> 
where 
    T: AsMut<PipelineBuilder>
{
    fn new(parent: T, socket: CameraBoardSocket) -> Self {
        let id = format!("camera_{:?}", socket);
        let config = CameraConfig {
            board_socket: Some(socket),
            ..Default::default()
        };
        
        Self {
            parent,
            socket,
            config,
            id,
        }
    }
    
    /// Set camera resolution
    pub fn resolution(mut self, resolution: CameraResolution) -> Self {
        self.config.resolution = resolution;
        self
    }
    
    /// Set camera FPS
    pub fn fps(mut self, fps: f32) -> Self {
        self.config.fps = fps;
        self
    }
    
    /// Set preview size
    pub fn preview_size(mut self, width: u32, height: u32) -> Self {
        self.config.preview_size = Some((width, height));
        self
    }
    
    /// Set color order
    pub fn color_order(mut self, order: ColorOrder) -> Self {
        self.config.color_order = Some(order);
        self
    }
    
    /// Set custom ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
    
    /// Finish building the camera and return to parent builder
    pub fn finish(mut self) -> T {
        let camera = Camera::with_config(&self.id, self.config);
        self.parent.as_mut().add_camera_internal(self.id, camera);
        self.parent
    }
}

impl AsMut<PipelineBuilder> for PipelineBuilder {
    fn as_mut(&mut self) -> &mut PipelineBuilder {
        self
    }
}

// Similar builders for other node types...
pub struct MonoCameraBuilder<T> {
    parent: T,
    socket: CameraBoardSocket,
    config: MonoCameraConfig,
    id: String,
}

impl<T> MonoCameraBuilder<T> 
where 
    T: AsMut<PipelineBuilder>
{
    fn new(parent: T, socket: CameraBoardSocket) -> Self {
        let id = format!("mono_camera_{:?}", socket);
        let config = MonoCameraConfig {
            board_socket: Some(socket),
            ..Default::default()
        };
        
        Self {
            parent,
            socket,
            config,
            id,
        }
    }
    
    pub fn resolution(mut self, resolution: MonoCameraResolution) -> Self {
        self.config.resolution = resolution;
        self
    }
    
    pub fn fps(mut self, fps: f32) -> Self {
        self.config.fps = fps;
        self
    }
    
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
    
    pub fn finish(mut self) -> T {
        let mono_camera = MonoCamera::with_config(&self.id, self.config);
        self.parent.as_mut().add_mono_camera_internal(self.id, mono_camera);
        self.parent
    }
}

pub struct NeuralNetworkBuilder<T> {
    parent: T,
    config: NeuralNetworkConfig,
    id: String,
}

impl<T> NeuralNetworkBuilder<T> 
where 
    T: AsMut<PipelineBuilder>
{
    fn new(parent: T, blob_path: impl Into<String>) -> Self {
        let config = NeuralNetworkConfig {
            blob_path: blob_path.into(),
            ..Default::default()
        };
        
        Self {
            parent,
            config,
            id: "neural_network".to_string(),
        }
    }
    
    pub fn num_threads(mut self, threads: u32) -> Self {
        self.config.num_threads = Some(threads);
        self
    }
    
    pub fn input_size(mut self, width: u32, height: u32) -> Self {
        self.config.input_size = Some((width, height));
        self
    }
    
    pub fn confidence_threshold(mut self, threshold: f32) -> Self {
        self.config.confidence_threshold = Some(threshold);
        self
    }
    
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
    
    pub fn finish(mut self) -> T {
        let neural_network = NeuralNetwork::with_config(&self.id, self.config);
        self.parent.as_mut().add_neural_network_internal(self.id, neural_network);
        self.parent
    }
}

pub struct DepthBuilder<T> {
    parent: T,
    config: DepthConfig,
    id: String,
}

impl<T> DepthBuilder<T> 
where 
    T: AsMut<PipelineBuilder>
{
    fn new(parent: T) -> Self {
        Self {
            parent,
            config: DepthConfig::default(),
            id: "stereo_depth".to_string(),
        }
    }
    
    pub fn preset(mut self, preset: DepthPreset) -> Self {
        self.config.preset = Some(preset);
        self
    }
    
    pub fn left_right_check(mut self, enable: bool) -> Self {
        self.config.left_right_check = enable;
        self
    }
    
    pub fn subpixel(mut self, enable: bool) -> Self {
        self.config.subpixel = enable;
        self
    }
    
    pub fn median_filter(mut self, filter: MedianFilter) -> Self {
        self.config.median_filter = Some(filter);
        self
    }
    
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
    
    pub fn finish(mut self) -> T {
        let depth = Depth::with_config(&self.id, self.config);
        self.parent.as_mut().add_depth_internal(self.id, depth);
        self.parent
    }
}

pub struct ImageManipBuilder<T> {
    parent: T,
    config: ImageManipConfig,
    id: String,
}

impl<T> ImageManipBuilder<T> 
where 
    T: AsMut<PipelineBuilder>
{
    fn new(parent: T) -> Self {
        Self {
            parent,
            config: ImageManipConfig::default(),
            id: "image_manip".to_string(),
        }
    }
    
    pub fn resize(mut self, width: u32, height: u32, mode: ResizeMode) -> Self {
        self.config.resize = Some(ResizeConfig {
            width,
            height,
            mode,
            keep_aspect_ratio: true,
        });
        self
    }
    
    pub fn crop(mut self, x_min: f32, y_min: f32, x_max: f32, y_max: f32) -> Self {
        let crop = CropConfig {
            x_min,
            y_min,
            x_max,
            y_max,
        };
        self.config.crop = Some(crop);
        self
    }
    
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
    
    pub fn finish(mut self) -> T {
        let manip = ImageManip::with_config(self.config);
        self.parent.as_mut().add_manip_internal(self.id, manip);
        self.parent
    }
}

pub struct OutputBuilder<T> {
    parent: T,
    config: XLinkOutConfig,
    id: String,
}

impl<T> OutputBuilder<T> 
where 
    T: AsMut<PipelineBuilder>
{
    fn new(parent: T, stream_name: impl Into<String>) -> Self {
        let stream_name = stream_name.into();
        let id = format!("output_{}", stream_name);
        let config = XLinkOutConfig {
            stream_name,
            ..Default::default()
        };
        
        Self {
            parent,
            config,
            id,
        }
    }
    
    pub fn metadata_only(mut self, metadata_only: bool) -> Self {
        self.config.metadata_only = metadata_only;
        self
    }
    
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
    
    pub fn finish(mut self) -> T {
        let output = XLinkOut::with_config(&self.id, self.config);
        self.parent.as_mut().add_output_internal(self.id, output);
        self.parent
    }
}

pub struct InputBuilder<T> {
    parent: T,
    config: XLinkInConfig,
    id: String,
}

impl<T> InputBuilder<T> 
where 
    T: AsMut<PipelineBuilder>
{
    fn new(parent: T, stream_name: impl Into<String>) -> Self {
        let stream_name = stream_name.into();
        let id = format!("input_{}", stream_name);
        let config = XLinkInConfig {
            stream_name,
            ..Default::default()
        };
        
        Self {
            parent,
            config,
            id,
        }
    }
    
    pub fn max_data_size(mut self, size: u32) -> Self {
        self.config.max_data_size = Some(size as usize);
        self
    }
    
    pub fn num_frames(mut self, frames: u32) -> Self {
        self.config.num_frames = Some(frames as u8);
        self
    }
    
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
    
    pub fn finish(mut self) -> T {
        let input = XLinkIn::with_config(&self.id, self.config);
        self.parent.as_mut().add_input_internal(self.id, input);
        self.parent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_builder() {
        let builder = PipelineBuilder::new()
            .add_camera(CameraBoardSocket::Rgb)
                .resolution(CameraResolution::The1080P)
                .fps(30.0)
                .finish()
            .add_output("video")
                .finish();
        
        // Builder should be constructible
        assert_eq!(builder.cameras.len(), 1);
        assert_eq!(builder.output_nodes.len(), 1);
    }
    
    #[test]
    fn test_complex_pipeline() {
        let builder = PipelineBuilder::new()
            .add_camera(CameraBoardSocket::Rgb)
                .resolution(CameraResolution::The1080P)
                .id("main_camera")
                .finish()
            .add_neural_network("model.blob")
                .input_size(416, 416)
                .confidence_threshold(0.5)
                .id("detector")
                .finish()
            .add_image_manip()
                .resize(416, 416, ResizeMode::Letterbox)
                .id("preprocessor")
                .finish()
            .connect("main_camera", "video", "preprocessor", "inputImage")
            .connect("preprocessor", "out", "detector", "input");
        
        assert_eq!(builder.cameras.len(), 1);
        assert_eq!(builder.neural_networks.len(), 1);
        assert_eq!(builder.manip_nodes.len(), 1);
        assert_eq!(builder.connections.len(), 2);
    }
}
