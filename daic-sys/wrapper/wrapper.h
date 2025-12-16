#pragma once

// NOTE: This header intentionally avoids including DepthAI / heavy C++ headers.
// It defines a stable C ABI surface using opaque handles (void*) and POD types.
//
// - Used by the C++ wrapper implementation (`wrapper.cpp`) and
// - Included by the binding generator (autocxx) via `autocxx_wrapper.h`.

#include <cstddef>  // size_t

#ifdef _WIN32
#define API __declspec(dllexport)
#else
#define API
#endif

#ifdef __cplusplus
extern "C" {
#endif

namespace daic {
// Version informations getters
API const char* dai_build_version();
API int dai_build_version_major();
API int dai_build_version_minor();
API int dai_build_version_patch();
API const char* dai_build_pre_release_type();
API int dai_build_pre_release_version();
API const char* dai_build_commit();
API const char* dai_build_commit_datetime();
API const char* dai_build_build_datetime();
API const char* dai_build_device_version();
API const char* dai_build_bootloader_version();
API const char* dai_build_device_rvc3_version();
API const char* dai_build_device_rvc4_version();

// Helper to duplicate/free returned strings (caller must free)
API char* dai_string_to_cstring(const char* str);
API void dai_free_cstring(char* cstring);

// Opaque handle types
typedef void* DaiDevice;      // currently: `std::shared_ptr<dai::Device>*`
typedef void* DaiPipeline;    // currently: `dai::Pipeline*`
typedef void* DaiNode;        // currently: `dai::Node*` (derived node instance)
typedef void* DaiCameraNode;  // currently: `dai::node::Camera*`
typedef void* DaiOutput;      // currently: `dai::Node::Output*`
typedef void* DaiDataQueue;   // currently: `std::shared_ptr<dai::MessageQueue>*`
typedef void* DaiImgFrame;    // currently: `std::shared_ptr<dai::ImgFrame>*`
typedef void* DaiPointCloud;  // currently: wrapper-owned view of `std::shared_ptr<dai::PointCloudData>`
typedef void* DaiRGBDData;    // currently: `std::shared_ptr<dai::RGBDData>*`

// POD view of `dai::Point3fRGBA`
typedef struct DaiPoint3fRGBA {
	float x;
	float y;
	float z;
	unsigned char r;
	unsigned char g;
	unsigned char b;
	unsigned char a;
} DaiPoint3fRGBA;

// Low-level device operations
API DaiDevice dai_device_new();
API DaiDevice dai_device_clone(DaiDevice device);
API void dai_device_delete(DaiDevice device);
API bool dai_device_is_closed(DaiDevice device);
API void dai_device_close(DaiDevice device);

// Low-level pipeline operations  
API DaiPipeline dai_pipeline_new();
API DaiPipeline dai_pipeline_new_with_device(DaiDevice device);
API void dai_pipeline_delete(DaiPipeline pipeline);
API bool dai_pipeline_start(DaiPipeline pipeline);
// Builder helpers (mirror native API: `pipeline.create<node::RGBD>()->build()`).
API DaiNode dai_rgbd_build(DaiNode rgbd);

// Pipeline <-> device interop
API DaiDevice dai_pipeline_get_default_device(DaiPipeline pipeline);

// Generic node creation / linking
// Note: `DaiNode` is an erased node pointer; it must originate from the same pipeline.
// kind values are defined by the Rust-side `NodeKind` enum.
API bool dai_pipeline_start_default(DaiPipeline pipeline);
API DaiNode dai_pipeline_create_node(DaiPipeline pipeline, int kind);
// Output helpers
API DaiOutput dai_node_get_output(DaiNode node, const char* group, const char* name);
API bool dai_output_link(DaiOutput from, DaiNode to, const char* in_group, const char* in_name);
API bool dai_node_link(DaiNode from, const char* out_group, const char* out_name, DaiNode to, const char* in_group, const char* in_name);
API bool dai_node_unlink(DaiNode from, const char* out_group, const char* out_name, DaiNode to, const char* in_group, const char* in_name);

// Device helpers
API int dai_device_get_platform(DaiDevice device);
API void dai_device_set_ir_laser_dot_projector_intensity(DaiDevice device, float intensity);

// StereoDepth configuration helpers
API void dai_stereo_set_subpixel(DaiNode stereo, bool enable);
API void dai_stereo_set_extended_disparity(DaiNode stereo, bool enable);
API void dai_stereo_set_default_profile_preset(DaiNode stereo, int preset_mode);
API void dai_stereo_set_left_right_check(DaiNode stereo, bool enable);
API void dai_stereo_set_rectify_edge_fill_color(DaiNode stereo, int color);
API void dai_stereo_enable_distortion_correction(DaiNode stereo, bool enable);
API void dai_stereo_initial_set_left_right_check_threshold(DaiNode stereo, int threshold);
API void dai_stereo_initial_set_threshold_filter_max_range(DaiNode stereo, int max_range);

// RGBD configuration helpers
API void dai_rgbd_set_depth_unit(DaiNode rgbd, int depth_unit);

// Low-level camera node operations
API DaiCameraNode dai_pipeline_create_camera(DaiPipeline pipeline, int board_socket);

// Camera output wrappers
API DaiOutput dai_camera_request_output(DaiCameraNode camera, int width, int height, int type, int resize_mode, float fps, int enable_undistortion);
API DaiOutput dai_camera_request_full_resolution_output(DaiCameraNode camera);

// Low-level output operations
API DaiDataQueue dai_output_create_queue(DaiOutput output, unsigned int max_size, bool blocking);

// Low-level queue operations
API void dai_queue_delete(DaiDataQueue queue);
API DaiImgFrame dai_queue_get_frame(DaiDataQueue queue, int timeout_ms);
API DaiImgFrame dai_queue_try_get_frame(DaiDataQueue queue);

// Message retrieval for non-ImgFrame outputs
API DaiPointCloud dai_queue_get_pointcloud(DaiDataQueue queue, int timeout_ms);
API DaiPointCloud dai_queue_try_get_pointcloud(DaiDataQueue queue);
API DaiRGBDData dai_queue_get_rgbd(DaiDataQueue queue, int timeout_ms);
API DaiRGBDData dai_queue_try_get_rgbd(DaiDataQueue queue);

// PointCloud view accessors
API int dai_pointcloud_get_width(DaiPointCloud pcl);
API int dai_pointcloud_get_height(DaiPointCloud pcl);
API const DaiPoint3fRGBA* dai_pointcloud_get_points_rgba(DaiPointCloud pcl);
API size_t dai_pointcloud_get_points_rgba_len(DaiPointCloud pcl);
API void dai_pointcloud_release(DaiPointCloud pcl);

// RGBDData accessors
API DaiImgFrame dai_rgbd_get_rgb_frame(DaiRGBDData rgbd);
API DaiImgFrame dai_rgbd_get_depth_frame(DaiRGBDData rgbd);
API void dai_rgbd_release(DaiRGBDData rgbd);

// Low-level frame operations
API void* dai_frame_get_data(DaiImgFrame frame);
API int dai_frame_get_width(DaiImgFrame frame);
API int dai_frame_get_height(DaiImgFrame frame);
API int dai_frame_get_type(DaiImgFrame frame);
API size_t dai_frame_get_size(DaiImgFrame frame);
API void dai_frame_release(DaiImgFrame frame);

// Low-level utility functions
API int dai_device_get_connected_camera_sockets(DaiDevice device, int* sockets, int max_count);
API const char* dai_camera_socket_name(int socket);

// Error handling
API const char* dai_get_last_error();
API void dai_clear_last_error();

} // namespace daic


#ifdef __cplusplus
}
#endif
