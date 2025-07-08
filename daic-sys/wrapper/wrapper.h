#pragma once

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

#ifdef __cplusplus
}
#endif