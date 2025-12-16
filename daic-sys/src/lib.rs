// Use autocxx to generate C++ bindings
use autocxx::prelude::*;

include_cpp! {
    #include "autocxx_wrapper.h"

    // Version information helpers
    generate!("daic::dai_build_version")
    generate!("daic::dai_build_version_major")
    generate!("daic::dai_build_version_minor")
    generate!("daic::dai_build_version_patch")
    generate!("daic::dai_build_pre_release_type")
    generate!("daic::dai_build_pre_release_version")
    generate!("daic::dai_build_commit")
    generate!("daic::dai_build_commit_datetime")
    generate!("daic::dai_build_build_datetime")
    generate!("daic::dai_build_device_version")
    generate!("daic::dai_build_bootloader_version")
    generate!("daic::dai_build_device_rvc3_version")
    generate!("daic::dai_build_device_rvc4_version")

    // Device functions
    generate!("daic::dai_device_new")
    generate!("daic::dai_device_clone")
    generate!("daic::dai_device_delete")
    generate!("daic::dai_device_is_closed")
    generate!("daic::dai_device_close")
    generate!("daic::dai_device_get_connected_camera_sockets")
    generate!("daic::dai_pipeline_new_with_device")

    // Pipeline functions
    generate!("daic::dai_pipeline_start_default")
    generate!("daic::dai_pipeline_get_default_device")
    generate!("daic::dai_pipeline_new")
    generate!("daic::dai_pipeline_delete")
    generate!("daic::dai_pipeline_start")
    generate!("daic::dai_rgbd_build")
    generate!("daic::dai_pipeline_create_camera")

    // Generic node creation / linking
    generate!("daic::dai_pipeline_create_node")
    generate!("daic::dai_node_get_output")
    generate!("daic::dai_output_link")
    generate!("daic::dai_node_link")
    generate!("daic::dai_node_unlink")

    // Device helpers
    generate!("daic::dai_device_get_platform")
    generate!("daic::dai_device_set_ir_laser_dot_projector_intensity")

    // StereoDepth configuration helpers
    generate!("daic::dai_stereo_set_subpixel")
    generate!("daic::dai_stereo_set_extended_disparity")
    generate!("daic::dai_stereo_set_default_profile_preset")
    generate!("daic::dai_stereo_set_left_right_check")
    generate!("daic::dai_stereo_set_rectify_edge_fill_color")
    generate!("daic::dai_stereo_enable_distortion_correction")
    generate!("daic::dai_stereo_initial_set_left_right_check_threshold")
    generate!("daic::dai_stereo_initial_set_threshold_filter_max_range")

    // RGBD configuration helpers
    generate!("daic::dai_rgbd_set_depth_unit")

    // Camera functions
    generate!("daic::dai_camera_request_output")
    generate!("daic::dai_camera_request_full_resolution_output")

    // Queue/frame helpers
    generate!("daic::dai_output_create_queue")
    generate!("daic::dai_queue_delete")
    generate!("daic::dai_queue_get_frame")
    generate!("daic::dai_queue_try_get_frame")
    generate!("daic::dai_queue_get_pointcloud")
    generate!("daic::dai_queue_try_get_pointcloud")
    generate!("daic::dai_queue_get_rgbd")
    generate!("daic::dai_queue_try_get_rgbd")
    generate!("daic::dai_frame_get_data")
    generate!("daic::dai_frame_get_width")
    generate!("daic::dai_frame_get_height")
    generate!("daic::dai_frame_get_type")
    generate!("daic::dai_frame_get_size")
    generate!("daic::dai_frame_release")

    // PointCloudData accessors
    generate!("daic::dai_pointcloud_get_width")
    generate!("daic::dai_pointcloud_get_height")
    generate!("daic::dai_pointcloud_get_points_rgba")
    generate!("daic::dai_pointcloud_get_points_rgba_len")
    generate!("daic::dai_pointcloud_release")

    // RGBDData accessors
    generate!("daic::dai_rgbd_get_rgb_frame")
    generate!("daic::dai_rgbd_get_depth_frame")
    generate!("daic::dai_rgbd_release")

    // Utilities
    generate!("daic::dai_camera_socket_name")
    generate!("daic::dai_string_to_cstring")
    generate!("daic::dai_free_cstring")
    generate!("daic::dai_get_last_error")
    generate!("daic::dai_clear_last_error")

    safety!(unsafe_ffi)
}

// Define our own opaque handle types for type safety
// These are just wrappers around void* but provide type distinction
pub type DaiDevice = *mut autocxx::c_void;
pub type DaiPipeline = *mut autocxx::c_void;
pub type DaiNode = *mut autocxx::c_void;
pub type DaiCameraNode = *mut autocxx::c_void;
pub type DaiOutput = *mut autocxx::c_void;
pub type DaiDataQueue = *mut autocxx::c_void;
pub type DaiImgFrame = *mut autocxx::c_void;
pub type DaiPointCloud = *mut autocxx::c_void;
pub type DaiRGBDData = *mut autocxx::c_void;

pub mod string_utils;

// Re-export for convenience
pub use ffi::*;
