//! Pipeline nodes module
//!
//! This module contains all the different types of nodes that can be added to a pipeline.

pub mod camera;
pub mod mono_camera;
pub mod neural_network;
pub mod depth;
pub mod manip;
pub mod output;
pub mod input;

// Re-export node types
pub use camera::{Camera, CameraConfig, CameraResolution, ColorOrder, CameraBoardSocket};
pub use mono_camera::{MonoCamera, MonoCameraConfig, MonoCameraResolution};
pub use neural_network::{NeuralNetwork, NeuralNetworkConfig};
pub use depth::{Depth, DepthConfig, DepthPreset, MedianFilter};
pub use manip::{ImageManip, ImageManipConfig, ResizeMode, ResizeConfig, CropConfig};
pub use output::{XLinkOut, XLinkOutConfig};
pub use input::{XLinkIn, XLinkInConfig};


