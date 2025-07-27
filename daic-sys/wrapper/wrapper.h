
#pragma once
#include "depthai/depthai.hpp"
#ifdef _WIN32
#define API __declspec(dllexport)
#else
#define API
#endif

#ifdef __cplusplus
extern "C" {
#endif

// Version informations getters
API const char* dai_build_version();
API const int dai_build_version_major();
API const int dai_build_version_minor();
API const int dai_build_version_patch();
API const char* dai_build_pre_release_type();
API const int dai_build_pre_release_version();
API const char* dai_build_commit();
API const char* dai_build_commit_datetime();
API const char* dai_build_build_datetime();
API const char* dai_build_device_version();
API const char* dai_build_bootloader_version();
API const char* dai_build_device_rvc3_version();
API const char* dai_build_device_rvc4_version();

// Helper function to convert std::string to C string (caller must free)
API char* dai_string_to_cstring(const char* std_string);
API void dai_free_cstring(char* cstring);


#ifdef __cplusplus
}
#endif