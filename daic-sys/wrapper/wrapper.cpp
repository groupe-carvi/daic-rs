#include "wrapper.h"
#include "depthai/build/version.hpp"
#include <cstring>
#include <cstdlib>
using namespace dai::node;

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