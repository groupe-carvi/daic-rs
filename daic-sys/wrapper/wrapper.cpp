#include "wrapper.h"
#include "depthai/build/version.hpp"
#include <cstring>
#include <cstdlib>
#include <string>
#include <memory>
using namespace dai;

// Global error storage
static std::string last_error = "";
namespace daic {

const char* dai_build_version() {
    return build::VERSION;
}
const int dai_build_version_major() {
    return build::VERSION_MAJOR;
}
const int dai_build_version_minor() {
    return build::VERSION_MINOR;
}
const int dai_build_version_patch() {
    return build::VERSION_PATCH;
}
const char* dai_build_pre_release_type() {
    return build::PRE_RELEASE_TYPE;
}
const int dai_build_pre_release_version() {
    return build::PRE_RELEASE_VERSION;
}
const char* dai_build_commit() {
    return build::COMMIT;
}
const char* dai_build_commit_datetime() {
    return build::COMMIT_DATETIME;
}
const char* dai_build_build_datetime() {
    return build::BUILD_DATETIME;
}
const char* dai_build_device_version() {
    return build::DEVICE_VERSION;
}
const char* dai_build_bootloader_version() {
    return build::BOOTLOADER_VERSION;
}
const char* dai_build_device_rvc3_version() {
    return build::DEVICE_RVC3_VERSION;
}
const char* dai_build_device_rvc4_version() {
    return build::DEVICE_RVC4_VERSION;
}
// Helper function to convert std::string to C string (caller must free)
char* dai_string_to_cstring(const char* std_string) {
    if (!std_string) return nullptr;
    
    size_t len = strlen(std_string);
    char* result = static_cast<char*>(malloc(len + 1));
    if (result) {
        strcpy(result, std_string);
    }
    return result;
}

void dai_free_cstring(char* cstring) {
    if (cstring) {
        free(cstring);
    }
}

// Device management
DeviceHandle device_create() {
    try {
        dai_clear_last_error();
        auto device = new Device();
        return static_cast<DeviceHandle>(device);
    } catch (const std::exception& e) {
        last_error = std::string("device_create failed: ") + e.what();
        return nullptr;
    }
}

void device_destroy(DeviceHandle handle) {
    if (handle) {
        auto device = static_cast<Device*>(handle);
        delete device;
    }
}

bool device_is_connected(DeviceHandle handle) {
    if (!handle) {
        last_error = "device_is_connected: null device handle";
        return false;
    }
    try {
        auto device = static_cast<Device*>(handle);
        return device->isClosed() == false; // Device is connected if not closed
    } catch (const std::exception& e) {
        last_error = std::string("device_is_connected failed: ") + e.what();
        return false;
    }
}

// Pipeline management
PipelineHandle pipeline_create() {
    try {
        dai_clear_last_error();
        auto pipeline = new Pipeline();
        return static_cast<PipelineHandle>(pipeline);
    } catch (const std::exception& e) {
        last_error = std::string("pipeline_create failed: ") + e.what();
        return nullptr;
    }
}

void pipeline_destroy(PipelineHandle handle) {
    if (handle) {
        auto pipeline = static_cast<Pipeline*>(handle);
        delete pipeline;
    }
}

bool pipeline_start(PipelineHandle handle, DeviceHandle device) {
    if (!handle || !device) {
        last_error = "pipeline_start: null handle(s)";
        return false;
    }
    try {
        auto pipeline = static_cast<Pipeline*>(handle);
        auto dev = static_cast<Device*>(device);
        dev->startPipeline(*pipeline);
        return true;
    } catch (const std::exception& e) {
        last_error = std::string("pipeline_start failed: ") + e.what();
        return false;
    }
}

void pipeline_stop(PipelineHandle handle) {
    // Note: In DepthAI, stopping is usually handled at device level
    // This is a placeholder for pipeline-specific stop operations
}

bool pipeline_is_running(PipelineHandle handle) {
    if (!handle) {
        last_error = "pipeline_is_running: null handle";
        return false;
    }
    // Note: This is a simplified implementation
    // In practice, you'd check the device state
    return true;
}

// Error handling
const char* dai_get_last_error() {
    return last_error.c_str();
}

void dai_clear_last_error() {
    last_error.clear();
}

} // namespace daic