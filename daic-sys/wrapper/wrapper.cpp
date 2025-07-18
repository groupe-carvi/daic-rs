#include "wrapper.h"
#include "depthai/build/version.hpp"
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
        std::shared_ptr<Device> device = std::shared_ptr<Device>(reinterpret_cast<Device*>(deviceHandle));
        Pipeline* pipeline = new Pipeline(device); // Create a new Pipeline instance
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
        std::shared_ptr<Device> device = std::shared_ptr<Device>(reinterpret_cast<Device*>(deviceHandle));
        Camera* camera = new Camera(device); // Create a new Camera instance
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

const char* dai_build_version() {
    return dai::build::VERSION;
}
const int dai_build_version_major() {
    return dai::build::VERSION_MAJOR;
}
const int dai_build_version_minor() {
    return dai::build::VERSION_MINOR;
}
const int dai_build_version_patch() {
    return dai::build::VERSION_PATCH;
}
const char* dai_build_pre_release_type() {
    return dai::build::PRE_RELEASE_TYPE;
}
const int dai_build_pre_release_version() {
    return dai::build::PRE_RELEASE_VERSION;
}
const char* dai_build_commit() {
    return dai::build::COMMIT;
}
const char* dai_build_commit_datetime() {
    return dai::build::COMMIT_DATETIME;
}
const char* dai_build_build_datetime() {
    return dai::build::BUILD_DATETIME;
}
const char* dai_build_device_version() {
    return dai::build::DEVICE_VERSION;
}
const char* dai_build_bootloader_version() {
    return dai::build::BOOTLOADER_VERSION;
}
const char* dai_build_device_rvc3_version() {
    return dai::build::DEVICE_RVC3_VERSION;
}
const char* dai_build_device_rvc4_version() {
    return dai::build::DEVICE_RVC4_VERSION;
}