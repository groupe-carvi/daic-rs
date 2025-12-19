use autocxx::c_int;
use depthai_sys::depthai;

use crate::error::clear_error_flag;

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

#[crate::native_node_wrapper(
    native = "dai::node::StereoDepth",
    inputs(left, right),
    outputs(depth, disparity)
)]
pub struct StereoDepthNode {
    node: crate::pipeline::Node,
}

impl StereoDepthNode {
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
