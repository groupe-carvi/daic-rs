#pragma once

// Include all the depthai headers we need
#include "depthai/depthai.hpp"
#include "depthai/build/version.hpp"
#include "XLink/XLink.h"
#include "XLink/XLinkPublicDefines.h"

// We'll use autocxx to generate bindings for these types and functions
// autocxx can handle C++ types directly, including std::string, std::vector, etc.

namespace daic {
    // Version information getters - simple functions that return const char*
    inline const char* get_build_version() { return dai::build::VERSION; }
    inline int get_version_major() { return dai::build::VERSION_MAJOR; }
    inline int get_version_minor() { return dai::build::VERSION_MINOR; }
    inline int get_version_patch() { return dai::build::VERSION_PATCH; }
    inline const char* get_pre_release_type() { return dai::build::PRE_RELEASE_TYPE; }
    inline int get_pre_release_version() { return dai::build::PRE_RELEASE_VERSION; }
    inline const char* get_commit() { return dai::build::COMMIT; }
    inline const char* get_commit_datetime() { return dai::build::COMMIT_DATETIME; }
    inline const char* get_build_datetime() { return dai::build::BUILD_DATETIME; }
    inline const char* get_device_version() { return dai::build::DEVICE_VERSION; }
    inline const char* get_bootloader_version() { return dai::build::BOOTLOADER_VERSION; }
    inline const char* get_device_rvc3_version() { return dai::build::DEVICE_RVC3_VERSION; }
    inline const char* get_device_rvc4_version() { return dai::build::DEVICE_RVC4_VERSION; }
}
