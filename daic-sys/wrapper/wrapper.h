
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
API const int dai_build_version_major();
API const int dai_build_version_minor();
API const int dai_build_version_patch();
API const char* dai_build_pre_release_type();
API const int dai_build_pre_release_version();
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

// XLink types exposure - these types are already defined in XLink headers
// We just re-expose them to ensure they're available in bindings
typedef XLinkError_t XLinkError;
typedef XLinkProtocol_t XLinkProtocol;  
typedef XLinkPlatform_t XLinkPlatform;
typedef XLinkDeviceState_t XLinkDeviceState;
typedef deviceDesc_t DeviceDesc;
API char* dai_string_to_cstring(const char* std_string);
API void dai_free_cstring(char* cstring);

// Device management
typedef void* DeviceHandle;
API DeviceHandle device_create();
API void device_destroy(DeviceHandle handle);
API bool device_is_connected(DeviceHandle handle);

// Pipeline management
typedef void* PipelineHandle;
API PipelineHandle pipeline_create();
API void pipeline_destroy(PipelineHandle handle);
API bool pipeline_start(PipelineHandle handle, DeviceHandle device);
API void pipeline_stop(PipelineHandle handle);
API bool pipeline_is_running(PipelineHandle handle);

// Error handling
API const char* dai_get_last_error();
API void dai_clear_last_error();

} // namespace daic


#ifdef __cplusplus
}
#endif