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

typedef void* NodeHandle;

typedef void* AssetManagerHandle;

#ifdef __cplusplus
}
#endif