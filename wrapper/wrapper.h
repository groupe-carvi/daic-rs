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

API DeviceHandle device_create(const char* name);
API void device_destroy(DeviceHandle handle);

#ifdef __cplusplus
}
#endif