#include "wrapper.h"
#include "depthai/depthai.hpp"
#include <string>

using namespace dai;

DeviceHandle device_create(const char* name) {
    return reinterpret_cast<DeviceHandle>(new Device(std::string(name)));
}

void device_destroy(DeviceHandle handle) {
    auto device = reinterpret_cast<Device*>(handle);
    delete device;
}