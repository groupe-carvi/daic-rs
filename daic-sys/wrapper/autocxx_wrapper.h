#pragma once

#include <cstddef>  // for size_t
#include <cstdint>  // for standard integer types

// Forward declarations only - don't include depthai.hpp to avoid nlohmann/json parsing issues
// This header is specifically for autocxx which only needs function signatures, not full implementations

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

// Low-level device operations - use void* directly
API void* dai_device_new();
API void dai_device_delete(void* device);
API bool dai_device_is_closed(void* device);
API void dai_device_close(void* device);

// Low-level pipeline operations  
API void* dai_pipeline_new();
API void dai_pipeline_delete(void* pipeline);
API bool dai_pipeline_start(void* pipeline, void* device);

// Low-level camera node operations
API void* dai_pipeline_create_camera(void* pipeline, int board_socket);
API void* dai_camera_request_output(void* camera, int width, int height, int type, int resize_mode, float fps, int enable_undistortion);
API void* dai_camera_request_full_resolution_output(void* camera);

// Low-level output operations
API void* dai_output_create_queue(void* output, unsigned int max_size, bool blocking);

// Low-level queue operations
API void dai_queue_delete(void* queue);
API void* dai_queue_get_frame(void* queue, int timeout_ms);
API void* dai_queue_try_get_frame(void* queue);

// Low-level frame operations
API void* dai_frame_get_data(void* frame);
API int dai_frame_get_width(void* frame);
API int dai_frame_get_height(void* frame);
API int dai_frame_get_type(void* frame);
API size_t dai_frame_get_size(void* frame);
API void dai_frame_release(void* frame);

// Low-level utility functions
API int dai_device_get_connected_camera_sockets(void* device, int* sockets, int max_count);
API const char* dai_camera_socket_name(int socket);

// Error handling
API const char* dai_get_last_error();
API void dai_clear_last_error();

} // namespace daic

#ifdef __cplusplus
}
#endif
