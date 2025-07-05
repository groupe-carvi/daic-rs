#include "wrapper.h"
#include "depthai/depthai.hpp"
#include <memory>
#include <string>

using namespace dai;

DeviceHandle device_create(){
     try {
        Device* device = new Device(); // Create a new Device instance
        return reinterpret_cast<DeviceHandle>(device); // Return as DeviceHandle
    } catch(const std::exception& e) {
        // Handle exceptions if needed
        return nullptr; // or handle error appropriately
    }
}

void device_destroy(DeviceHandle handle) {
    auto device = reinterpret_cast<Device*>(handle);
    delete device;
}