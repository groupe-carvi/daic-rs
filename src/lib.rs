pub use depthai_sys as bindings;

// Re-export proc-macros for ergonomic use: `use depthai::native_node_wrapper;`.
extern crate self as depthai;

pub use depthai_macros::native_node_wrapper;
pub use depthai_macros::depthai_composite;
pub use depthai_macros::depthai_host_node;
pub use depthai_macros::depthai_threaded_host_node;

pub mod camera;
pub mod common;
pub mod device;
pub mod error;
pub mod host_node;
pub mod threaded_host_node;
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

pub use output::{Output, Input};
pub use pointcloud::{Point3fRGBA, PointCloudData};
pub use rgbd::{DepthUnit, RgbdData, RgbdNode};
pub use stereo_depth::{PresetMode as StereoPresetMode, StereoDepthNode};
pub use host_node::{HostNode, HostNodeImpl, MessageGroup, Buffer};
pub use threaded_host_node::{ThreadedHostNode, ThreadedHostNodeImpl, ThreadedHostNodeContext};
