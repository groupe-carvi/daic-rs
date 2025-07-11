#include "wrapper.h"
#include <memory>
#include <string>

using namespace dai;
using namespace dai::node;

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

PipelineHandle pipeline_create(DeviceHandle deviceHandle) {
    try {
        auto device = reinterpret_cast<Device*>(deviceHandle);
        Pipeline* pipeline = new Pipeline(*device); // Create a new Pipeline instance
        return reinterpret_cast<PipelineHandle>(pipeline); // Return as PipelineHandle
    } catch(const std::exception& e) {
        // Handle exceptions if needed
        return nullptr; // or handle error appropriately
    }
}

void pipeline_destroy(PipelineHandle handle) {
    auto pipeline = reinterpret_cast<Pipeline*>(handle);
    delete pipeline;
}

CameraHandle camera_create(DeviceHandle deviceHandle) {
    try {
        auto device = reinterpret_cast<Device*>(deviceHandle);
        Camera* camera = new Camera(*device); // Create a new Camera instance
        return reinterpret_cast<CameraHandle>(camera); // Return as CameraHandle
    } catch(const std::exception& e) {
        // Handle exceptions if needed
        return nullptr; // or handle error appropriately
    }
}

void camera_destroy(CameraHandle handle) {
    auto camera = reinterpret_cast<Camera*>(handle);
    delete camera;
}