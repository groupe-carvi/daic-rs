#include "wrapper.h"
#include "depthai/build/version.hpp"
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