// depthai_core_wrapper.h
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef void* DeviceHandle;

DeviceHandle device_create(const char* name);
void device_start(DeviceHandle handle);
void device_destroy(DeviceHandle handle);

#ifdef __cplusplus
}
#endif