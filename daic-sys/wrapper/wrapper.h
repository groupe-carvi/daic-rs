
#pragma once
#include "depthai/depthai.hpp"
#include "XLink/XLink.h"
#include "XLink/XLinkPublicDefines.h"
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

// Helper function to convert std::string to C string (caller must free)
API char* dai_string_to_cstring(const char* std_string);
API void dai_free_cstring(char* cstring);

// Better string handling functions
API char* dai_std_string_to_cstring(const std::string& str);
API void dai_std_string_destroy(const std::string* str);
API std::string* dai_create_std_string(const char* cstr);
API const char* dai_std_string_c_str(const std::string* str);
API size_t dai_std_string_length(const std::string* str);

// XLink types exposure - these types are already defined in XLink headers
// We just re-expose them to ensure they're available in bindings
typedef XLinkError_t XLinkError;
typedef XLinkProtocol_t XLinkProtocol;  
typedef XLinkPlatform_t XLinkPlatform;
typedef XLinkDeviceState_t XLinkDeviceState;
typedef deviceDesc_t DeviceDesc;
API char* dai_string_to_cstring(const char* std_string);
API void dai_free_cstring(char* cstring);

// Low-level raw pointer types - direct exposure of C++ objects
typedef void* DaiDevice;        // Raw dai::Device*
typedef void* DaiPipeline;      // Raw dai::Pipeline*
typedef void* DaiNode;          // Raw dai::Node* (actually points to a derived dai::node::*)
typedef void* DaiCameraNode;    // Raw dai::node::Camera*
typedef void* DaiOutput;        // Raw dai::Node::Output*
typedef void* DaiDataQueue;     // Raw dai::DataOutputQueue*
typedef void* DaiImgFrame;      // Raw dai::ImgFrame*

// Low-level device operations
API DaiDevice dai_device_new();
API void dai_device_delete(DaiDevice device);
API bool dai_device_is_closed(DaiDevice device);
API void dai_device_close(DaiDevice device);

// Low-level pipeline operations  
API DaiPipeline dai_pipeline_new();
API void dai_pipeline_delete(DaiPipeline pipeline);
API bool dai_pipeline_start(DaiPipeline pipeline, DaiDevice device);

// Generic node creation / linking
// Note: `DaiNode` is an erased node pointer; it must originate from the same pipeline.
// kind values are defined by the Rust-side `NodeKind` enum.
API DaiNode dai_pipeline_create_node(DaiPipeline pipeline, int kind);
API bool dai_node_link(DaiNode from, const char* out_group, const char* out_name, DaiNode to, const char* in_group, const char* in_name);
API bool dai_node_unlink(DaiNode from, const char* out_group, const char* out_name, DaiNode to, const char* in_group, const char* in_name);

// Low-level camera node operations
API DaiCameraNode dai_pipeline_create_camera(DaiPipeline pipeline, int board_socket);

// Camera output wrappers
API dai::Node::Output* dai_camera_request_output(DaiCameraNode camera, int width, int height, int type, int resize_mode, float fps, int enable_undistortion);
API dai::Node::Output* dai_camera_request_output_capability(DaiCameraNode camera, const dai::Capability* capability, int on_host);
API dai::Node::Output* dai_camera_request_full_resolution_output(DaiCameraNode camera);

// Low-level output operations
API DaiDataQueue dai_output_create_queue(DaiOutput output, unsigned int max_size, bool blocking);

// Low-level queue operations
API void dai_queue_delete(DaiDataQueue queue);
API DaiImgFrame dai_queue_get_frame(DaiDataQueue queue, int timeout_ms);
API DaiImgFrame dai_queue_try_get_frame(DaiDataQueue queue);

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
