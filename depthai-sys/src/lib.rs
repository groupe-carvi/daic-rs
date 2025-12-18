// Use autocxx to generate C++ bindings
use autocxx::prelude::*;

include_cpp! {
    #include "autocxx_wrapper.h"

    // Version information helpers
    generate!("dai::dai_build_version")
    generate!("dai::dai_build_version_major")
    generate!("dai::dai_build_version_minor")
    generate!("dai::dai_build_version_patch")
    generate!("dai::dai_build_pre_release_type")
    generate!("dai::dai_build_pre_release_version")
    generate!("dai::dai_build_commit")
    generate!("dai::dai_build_commit_datetime")
    generate!("dai::dai_build_build_datetime")
    generate!("dai::dai_build_device_version")
    generate!("dai::dai_build_bootloader_version")
    generate!("dai::dai_build_device_rvc3_version")
    generate!("dai::dai_build_device_rvc4_version")

    // Device functions
    generate!("dai::dai_device_new")
    generate!("dai::dai_device_clone")
    generate!("dai::dai_device_delete")
    generate!("dai::dai_device_is_closed")
    generate!("dai::dai_device_close")
    generate!("dai::dai_device_get_connected_camera_sockets")
    generate!("dai::dai_pipeline_new_with_device")

    // Pipeline functions
    generate!("dai::dai_pipeline_start_default")
    generate!("dai::dai_pipeline_get_default_device")
    generate!("dai::dai_pipeline_new")
    generate!("dai::dai_pipeline_delete")
    generate!("dai::dai_pipeline_start")
    generate!("dai::dai_rgbd_build")
    generate!("dai::dai_pipeline_create_camera")

    // Generic node creation / linking
    generate!("dai::dai_pipeline_create_node")
    generate!("dai::dai_node_get_output")
    generate!("dai::dai_output_link")
    generate!("dai::dai_node_link")
    generate!("dai::dai_node_unlink")

    // Device helpers
    generate!("dai::dai_device_get_platform")
    generate!("dai::dai_device_set_ir_laser_dot_projector_intensity")

    // StereoDepth configuration helpers
    generate!("dai::dai_stereo_set_subpixel")
    generate!("dai::dai_stereo_set_extended_disparity")
    generate!("dai::dai_stereo_set_default_profile_preset")
    generate!("dai::dai_stereo_set_left_right_check")
    generate!("dai::dai_stereo_set_rectify_edge_fill_color")
    generate!("dai::dai_stereo_enable_distortion_correction")
    generate!("dai::dai_stereo_initial_set_left_right_check_threshold")
    generate!("dai::dai_stereo_initial_set_threshold_filter_max_range")

    // RGBD configuration helpers
    generate!("dai::dai_rgbd_set_depth_unit")

    // Camera functions
    generate!("dai::dai_camera_request_output")
    generate!("dai::dai_camera_request_full_resolution_output")

    // Queue/frame helpers
    generate!("dai::dai_output_create_queue")
    generate!("dai::dai_queue_delete")
    generate!("dai::dai_queue_get_frame")
    generate!("dai::dai_queue_try_get_frame")
    generate!("dai::dai_queue_get_pointcloud")
    generate!("dai::dai_queue_try_get_pointcloud")
    generate!("dai::dai_queue_get_rgbd")
    generate!("dai::dai_queue_try_get_rgbd")
    generate!("dai::dai_frame_get_data")
    generate!("dai::dai_frame_get_width")
    generate!("dai::dai_frame_get_height")
    generate!("dai::dai_frame_get_type")
    generate!("dai::dai_frame_get_size")
    generate!("dai::dai_frame_release")

    // PointCloudData accessors
    generate!("dai::dai_pointcloud_get_width")
    generate!("dai::dai_pointcloud_get_height")
    generate!("dai::dai_pointcloud_get_points_rgba")
    generate!("dai::dai_pointcloud_get_points_rgba_len")
    generate!("dai::dai_pointcloud_release")

    // RGBDData accessors
    generate!("dai::dai_rgbd_get_rgb_frame")
    generate!("dai::dai_rgbd_get_depth_frame")
    generate!("dai::dai_rgbd_release")

    // Utilities
    generate!("dai::dai_camera_socket_name")
    generate!("dai::dai_string_to_cstring")
    generate!("dai::dai_free_cstring")
    generate!("dai::dai_get_last_error")
    generate!("dai::dai_clear_last_error")

    safety!(unsafe_ffi)
}

// Define our own opaque handle types for type safety
// These are just wrappers around void* but provide type distinction
pub type DaiDevice = *mut autocxx::c_void;
pub type DaiPipeline = *mut autocxx::c_void;
pub type DaiNode = *mut autocxx::c_void;
pub type DepthaiameraNode = *mut autocxx::c_void;
pub type DaiOutput = *mut autocxx::c_void;
pub type DaiDataQueue = *mut autocxx::c_void;
pub type DaiImgFrame = *mut autocxx::c_void;
pub type DaiPointCloud = *mut autocxx::c_void;
pub type DaiRGBDData = *mut autocxx::c_void;

pub mod string_utils;

// Re-export for convenience
pub use ffi::*;
pub use ffi::dai as depthai;
