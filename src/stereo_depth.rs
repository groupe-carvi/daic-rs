use autocxx::c_int;
use depthai_sys::depthai;

use crate::error::{clear_error_flag, Result};
use crate::pipeline::{DeviceNode, NodeKind, Pipeline};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetMode {
    FastAccuracy = 0,
    FastDensity = 1,
    Default = 2,
    Face = 3,
    HighDetail = 4,
    Robotics = 5,
}

pub struct StereoDepthNode {
    node: crate::pipeline::Node,
}

impl StereoDepthNode {
    pub fn as_node(&self) -> &crate::pipeline::Node {
        &self.node
    }

    pub fn set_default_profile_preset(&self, mode: PresetMode) {
        clear_error_flag();
        unsafe { depthai::dai_stereo_set_default_profile_preset(self.node.handle(), c_int(mode as i32)) };
    }

    pub fn set_left_right_check(&self, enable: bool) {
        clear_error_flag();
        unsafe { depthai::dai_stereo_set_left_right_check(self.node.handle(), enable) };
    }

    pub fn set_subpixel(&self, enable: bool) {
        clear_error_flag();
        unsafe { depthai::dai_stereo_set_subpixel(self.node.handle(), enable) };
    }

    pub fn set_extended_disparity(&self, enable: bool) {
        clear_error_flag();
        unsafe { depthai::dai_stereo_set_extended_disparity(self.node.handle(), enable) };
    }

    pub fn enable_distortion_correction(&self, enable: bool) {
        clear_error_flag();
        unsafe { depthai::dai_stereo_enable_distortion_correction(self.node.handle(), enable) };
    }
}

unsafe impl DeviceNode for StereoDepthNode {
    fn create_in_pipeline(pipeline: &Pipeline) -> Result<Self> {
        let node = pipeline.create_node(NodeKind::StereoDepth)?;
        Ok(Self { node })
    }
}
