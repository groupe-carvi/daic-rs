use autocxx::c_int;
use depthai_sys::depthai;

use crate::error::clear_error_flag;

#[crate::native_node_wrapper(
    native = "dai::node::ImageAlign",
    inputs(inputConfig, input, inputAlignTo),
    outputs(outputAligned, passthroughInput)
)]
pub struct ImageAlignNode {
    node: crate::pipeline::Node,
}

impl ImageAlignNode {
    /// Specify whether to run on host or device.
    ///
    /// Mirrors C++: `ImageAlign::setRunOnHost(bool)`.
    pub fn set_run_on_host(&self, run_on_host: bool) {
        clear_error_flag();
        unsafe { depthai::dai_image_align_set_run_on_host(self.node.handle(), run_on_host) };
    }

    /// Specify the output size of the aligned image.
    ///
    /// Mirrors C++: `ImageAlign::setOutputSize(width, height)`.
    pub fn set_output_size(&self, width: i32, height: i32) {
        clear_error_flag();
        unsafe { depthai::dai_image_align_set_output_size(self.node.handle(), c_int(width), c_int(height)) };
    }

    /// Specify whether to keep aspect ratio when resizing.
    ///
    /// Mirrors C++: `ImageAlign::setOutKeepAspectRatio(bool)`.
    pub fn set_out_keep_aspect_ratio(&self, keep: bool) {
        clear_error_flag();
        unsafe { depthai::dai_image_align_set_out_keep_aspect_ratio(self.node.handle(), keep) };
    }
}
