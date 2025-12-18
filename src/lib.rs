pub use depthai_sys as bindings;

pub mod camera;
pub mod common;
pub mod device;
pub mod error;
pub mod output;
pub mod pipeline;
pub mod pointcloud;
pub mod rgbd;
pub mod stereo_depth;

pub use error::{DepthaiError, Result};
pub use pipeline::{CreateInPipeline, CreateInPipelineWith, DeviceNode, DeviceNodeWithParams};

pub use device::Device;
pub use device::DevicePlatform;
pub use pipeline::Pipeline;

pub use output::Output;
pub use pointcloud::{Point3fRGBA, PointCloudData};
pub use rgbd::{DepthUnit, RgbdData, RgbdNode};
pub use stereo_depth::{PresetMode as StereoPresetMode, StereoDepthNode};
