#include "wrapper.h"
#include "depthai/depthai.hpp"
#include <string>

using namespace dai;

DeviceHandle device_create(const char* name) {
    try {
        if(name == nullptr) {
            return reinterpret_cast<DeviceHandle>(new Device());
        }
        return reinterpret_cast<DeviceHandle>(new Device(std::string(name)));
    } catch(const std::exception& e) {
        // Handle exceptions if needed
        return nullptr; // or handle error appropriately
    }
}

void device_destroy(DeviceHandle handle) {
    auto device = reinterpret_cast<Device*>(handle);
    delete device;
}