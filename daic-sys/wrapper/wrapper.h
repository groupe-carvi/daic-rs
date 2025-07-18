#pragma once

#include "depthai/depthai.hpp"

#ifdef _WIN32
#define API __declspec(dllexport)
#else
#define API
#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef void* DeviceHandle;

API DeviceHandle device_create();
API void device_destroy(DeviceHandle handle);

typedef void* PipelineHandle;

API PipelineHandle pipeline_create(DeviceHandle device);
API void pipeline_destroy(PipelineHandle handle);

typedef void* CameraHandle;
API CameraHandle camera_create(DeviceHandle device);
API void camera_destroy(CameraHandle handle);

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


#ifdef __cplusplus
}
#endif