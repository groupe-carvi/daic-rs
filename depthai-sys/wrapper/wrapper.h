#pragma once

// NOTE: This header intentionally avoids including DepthAI / heavy C++ headers.
// It defines a stable C ABI surface using opaque handles (void*) and POD types.
//
// - Used by the C++ wrapper implementation (`wrapper.cpp`) and
// - Included by the binding generator (autocxx) via `autocxx_wrapper.h`.

#include <cstddef>  // size_t
#include <cstdint>  // uint32_t

#ifdef _WIN32
#define API __declspec(dllexport)
#else
#define API
#endif

#ifdef __cplusplus
namespace dai {
extern "C" {
#endif

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
typedef void* DaiInput;       // currently: `dai::Node::Input*`
typedef void* DaiDataQueue;   // currently: `std::shared_ptr<dai::MessageQueue>*`
typedef void* DaiImgFrame;    // currently: `std::shared_ptr<dai::ImgFrame>*`
typedef void* DaiPointCloud;  // currently: wrapper-owned view of `std::shared_ptr<dai::PointCloudData>`
typedef void* DaiRGBDData;    // currently: `std::shared_ptr<dai::RGBDData>*`
typedef void* DaiMessageGroup; // currently: `std::shared_ptr<dai::MessageGroup>*`
typedef void* DaiBuffer;       // currently: `std::shared_ptr<dai::Buffer>*`

// Host node callback types
typedef DaiBuffer (*DaiHostNodeProcessGroup)(void* ctx, DaiMessageGroup group);
typedef void (*DaiHostNodeCallback)(void* ctx);
typedef void (*DaiThreadedHostNodeRun)(void* ctx);

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
API DaiNode dai_pipeline_create_host_node(DaiPipeline pipeline,
                                          void* ctx,
                                          DaiHostNodeProcessGroup process_cb,
                                          DaiHostNodeCallback on_start_cb,
                                          DaiHostNodeCallback on_stop_cb,
                                          DaiHostNodeCallback drop_cb);
API DaiNode dai_pipeline_create_threaded_host_node(DaiPipeline pipeline,
                                                   void* ctx,
                                                   DaiThreadedHostNodeRun run_cb,
                                                   DaiHostNodeCallback on_start_cb,
                                                   DaiHostNodeCallback on_stop_cb,
                                                   DaiHostNodeCallback drop_cb);
// Builder helpers (mirror native API: `pipeline.create<node::RGBD>()->build()`).
API DaiNode dai_rgbd_build(DaiNode rgbd);

// Pipeline <-> device interop
API DaiDevice dai_pipeline_get_default_device(DaiPipeline pipeline);

// Generic node creation / linking
// Note: `DaiNode` is an erased node pointer; it must originate from the same pipeline.
API bool dai_pipeline_start_default(DaiPipeline pipeline);
API DaiNode dai_pipeline_create_node_by_name(DaiPipeline pipeline, const char* name);
// Output/Input helpers
API DaiOutput dai_node_get_output(DaiNode node, const char* group, const char* name);
API DaiInput dai_node_get_input(DaiNode node, const char* group, const char* name);
API bool dai_output_link(DaiOutput from, DaiNode to, const char* in_group, const char* in_name);
API bool dai_output_link_input(DaiOutput from, DaiInput to);
API bool dai_node_link(DaiNode from, const char* out_group, const char* out_name, DaiNode to, const char* in_group, const char* in_name);
API bool dai_node_unlink(DaiNode from, const char* out_group, const char* out_name, DaiNode to, const char* in_group, const char* in_name);

// Host node helpers
API DaiInput dai_hostnode_get_input(DaiNode node, const char* name);
API void dai_hostnode_run_sync_on_host(DaiNode node);
API void dai_hostnode_run_sync_on_device(DaiNode node);
API void dai_hostnode_send_processing_to_pipeline(DaiNode node, bool send);

// Threaded host node helpers
API DaiInput dai_threaded_hostnode_create_input(DaiNode node,
                                                const char* name,
                                                const char* group,
                                                bool blocking,
                                                int queue_size,
                                                bool wait_for_message);
API DaiOutput dai_threaded_hostnode_create_output(DaiNode node,
                                                  const char* name,
                                                  const char* group);
API bool dai_threaded_node_is_running(DaiNode node);

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
API DaiOutput dai_camera_request_full_resolution_output_ex(DaiCameraNode camera, int type, float fps, bool use_highest_resolution);

// Camera configuration / introspection
API bool dai_camera_build(DaiCameraNode camera, int board_socket, int sensor_width, int sensor_height, float sensor_fps);
API int dai_camera_get_board_socket(DaiCameraNode camera);
API uint32_t dai_camera_get_max_width(DaiCameraNode camera);
API uint32_t dai_camera_get_max_height(DaiCameraNode camera);

API void dai_camera_set_sensor_type(DaiCameraNode camera, int sensor_type);
API int dai_camera_get_sensor_type(DaiCameraNode camera);

// Camera pools configuration
API void dai_camera_set_raw_num_frames_pool(DaiCameraNode camera, int num);
API void dai_camera_set_max_size_pool_raw(DaiCameraNode camera, int size);
API void dai_camera_set_isp_num_frames_pool(DaiCameraNode camera, int num);
API void dai_camera_set_max_size_pool_isp(DaiCameraNode camera, int size);
API void dai_camera_set_num_frames_pools(DaiCameraNode camera, int raw, int isp, int outputs);
API void dai_camera_set_max_size_pools(DaiCameraNode camera, int raw, int isp, int outputs);
API void dai_camera_set_outputs_num_frames_pool(DaiCameraNode camera, int num);
API void dai_camera_set_outputs_max_size_pool(DaiCameraNode camera, int size);

API int dai_camera_get_raw_num_frames_pool(DaiCameraNode camera);
API int dai_camera_get_max_size_pool_raw(DaiCameraNode camera);
API int dai_camera_get_isp_num_frames_pool(DaiCameraNode camera);
API int dai_camera_get_max_size_pool_isp(DaiCameraNode camera);
API bool dai_camera_get_outputs_num_frames_pool(DaiCameraNode camera, int* out_num);
API bool dai_camera_get_outputs_max_size_pool(DaiCameraNode camera, size_t* out_size);

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

// Input queue helpers (host node)
API DaiBuffer dai_input_get_buffer(DaiInput input);
API DaiBuffer dai_input_try_get_buffer(DaiInput input);
API DaiImgFrame dai_input_get_img_frame(DaiInput input);
API DaiImgFrame dai_input_try_get_img_frame(DaiInput input);

// Output send helpers (host node)
API void dai_output_send_buffer(DaiOutput output, DaiBuffer buffer);
API void dai_output_send_img_frame(DaiOutput output, DaiImgFrame frame);

// MessageGroup helpers
API DaiMessageGroup dai_message_group_clone(DaiMessageGroup group);
API void dai_message_group_release(DaiMessageGroup group);
API DaiBuffer dai_message_group_get_buffer(DaiMessageGroup group, const char* name);
API DaiImgFrame dai_message_group_get_img_frame(DaiMessageGroup group, const char* name);

// Buffer helpers
API DaiBuffer dai_buffer_new(size_t size);
API void dai_buffer_release(DaiBuffer buffer);
API void dai_buffer_set_data(DaiBuffer buffer, const void* data, size_t len);

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

#ifdef __cplusplus
} // extern "C"
} // namespace dai
#endif
