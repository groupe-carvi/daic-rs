#include "wrapper.h"
#include "depthai/depthai.hpp"
#include "depthai/pipeline/node/internal/XLinkIn.hpp"
#include "depthai/pipeline/node/internal/XLinkOut.hpp"
#include "depthai/build/version.hpp"
#include "depthai/common/Point3fRGBA.hpp"
#include "depthai/pipeline/datatype/PointCloudData.hpp"
#include "depthai/pipeline/datatype/RGBDData.hpp"
#include "XLink/XLink.h"
#include "XLink/XLinkPublicDefines.h"
#include <chrono>
#include <cstring>
#include <cstdlib>
#include <limits>
#include <memory>
#include <mutex>
#include <string>
#include <unordered_map>
#include <functional>

// Global error storage
static std::string last_error = "";

// Device lifetime management
//
// DepthAI devices generally represent an exclusive connection. Creating multiple `dai::Device()`
// instances without selecting distinct physical devices can fail with:
//   "No available devices (1 connected, but in use)"
//
// The C++ API commonly passes around shared pointers to a single selected device.
// To mirror that behavior across the C ABI, we represent `DaiDevice` as a pointer to a
// heap-allocated `std::shared_ptr<dai::Device>`.
//
// We also keep a process-wide default device which `dai_device_new()` returns (or creates).
static std::mutex g_device_mutex;
static std::weak_ptr<dai::Device> g_default_device;

// Some XLink versions/platforms can report device state as X_LINK_ANY_STATE when queried with
// X_LINK_ANY_STATE, which breaks DepthAI's "find any available device" logic.
// To be more robust in our C ABI, we query per concrete state in priority order and then
// construct `dai::Device` from the returned `DeviceInfo`.
static bool select_first_device_info(dai::DeviceInfo& out) {
    // Prefer devices that can be booted/connected immediately.
    const XLinkDeviceState_t states[] = {
        X_LINK_UNBOOTED,
        X_LINK_BOOTLOADER,
        X_LINK_FLASH_BOOTED,
        X_LINK_GATE,
        X_LINK_GATE_SETUP,
        X_LINK_BOOTED,
    };

    for(const auto state : states) {
        try {
            auto devices = dai::XLinkConnection::getAllConnectedDevices(state, /*skipInvalidDevices=*/true);
            if(!devices.empty()) {
                out = devices.front();
                return true;
            }
        } catch(...) {
            // Ignore and continue to next state.
        }
    }
    return false;
}

namespace dai {

const char* dai_build_version() {
    return dai::build::VERSION;
}
int dai_build_version_major() {
    return dai::build::VERSION_MAJOR;
}
int dai_build_version_minor() {
    return dai::build::VERSION_MINOR;
}
int dai_build_version_patch() {
    return dai::build::VERSION_PATCH;
}
const char* dai_build_pre_release_type() {
    return dai::build::PRE_RELEASE_TYPE;
}
int dai_build_pre_release_version() {
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

// Basic string utilities
char* dai_string_to_cstring(const char* str) {
    if(!str) return nullptr;

    size_t len = strlen(str);
    char* result = static_cast<char*>(malloc(len + 1));
    if(result) {
        strcpy(result, str);
    }
    return result;
}

void dai_free_cstring(char* cstring) {
    if (cstring) {
        free(cstring);
    }
}

// Low-level device operations - direct pointer manipulation
DaiDevice dai_device_new() {
    try {
        dai_clear_last_error();
        std::lock_guard<std::mutex> lock(g_device_mutex);

        // Reuse existing default device if it is still alive and not closed.
        if(auto existing = g_default_device.lock()) {
            try {
                if(!existing->isClosed()) {
                    return static_cast<DaiDevice>(new std::shared_ptr<dai::Device>(existing));
                }
            } catch(...) {
                // If isClosed throws for some reason, fall back to creating a new device.
            }
        }

        // Create new default device.
        // Instead of calling `dai::Device()` (which internally uses getAnyAvailableDevice),
        // explicitly select a concrete state/device and construct from DeviceInfo.
        dai::DeviceInfo info;
        if(!select_first_device_info(info)) {
            // Mirror DepthAI's wording as closely as possible.
            auto numConnected = dai::DeviceBase::getAllAvailableDevices().size();
            if(numConnected > 0) {
                throw std::runtime_error(std::string("No available devices (") + std::to_string(numConnected) +
                                         " connected, but in use)");
            }
            throw std::runtime_error("No available devices");
        }

        auto created = std::make_shared<dai::Device>(info, dai::DeviceBase::DEFAULT_USB_SPEED);
        g_default_device = created;
        return static_cast<DaiDevice>(new std::shared_ptr<dai::Device>(created));
    } catch (const std::exception& e) {
        last_error = std::string("dai_device_new failed: ") + e.what();
        return nullptr;
    }
}

DaiDevice dai_device_clone(DaiDevice device) {
    if(!device) {
        last_error = "dai_device_clone: null device";
        return nullptr;
    }
    try {
        auto ptr = static_cast<std::shared_ptr<dai::Device>*>(device);
        return static_cast<DaiDevice>(new std::shared_ptr<dai::Device>(*ptr));
    } catch (const std::exception& e) {
        last_error = std::string("dai_device_clone failed: ") + e.what();
        return nullptr;
    }
}

void dai_device_delete(DaiDevice device) {
    if (device) {
        auto dev = static_cast<std::shared_ptr<dai::Device>*>(device);
        // If this is the last strong reference, proactively close the device.
        // Some DepthAI backends can otherwise keep the device marked as "in use"
        // for longer than expected.
        try {
            if(dev->use_count() == 1 && dev->get() && (*dev) && !(*dev)->isClosed()) {
                (*dev)->close();
            }
        } catch(...) {
            // Best-effort: proceed with deletion.
        }
        delete dev;
    }
}

bool dai_device_is_closed(DaiDevice device) {
    if (!device) {
        last_error = "dai_device_is_closed: null device";
        return true;
    }
    try {
        auto dev = static_cast<std::shared_ptr<dai::Device>*>(device);
        if(!dev->get() || !(*dev)) return true;
        return (*dev)->isClosed();
    } catch (const std::exception& e) {
        last_error = std::string("dai_device_is_closed failed: ") + e.what();
        return true;
    }
}

void dai_device_close(DaiDevice device) {
    if (!device) {
        last_error = "dai_device_close: null device";
        return;
    }
    try {
        auto dev = static_cast<std::shared_ptr<dai::Device>*>(device);
        if(!dev->get() || !(*dev)) {
            last_error = "dai_device_close: invalid device";
            return;
        }
        (*dev)->close();
    } catch (const std::exception& e) {
        last_error = std::string("dai_device_close failed: ") + e.what();
    }
}

// Low-level pipeline operations
DaiPipeline dai_pipeline_new() {
    try {
        dai_clear_last_error();
        // Use the constructor that doesn't require a device.
        // dai::Pipeline() in C++ is just a graph container.
        // printf("DEBUG: Creating dai::Pipeline...\n");
        auto pipeline = new dai::Pipeline();
        // printf("DEBUG: dai::Pipeline created at %p\n", pipeline);
        return static_cast<DaiPipeline>(pipeline);
    } catch (const std::exception& e) {
        // printf("DEBUG: dai::Pipeline creation failed: %s\n", e.what());
        last_error = std::string("dai_pipeline_new failed: ") + e.what();
        return nullptr;
    }
}

DaiPipeline dai_pipeline_new_with_device(DaiDevice device) {
    if(!device) {
        last_error = "dai_pipeline_new_with_device: null device";
        return nullptr;
    }
    try {
        dai_clear_last_error();
        auto dev = static_cast<std::shared_ptr<dai::Device>*>(device);
        if(!dev->get() || !(*dev)) {
            last_error = "dai_pipeline_new_with_device: invalid device";
            return nullptr;
        }
        auto pipeline = new dai::Pipeline(*dev);
        return static_cast<DaiPipeline>(pipeline);
    } catch (const std::exception& e) {
        last_error = std::string("dai_pipeline_new_with_device failed: ") + e.what();
        return nullptr;
    }
}

DaiNode dai_rgbd_build(DaiNode rgbd) {
    if(!rgbd) {
        last_error = "dai_rgbd_build: null rgbd";
        return nullptr;
    }
    try {
        dai_clear_last_error();
        auto node = static_cast<dai::node::RGBD*>(rgbd);
        auto built = node->build();
        return static_cast<DaiNode>(built.get());
    } catch(const std::exception& e) {
        last_error = std::string("dai_rgbd_build failed: ") + e.what();
        return nullptr;
    }
}

void dai_pipeline_delete(DaiPipeline pipeline) {
    if (pipeline) {
        auto pipe = static_cast<dai::Pipeline*>(pipeline);
        delete pipe;
    }
}

bool dai_pipeline_start(DaiPipeline pipeline) {
    if(!pipeline) {
        last_error = "dai_pipeline_start: null pipeline";
        return false;
    }
    try {
        auto pipe = static_cast<dai::Pipeline*>(pipeline);
        pipe->start();
        return true;
    } catch (const std::exception& e) {
        last_error = std::string("dai_pipeline_start failed: ") + e.what();
        return false;
    }
}

// Backwards-compatible alias. Historically the Rust wrapper exposed `start_default()`,
// but DepthAI's `dai::Pipeline` already manages a default device internally.
bool dai_pipeline_start_default(DaiPipeline pipeline) {
    return dai_pipeline_start(pipeline);
}

DaiDevice dai_pipeline_get_default_device(DaiPipeline pipeline) {
    if(!pipeline) {
        last_error = "dai_pipeline_get_default_device: null pipeline";
        return nullptr;
    }
    try {
        auto pipe = static_cast<dai::Pipeline*>(pipeline);
        auto dev = pipe->getDefaultDevice();
        if(!dev) {
            last_error = "dai_pipeline_get_default_device: pipeline has no default device";
            return nullptr;
        }
        return static_cast<DaiDevice>(new std::shared_ptr<dai::Device>(std::move(dev)));
    } catch (const std::exception& e) {
        last_error = std::string("dai_pipeline_get_default_device failed: ") + e.what();
        return nullptr;
    }
}

// Generic node creation / linking
using NodeCreator = std::function<dai::Node*(dai::Pipeline*)>;

#define REGISTER_NODE(name) registry[#name] = [](dai::Pipeline* p) { return p->create<name>().get(); }

static std::unordered_map<std::string, NodeCreator>& get_node_registry() {
    static std::unordered_map<std::string, NodeCreator> registry;
    if (registry.empty()) {
        REGISTER_NODE(dai::node::Camera);
        REGISTER_NODE(dai::node::ColorCamera);
        REGISTER_NODE(dai::node::MonoCamera);
        REGISTER_NODE(dai::node::StereoDepth);
        REGISTER_NODE(dai::node::ImageAlign);
        REGISTER_NODE(dai::node::RGBD);
        REGISTER_NODE(dai::node::VideoEncoder);
        REGISTER_NODE(dai::node::NeuralNetwork);
        REGISTER_NODE(dai::node::ImageManip);
        REGISTER_NODE(dai::node::Script);
        REGISTER_NODE(dai::node::SystemLogger);
        REGISTER_NODE(dai::node::SpatialLocationCalculator);
        REGISTER_NODE(dai::node::FeatureTracker);
        REGISTER_NODE(dai::node::ObjectTracker);
        REGISTER_NODE(dai::node::IMU);
        REGISTER_NODE(dai::node::EdgeDetector);
        REGISTER_NODE(dai::node::Warp);
        REGISTER_NODE(dai::node::AprilTag);
        REGISTER_NODE(dai::node::DetectionParser);
        REGISTER_NODE(dai::node::PointCloud);
        REGISTER_NODE(dai::node::Sync);
        REGISTER_NODE(dai::node::ToF);
        REGISTER_NODE(dai::node::UVC);
        REGISTER_NODE(dai::node::DetectionNetwork);
        REGISTER_NODE(dai::node::SpatialDetectionNetwork);
        REGISTER_NODE(dai::node::BenchmarkIn);
        REGISTER_NODE(dai::node::BenchmarkOut);
        REGISTER_NODE(dai::node::Rectification);
        REGISTER_NODE(dai::node::MessageDemux);
        REGISTER_NODE(dai::node::NeuralDepth);
        REGISTER_NODE(dai::node::SPIIn);
        REGISTER_NODE(dai::node::SPIOut);
        REGISTER_NODE(dai::node::Thermal);

        // XLink nodes are in internal namespace but we expose them as dai::node::XLinkIn/Out
        registry["dai::node::XLinkIn"] = [](dai::Pipeline* p) { return p->create<dai::node::internal::XLinkIn>().get(); };
        registry["dai::node::XLinkOut"] = [](dai::Pipeline* p) { return p->create<dai::node::internal::XLinkOut>().get(); };
    }
    return registry;
}

DaiNode dai_pipeline_create_node_by_name(DaiPipeline pipeline, const char* name) {
    if (!pipeline || !name) {
        last_error = "dai_pipeline_create_node_by_name: null pipeline or name";
        return nullptr;
    }
    try {
        auto pipe = static_cast<dai::Pipeline*>(pipeline);
        auto& registry = get_node_registry();
        auto it = registry.find(name);
        if (it != registry.end()) {
            return static_cast<DaiNode>(it->second(pipe));
        }
        
        last_error = std::string("dai_pipeline_create_node_by_name: unknown node name: ") + name;
        return nullptr;
    } catch (const std::exception& e) {
        last_error = std::string("dai_pipeline_create_node_by_name failed: ") + e.what();
        return nullptr;
    }
}

// Forward declarations for helpers defined later in this file.
static inline bool _dai_cstr_empty(const char* s);
static inline dai::Node::Input* _dai_pick_input_for_output(dai::Node* toNode, dai::Node::Output* output, const char* in_group);

DaiOutput dai_node_get_output(DaiNode node, const char* group, const char* name) {
    if(!node) {
        last_error = "dai_node_get_output: null node";
        return nullptr;
    }
    if(_dai_cstr_empty(name)) {
        last_error = "dai_node_get_output: empty name";
        return nullptr;
    }
    try {
        auto n = static_cast<dai::Node*>(node);
        dai::Node::Output* out = group ? n->getOutputRef(std::string(group), std::string(name)) : n->getOutputRef(std::string(name));
        if(!out) {
            last_error = "dai_node_get_output: output not found";
            return nullptr;
        }
        return static_cast<DaiOutput>(out);
    } catch(const std::exception& e) {
        last_error = std::string("dai_node_get_output failed: ") + e.what();
        return nullptr;
    }
}

DaiInput dai_node_get_input(DaiNode node, const char* group, const char* name) {
    if(!node) {
        last_error = "dai_node_get_input: null node";
        return nullptr;
    }
    if(_dai_cstr_empty(name)) {
        last_error = "dai_node_get_input: empty name";
        return nullptr;
    }
    try {
        auto n = static_cast<dai::Node*>(node);
        dai::Node::Input* in = group ? n->getInputRef(std::string(group), std::string(name)) : n->getInputRef(std::string(name));
        if(!in) {
            last_error = "dai_node_get_input: input not found";
            return nullptr;
        }
        return static_cast<DaiInput>(in);
    } catch(const std::exception& e) {
        last_error = std::string("dai_node_get_input failed: ") + e.what();
        return nullptr;
    }
}

bool dai_output_link(DaiOutput from, DaiNode to, const char* in_group, const char* in_name) {
    if(!from || !to) {
        last_error = "dai_output_link: null from/to";
        return false;
    }
    try {
        auto out = static_cast<dai::Node::Output*>(from);
        auto toNode = static_cast<dai::Node*>(to);

        const bool inSpecified = !_dai_cstr_empty(in_name);
        dai::Node::Input* input = nullptr;

        if(inSpecified) {
            const std::string inNameStr(in_name);
            const std::optional<std::string> inGroupStr = in_group ? std::optional<std::string>(std::string(in_group)) : std::nullopt;

            auto try_find_on_node = [&](dai::Node* n) -> dai::Node::Input* {
                if(!n) return nullptr;

                // Most nodes expose their inputs directly via getInputRef(name).
                if(inGroupStr.has_value()) {
                    if(auto* i = n->getInputRef(inGroupStr.value(), inNameStr)) return i;
                }
                if(auto* i = n->getInputRef(inNameStr)) return i;

                // Some nodes (e.g. Sync-based host nodes) keep dynamic inputs under an InputMap named "inputs".
                // When callers don't specify a group, try that common map name as a fallback.
                if(!inGroupStr.has_value()) {
                    if(auto* i = n->getInputRef(std::string("inputs"), inNameStr)) return i;
                }

                return nullptr;
            };

            // First try on the target node itself.
            input = try_find_on_node(toNode);

            // If not found, try any subnodes (e.g. RGBD -> Sync subnode).
            if(!input) {
                for(const auto& child : toNode->getNodeMap()) {
                    input = try_find_on_node(child.get());
                    if(input) break;
                }
            }

            if(!input) {
                last_error = "dai_output_link: input not found";
                return false;
            }
        } else {
            input = _dai_pick_input_for_output(toNode, out, in_group);
        }

        if(!input) {
            last_error = "dai_output_link: no compatible input found";
            return false;
        }
        out->link(*input);
        return true;
    } catch(const std::exception& e) {
        last_error = std::string("dai_output_link failed: ") + e.what();
        return false;
    }
}

bool dai_output_link_input(DaiOutput from, DaiInput to) {
    if(!from || !to) {
        last_error = "dai_output_link_input: null from/to";
        return false;
    }
    try {
        auto out = static_cast<dai::Node::Output*>(from);
        auto in = static_cast<dai::Node::Input*>(to);
        out->link(*in);
        return true;
    } catch(const std::exception& e) {
        last_error = std::string("dai_output_link_input failed: ") + e.what();
        return false;
    }
}

int dai_device_get_platform(DaiDevice device) {
    if(!device) {
        last_error = "dai_device_get_platform: null device";
        return -1;
    }
    try {
        auto dev = static_cast<std::shared_ptr<dai::Device>*>(device);
        if(!dev->get() || !(*dev)) {
            last_error = "dai_device_get_platform: invalid device";
            return -1;
        }
        return static_cast<int>((*dev)->getPlatform());
    } catch(const std::exception& e) {
        last_error = std::string("dai_device_get_platform failed: ") + e.what();
        return -1;
    }
}

void dai_device_set_ir_laser_dot_projector_intensity(DaiDevice device, float intensity) {
    if(!device) {
        last_error = "dai_device_set_ir_laser_dot_projector_intensity: null device";
        return;
    }
    try {
        auto dev = static_cast<std::shared_ptr<dai::Device>*>(device);
        if(!dev->get() || !(*dev)) {
            last_error = "dai_device_set_ir_laser_dot_projector_intensity: invalid device";
            return;
        }
        (*dev)->setIrLaserDotProjectorIntensity(intensity);
    } catch(const std::exception& e) {
        last_error = std::string("dai_device_set_ir_laser_dot_projector_intensity failed: ") + e.what();
    }
}

static inline dai::node::StereoDepth* _dai_as_stereo(DaiNode stereo) {
    return static_cast<dai::node::StereoDepth*>(stereo);
}

void dai_stereo_set_subpixel(DaiNode stereo, bool enable) {
    if(!stereo) {
        last_error = "dai_stereo_set_subpixel: null stereo";
        return;
    }
    try {
        _dai_as_stereo(stereo)->setSubpixel(enable);
    } catch(const std::exception& e) {
        last_error = std::string("dai_stereo_set_subpixel failed: ") + e.what();
    }
}

void dai_stereo_set_extended_disparity(DaiNode stereo, bool enable) {
    if(!stereo) {
        last_error = "dai_stereo_set_extended_disparity: null stereo";
        return;
    }
    try {
        _dai_as_stereo(stereo)->setExtendedDisparity(enable);
    } catch(const std::exception& e) {
        last_error = std::string("dai_stereo_set_extended_disparity failed: ") + e.what();
    }
}

void dai_stereo_set_default_profile_preset(DaiNode stereo, int preset_mode) {
    if(!stereo) {
        last_error = "dai_stereo_set_default_profile_preset: null stereo";
        return;
    }
    try {
        _dai_as_stereo(stereo)->setDefaultProfilePreset(static_cast<dai::node::StereoDepth::PresetMode>(preset_mode));
    } catch(const std::exception& e) {
        last_error = std::string("dai_stereo_set_default_profile_preset failed: ") + e.what();
    }
}

void dai_stereo_set_left_right_check(DaiNode stereo, bool enable) {
    if(!stereo) {
        last_error = "dai_stereo_set_left_right_check: null stereo";
        return;
    }
    try {
        _dai_as_stereo(stereo)->setLeftRightCheck(enable);
    } catch(const std::exception& e) {
        last_error = std::string("dai_stereo_set_left_right_check failed: ") + e.what();
    }
}

void dai_stereo_set_rectify_edge_fill_color(DaiNode stereo, int color) {
    if(!stereo) {
        last_error = "dai_stereo_set_rectify_edge_fill_color: null stereo";
        return;
    }
    try {
        _dai_as_stereo(stereo)->setRectifyEdgeFillColor(color);
    } catch(const std::exception& e) {
        last_error = std::string("dai_stereo_set_rectify_edge_fill_color failed: ") + e.what();
    }
}

void dai_stereo_enable_distortion_correction(DaiNode stereo, bool enable) {
    if(!stereo) {
        last_error = "dai_stereo_enable_distortion_correction: null stereo";
        return;
    }
    try {
        _dai_as_stereo(stereo)->enableDistortionCorrection(enable);
    } catch(const std::exception& e) {
        last_error = std::string("dai_stereo_enable_distortion_correction failed: ") + e.what();
    }
}

void dai_stereo_initial_set_left_right_check_threshold(DaiNode stereo, int threshold) {
    if(!stereo) {
        last_error = "dai_stereo_initial_set_left_right_check_threshold: null stereo";
        return;
    }
    try {
        auto s = _dai_as_stereo(stereo);
        if(!s->initialConfig) {
            last_error = "dai_stereo_initial_set_left_right_check_threshold: initialConfig is null";
            return;
        }
        s->initialConfig->setLeftRightCheckThreshold(threshold);
    } catch(const std::exception& e) {
        last_error = std::string("dai_stereo_initial_set_left_right_check_threshold failed: ") + e.what();
    }
}

void dai_stereo_initial_set_threshold_filter_max_range(DaiNode stereo, int max_range) {
    if(!stereo) {
        last_error = "dai_stereo_initial_set_threshold_filter_max_range: null stereo";
        return;
    }
    try {
        auto s = _dai_as_stereo(stereo);
        if(!s->initialConfig) {
            last_error = "dai_stereo_initial_set_threshold_filter_max_range: initialConfig is null";
            return;
        }
        s->initialConfig->postProcessing.thresholdFilter.maxRange = max_range;
    } catch(const std::exception& e) {
        last_error = std::string("dai_stereo_initial_set_threshold_filter_max_range failed: ") + e.what();
    }
}

void dai_rgbd_set_depth_unit(DaiNode rgbd, int depth_unit) {
    if(!rgbd) {
        last_error = "dai_rgbd_set_depth_unit: null rgbd";
        return;
    }
    try {
        auto r = static_cast<dai::node::RGBD*>(rgbd);
        r->setDepthUnit(static_cast<dai::StereoDepthConfig::AlgorithmControl::DepthUnit>(depth_unit));
    } catch(const std::exception& e) {
        last_error = std::string("dai_rgbd_set_depth_unit failed: ") + e.what();
    }
}

// Wrapper-owned pointcloud view. PointCloudData::getPointsRGB() returns by value, so we
// store the returned vector and expose a stable pointer + length to Rust.
struct DaiPointCloudView {
    std::shared_ptr<dai::PointCloudData> msg;
    std::vector<dai::Point3fRGBA> points;
};

DaiPointCloud dai_queue_get_pointcloud(DaiDataQueue queue, int timeout_ms) {
    if(!queue) {
        last_error = "dai_queue_get_pointcloud: null queue";
        return nullptr;
    }
    try {
        auto ptr = static_cast<std::shared_ptr<dai::MessageQueue>*>(queue);
        std::shared_ptr<dai::PointCloudData> pcl;
        if(timeout_ms < 0) {
            pcl = (*ptr)->get<dai::PointCloudData>();
        } else {
            bool timedOut = false;
            pcl = (*ptr)->get<dai::PointCloudData>(std::chrono::milliseconds(timeout_ms), timedOut);
            if(timedOut) return nullptr;
        }
        if(!pcl) return nullptr;

        auto view = new DaiPointCloudView();
        view->msg = pcl;
        view->points = pcl->getPointsRGB();
        return static_cast<DaiPointCloud>(view);
    } catch(const std::exception& e) {
        last_error = std::string("dai_queue_get_pointcloud failed: ") + e.what();
        return nullptr;
    }
}

DaiPointCloud dai_queue_try_get_pointcloud(DaiDataQueue queue) {
    if(!queue) {
        last_error = "dai_queue_try_get_pointcloud: null queue";
        return nullptr;
    }
    try {
        auto ptr = static_cast<std::shared_ptr<dai::MessageQueue>*>(queue);
        auto pcl = (*ptr)->tryGet<dai::PointCloudData>();
        if(!pcl) return nullptr;
        auto view = new DaiPointCloudView();
        view->msg = pcl;
        view->points = pcl->getPointsRGB();
        return static_cast<DaiPointCloud>(view);
    } catch(const std::exception& e) {
        last_error = std::string("dai_queue_try_get_pointcloud failed: ") + e.what();
        return nullptr;
    }
}

int dai_pointcloud_get_width(DaiPointCloud pcl) {
    if(!pcl) {
        last_error = "dai_pointcloud_get_width: null pointcloud";
        return 0;
    }
    auto view = static_cast<DaiPointCloudView*>(pcl);
    return static_cast<int>(view->msg ? view->msg->getWidth() : 0);
}

int dai_pointcloud_get_height(DaiPointCloud pcl) {
    if(!pcl) {
        last_error = "dai_pointcloud_get_height: null pointcloud";
        return 0;
    }
    auto view = static_cast<DaiPointCloudView*>(pcl);
    return static_cast<int>(view->msg ? view->msg->getHeight() : 0);
}

const DaiPoint3fRGBA* dai_pointcloud_get_points_rgba(DaiPointCloud pcl) {
    if(!pcl) {
        last_error = "dai_pointcloud_get_points_rgba: null pointcloud";
        return nullptr;
    }
    auto view = static_cast<DaiPointCloudView*>(pcl);
    if(view->points.empty()) return nullptr;
    return reinterpret_cast<const DaiPoint3fRGBA*>(view->points.data());
}

size_t dai_pointcloud_get_points_rgba_len(DaiPointCloud pcl) {
    if(!pcl) {
        last_error = "dai_pointcloud_get_points_rgba_len: null pointcloud";
        return 0;
    }
    auto view = static_cast<DaiPointCloudView*>(pcl);
    return view->points.size();
}

void dai_pointcloud_release(DaiPointCloud pcl) {
    if(pcl) {
        auto view = static_cast<DaiPointCloudView*>(pcl);
        delete view;
    }
}

DaiRGBDData dai_queue_get_rgbd(DaiDataQueue queue, int timeout_ms) {
    if(!queue) {
        last_error = "dai_queue_get_rgbd: null queue";
        return nullptr;
    }
    try {
        auto ptr = static_cast<std::shared_ptr<dai::MessageQueue>*>(queue);
        std::shared_ptr<dai::RGBDData> rgbd;
        if(timeout_ms < 0) {
            rgbd = (*ptr)->get<dai::RGBDData>();
        } else {
            bool timedOut = false;
            rgbd = (*ptr)->get<dai::RGBDData>(std::chrono::milliseconds(timeout_ms), timedOut);
            if(timedOut) return nullptr;
        }
        if(!rgbd) return nullptr;
        return static_cast<DaiRGBDData>(new std::shared_ptr<dai::RGBDData>(rgbd));
    } catch(const std::exception& e) {
        last_error = std::string("dai_queue_get_rgbd failed: ") + e.what();
        return nullptr;
    }
}

DaiRGBDData dai_queue_try_get_rgbd(DaiDataQueue queue) {
    if(!queue) {
        last_error = "dai_queue_try_get_rgbd: null queue";
        return nullptr;
    }
    try {
        auto ptr = static_cast<std::shared_ptr<dai::MessageQueue>*>(queue);
        auto rgbd = (*ptr)->tryGet<dai::RGBDData>();
        if(!rgbd) return nullptr;
        return static_cast<DaiRGBDData>(new std::shared_ptr<dai::RGBDData>(rgbd));
    } catch(const std::exception& e) {
        last_error = std::string("dai_queue_try_get_rgbd failed: ") + e.what();
        return nullptr;
    }
}

DaiImgFrame dai_rgbd_get_rgb_frame(DaiRGBDData rgbd) {
    if(!rgbd) {
        last_error = "dai_rgbd_get_rgb_frame: null rgbd";
        return nullptr;
    }
    try {
        auto ptr = static_cast<std::shared_ptr<dai::RGBDData>*>(rgbd);
        auto frame = (*ptr)->getRGBFrame();
        if(!frame) return nullptr;
        return new std::shared_ptr<dai::ImgFrame>(frame);
    } catch(const std::exception& e) {
        last_error = std::string("dai_rgbd_get_rgb_frame failed: ") + e.what();
        return nullptr;
    }
}

DaiImgFrame dai_rgbd_get_depth_frame(DaiRGBDData rgbd) {
    if(!rgbd) {
        last_error = "dai_rgbd_get_depth_frame: null rgbd";
        return nullptr;
    }
    try {
        auto ptr = static_cast<std::shared_ptr<dai::RGBDData>*>(rgbd);
        auto frame = (*ptr)->getDepthFrame();
        if(!frame) return nullptr;
        return new std::shared_ptr<dai::ImgFrame>(frame);
    } catch(const std::exception& e) {
        last_error = std::string("dai_rgbd_get_depth_frame failed: ") + e.what();
        return nullptr;
    }
}

void dai_rgbd_release(DaiRGBDData rgbd) {
    if(rgbd) {
        auto ptr = static_cast<std::shared_ptr<dai::RGBDData>*>(rgbd);
        delete ptr;
    }
}

static inline std::string _dai_opt_cstr(const char* s) {
    return s ? std::string(s) : std::string();
}

static inline bool _dai_cstr_empty(const char* s) {
    return s == nullptr || *s == '\0';
}

static inline int _dai_score_port_name(const std::string& name, bool isOutput) {
    // Heuristic only; compatibility checks decide feasibility.
    // Prefer commonly-used/default ports and avoid raw/metadata ports.
    int score = 0;
    auto has = [&](const char* needle) { return name.find(needle) != std::string::npos; };

    if(name == "out") score += 100;
    if(isOutput) {
        if(has("video")) score += 90;
        if(has("preview")) score += 85;
        if(has("isp")) score += 80;
        if(has("passthrough")) score += 40;
        if(has("rgbd")) score += 70;
        if(has("pcl")) score += 60;
        if(has("depth")) score += 60;
        if(has("raw")) score -= 30;
        if(has("meta")) score -= 20;
        if(has("metadata")) score -= 20;
        if(has("control")) score -= 10;
    } else {
        if(has("input")) score += 80;
        if(has("inColor")) score += 70;
        if(has("inDepth")) score += 70;
        if(name == "in") score += 60;
        if(name == "inSync") score -= 10;
    }
    return score;
}

static inline std::vector<dai::Node::Output*> _dai_collect_outputs(dai::Node* node) {
    std::vector<dai::Node::Output*> outs;
    if(!node) return outs;
    auto refs = node->getOutputRefs();
    outs.insert(outs.end(), refs.begin(), refs.end());
    auto maps = node->getOutputMapRefs();
    for(auto* m : maps) {
        if(!m) continue;
        for(auto& kv : *m) {
            outs.push_back(&kv.second);
        }
    }
    return outs;
}

static inline std::vector<dai::Node::Input*> _dai_collect_inputs(dai::Node* node) {
    std::vector<dai::Node::Input*> ins;
    if(!node) return ins;
    auto refs = node->getInputRefs();
    ins.insert(ins.end(), refs.begin(), refs.end());
    auto maps = node->getInputMapRefs();
    for(auto* m : maps) {
        if(!m) continue;
        for(auto& kv : *m) {
            ins.push_back(&kv.second);
        }
    }
    return ins;
}

static inline bool _dai_group_matches(const std::string& portGroup, const char* filterGroup) {
    if(filterGroup == nullptr) return true;
    return portGroup == std::string(filterGroup);
}

static inline dai::Node::Output* _dai_pick_output_for_input(dai::Node* fromNode, dai::Node::Input* input, const char* out_group) {
    if(!fromNode || !input) return nullptr;
    dai::Node::Output* best = nullptr;
    int bestScore = std::numeric_limits<int>::min();
    for(auto* o : _dai_collect_outputs(fromNode)) {
        if(!o) continue;
        if(!_dai_group_matches(o->getGroup(), out_group)) continue;
        if(!o->canConnect(*input)) continue;
        int score = _dai_score_port_name(o->getName(), /*isOutput=*/true);
        if(o->getGroup().empty()) score += 2;
        if(score > bestScore) {
            bestScore = score;
            best = o;
        }
    }
    return best;
}

static inline dai::Node::Input* _dai_pick_input_for_output(dai::Node* toNode, dai::Node::Output* output, const char* in_group) {
    if(!toNode || !output) return nullptr;
    dai::Node::Input* best = nullptr;
    int bestScore = std::numeric_limits<int>::min();
    for(auto* i : _dai_collect_inputs(toNode)) {
        if(!i) continue;
        if(!_dai_group_matches(i->getGroup(), in_group)) continue;
        if(!output->canConnect(*i)) continue;
        int score = _dai_score_port_name(i->getName(), /*isOutput=*/false);
        if(i->getGroup().empty()) score += 2;
        if(score > bestScore) {
            bestScore = score;
            best = i;
        }
    }
    return best;
}

bool dai_node_link(DaiNode from, const char* out_group, const char* out_name, DaiNode to, const char* in_group, const char* in_name) {
    if (!from || !to) {
        last_error = "dai_node_link: null from/to";
        return false;
    }
    try {
        auto fromNode = static_cast<dai::Node*>(from);
        auto toNode = static_cast<dai::Node*>(to);

        dai::Node::Output* out = nullptr;
        dai::Node::Input* input = nullptr;

        const bool outSpecified = !_dai_cstr_empty(out_name);
        const bool inSpecified = !_dai_cstr_empty(in_name);

        if(outSpecified) {
            out = out_group ? fromNode->getOutputRef(std::string(out_group), std::string(out_name)) : fromNode->getOutputRef(std::string(out_name));
            if(!out) {
                last_error = "dai_node_link: output not found";
                return false;
            }
        }
        if(inSpecified) {
            input = in_group ? toNode->getInputRef(std::string(in_group), std::string(in_name)) : toNode->getInputRef(std::string(in_name));
            if(!input) {
                last_error = "dai_node_link: input not found";
                return false;
            }
        }

        if(!outSpecified && !inSpecified) {
            // Choose the best compatible pair.
            dai::Node::Output* bestOut = nullptr;
            dai::Node::Input* bestIn = nullptr;
            int bestScore = std::numeric_limits<int>::min();
            for(auto* o : _dai_collect_outputs(fromNode)) {
                if(!o) continue;
                if(!_dai_group_matches(o->getGroup(), out_group)) continue;
                for(auto* i : _dai_collect_inputs(toNode)) {
                    if(!i) continue;
                    if(!_dai_group_matches(i->getGroup(), in_group)) continue;
                    if(!o->canConnect(*i)) continue;
                    int score = _dai_score_port_name(o->getName(), /*isOutput=*/true) + _dai_score_port_name(i->getName(), /*isOutput=*/false);
                    if(o->getGroup().empty()) score += 2;
                    if(i->getGroup().empty()) score += 2;
                    if(score > bestScore) {
                        bestScore = score;
                        bestOut = o;
                        bestIn = i;
                    }
                }
            }
            out = bestOut;
            input = bestIn;
        } else if(!outSpecified && inSpecified) {
            out = _dai_pick_output_for_input(fromNode, input, out_group);
        } else if(outSpecified && !inSpecified) {
            input = _dai_pick_input_for_output(toNode, out, in_group);
        }

        if(!out || !input) {
            last_error = "dai_node_link: no compatible ports found";
            return false;
        }

        out->link(*input);
        return true;
    } catch (const std::exception& e) {
        last_error = std::string("dai_node_link failed: ") + e.what();
        return false;
    }
}

bool dai_node_unlink(DaiNode from, const char* out_group, const char* out_name, DaiNode to, const char* in_group, const char* in_name) {
    if (!from || !to) {
        last_error = "dai_node_unlink: null from/to";
        return false;
    }
    try {
        auto fromNode = static_cast<dai::Node*>(from);
        auto toNode = static_cast<dai::Node*>(to);

        dai::Node::Output* out = nullptr;
        dai::Node::Input* input = nullptr;

        const bool outSpecified = !_dai_cstr_empty(out_name);
        const bool inSpecified = !_dai_cstr_empty(in_name);

        if(outSpecified) {
            out = out_group ? fromNode->getOutputRef(std::string(out_group), std::string(out_name)) : fromNode->getOutputRef(std::string(out_name));
            if(!out) {
                last_error = "dai_node_unlink: output not found";
                return false;
            }
        }
        if(inSpecified) {
            input = in_group ? toNode->getInputRef(std::string(in_group), std::string(in_name)) : toNode->getInputRef(std::string(in_name));
            if(!input) {
                last_error = "dai_node_unlink: input not found";
                return false;
            }
        }

        if(!outSpecified || !inSpecified) {
            // Find an actual existing connection between `fromNode` and `toNode` that matches any provided filters.
            dai::Node::Output* bestOut = nullptr;
            dai::Node::Input* bestIn = nullptr;
            int bestScore = std::numeric_limits<int>::min();

            auto outputs = outSpecified ? std::vector<dai::Node::Output*>{out} : _dai_collect_outputs(fromNode);
            for(auto* o : outputs) {
                if(!o) continue;
                if(!_dai_group_matches(o->getGroup(), out_group)) continue;
                for(const auto& c : o->getConnections()) {
                    if(c.in == nullptr) continue;
                    auto inNode = c.inputNode.lock();
                    if(!inNode) continue;
                    if(inNode.get() != toNode) continue;
                    if(!_dai_group_matches(c.inputGroup, in_group)) continue;
                    if(inSpecified && c.inputName != std::string(in_name)) continue;

                    int score = _dai_score_port_name(o->getName(), /*isOutput=*/true) + _dai_score_port_name(c.inputName, /*isOutput=*/false);
                    if(score > bestScore) {
                        bestScore = score;
                        bestOut = o;
                        bestIn = c.in;
                    }
                }
            }
            out = bestOut;
            input = bestIn;
        }

        if(!out || !input) {
            last_error = "dai_node_unlink: no matching connection found";
            return false;
        }
        out->unlink(*input);
        return true;
    } catch (const std::exception& e) {
        last_error = std::string("dai_node_unlink failed: ") + e.what();
        return false;
    }
}

// Low-level camera operations
DaiOutput dai_camera_request_full_resolution_output(DaiCameraNode camera) {
    if (!camera) {
        last_error = "dai_camera_request_full_resolution_output: null camera";
        return nullptr;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        dai::Node::Output* output = cam->requestFullResolutionOutput();
        return static_cast<DaiOutput>(output);
    } catch (const std::exception& e) {
        last_error = std::string("dai_camera_request_full_resolution_output failed: ") + e.what();
        return nullptr;
    }
}

DaiOutput dai_camera_request_full_resolution_output_ex(DaiCameraNode camera, int type, float fps, bool use_highest_resolution) {
    if (!camera) {
        last_error = "dai_camera_request_full_resolution_output_ex: null camera";
        return nullptr;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        std::optional<dai::ImgFrame::Type> opt_type = (type >= 0) ? std::optional<dai::ImgFrame::Type>(static_cast<dai::ImgFrame::Type>(type))
                                                                  : std::nullopt;
        std::optional<float> opt_fps = (fps > 0.0f) ? std::optional<float>(fps) : std::nullopt;
        dai::Node::Output* output = cam->requestFullResolutionOutput(opt_type, opt_fps, use_highest_resolution);
        return static_cast<DaiOutput>(output);
    } catch (const std::exception& e) {
        last_error = std::string("dai_camera_request_full_resolution_output_ex failed: ") + e.what();
        return nullptr;
    }
}

bool dai_camera_build(DaiCameraNode camera, int board_socket, int sensor_width, int sensor_height, float sensor_fps) {
    if(!camera) {
        last_error = "dai_camera_build: null camera";
        return false;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        auto socket = static_cast<dai::CameraBoardSocket>(board_socket);

        std::optional<std::pair<uint32_t, uint32_t>> opt_res = std::nullopt;
        if(sensor_width > 0 && sensor_height > 0) {
            opt_res = std::make_pair(static_cast<uint32_t>(sensor_width), static_cast<uint32_t>(sensor_height));
        }
        std::optional<float> opt_fps = (sensor_fps > 0.0f) ? std::optional<float>(sensor_fps) : std::nullopt;

        cam->build(socket, opt_res, opt_fps);
        return true;
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_build failed: ") + e.what();
        return false;
    }
}

int dai_camera_get_board_socket(DaiCameraNode camera) {
    if(!camera) {
        last_error = "dai_camera_get_board_socket: null camera";
        return -1;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        return static_cast<int>(cam->getBoardSocket());
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_get_board_socket failed: ") + e.what();
        return -1;
    }
}

uint32_t dai_camera_get_max_width(DaiCameraNode camera) {
    if(!camera) {
        last_error = "dai_camera_get_max_width: null camera";
        return 0;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        return cam->getMaxWidth();
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_get_max_width failed: ") + e.what();
        return 0;
    }
}

uint32_t dai_camera_get_max_height(DaiCameraNode camera) {
    if(!camera) {
        last_error = "dai_camera_get_max_height: null camera";
        return 0;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        return cam->getMaxHeight();
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_get_max_height failed: ") + e.what();
        return 0;
    }
}

void dai_camera_set_sensor_type(DaiCameraNode camera, int sensor_type) {
    if(!camera) {
        last_error = "dai_camera_set_sensor_type: null camera";
        return;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        cam->setSensorType(static_cast<dai::CameraSensorType>(sensor_type));
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_set_sensor_type failed: ") + e.what();
    }
}

int dai_camera_get_sensor_type(DaiCameraNode camera) {
    if(!camera) {
        last_error = "dai_camera_get_sensor_type: null camera";
        return -1;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        return static_cast<int>(cam->getSensorType());
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_get_sensor_type failed: ") + e.what();
        return -1;
    }
}

void dai_camera_set_raw_num_frames_pool(DaiCameraNode camera, int num) {
    if(!camera) {
        last_error = "dai_camera_set_raw_num_frames_pool: null camera";
        return;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        cam->setRawNumFramesPool(num);
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_set_raw_num_frames_pool failed: ") + e.what();
    }
}

void dai_camera_set_max_size_pool_raw(DaiCameraNode camera, int size) {
    if(!camera) {
        last_error = "dai_camera_set_max_size_pool_raw: null camera";
        return;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        cam->setMaxSizePoolRaw(size);
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_set_max_size_pool_raw failed: ") + e.what();
    }
}

void dai_camera_set_isp_num_frames_pool(DaiCameraNode camera, int num) {
    if(!camera) {
        last_error = "dai_camera_set_isp_num_frames_pool: null camera";
        return;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        cam->setIspNumFramesPool(num);
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_set_isp_num_frames_pool failed: ") + e.what();
    }
}

void dai_camera_set_max_size_pool_isp(DaiCameraNode camera, int size) {
    if(!camera) {
        last_error = "dai_camera_set_max_size_pool_isp: null camera";
        return;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        cam->setMaxSizePoolIsp(size);
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_set_max_size_pool_isp failed: ") + e.what();
    }
}

void dai_camera_set_num_frames_pools(DaiCameraNode camera, int raw, int isp, int outputs) {
    if(!camera) {
        last_error = "dai_camera_set_num_frames_pools: null camera";
        return;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        cam->setNumFramesPools(raw, isp, outputs);
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_set_num_frames_pools failed: ") + e.what();
    }
}

void dai_camera_set_max_size_pools(DaiCameraNode camera, int raw, int isp, int outputs) {
    if(!camera) {
        last_error = "dai_camera_set_max_size_pools: null camera";
        return;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        cam->setMaxSizePools(raw, isp, outputs);
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_set_max_size_pools failed: ") + e.what();
    }
}

void dai_camera_set_outputs_num_frames_pool(DaiCameraNode camera, int num) {
    if(!camera) {
        last_error = "dai_camera_set_outputs_num_frames_pool: null camera";
        return;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        cam->setOutputsNumFramesPool(num);
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_set_outputs_num_frames_pool failed: ") + e.what();
    }
}

void dai_camera_set_outputs_max_size_pool(DaiCameraNode camera, int size) {
    if(!camera) {
        last_error = "dai_camera_set_outputs_max_size_pool: null camera";
        return;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        cam->setOutputsMaxSizePool(size);
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_set_outputs_max_size_pool failed: ") + e.what();
    }
}

int dai_camera_get_raw_num_frames_pool(DaiCameraNode camera) {
    if(!camera) {
        last_error = "dai_camera_get_raw_num_frames_pool: null camera";
        return 0;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        return cam->getRawNumFramesPool();
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_get_raw_num_frames_pool failed: ") + e.what();
        return 0;
    }
}

int dai_camera_get_max_size_pool_raw(DaiCameraNode camera) {
    if(!camera) {
        last_error = "dai_camera_get_max_size_pool_raw: null camera";
        return 0;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        return cam->getMaxSizePoolRaw();
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_get_max_size_pool_raw failed: ") + e.what();
        return 0;
    }
}

int dai_camera_get_isp_num_frames_pool(DaiCameraNode camera) {
    if(!camera) {
        last_error = "dai_camera_get_isp_num_frames_pool: null camera";
        return 0;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        return cam->getIspNumFramesPool();
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_get_isp_num_frames_pool failed: ") + e.what();
        return 0;
    }
}

int dai_camera_get_max_size_pool_isp(DaiCameraNode camera) {
    if(!camera) {
        last_error = "dai_camera_get_max_size_pool_isp: null camera";
        return 0;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        return cam->getMaxSizePoolIsp();
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_get_max_size_pool_isp failed: ") + e.what();
        return 0;
    }
}

bool dai_camera_get_outputs_num_frames_pool(DaiCameraNode camera, int* out_num) {
    if(!camera || !out_num) {
        last_error = "dai_camera_get_outputs_num_frames_pool: null camera or out_num";
        return false;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        auto opt = cam->getOutputsNumFramesPool();
        if(opt.has_value()) {
            *out_num = opt.value();
            return true;
        }
        return false;
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_get_outputs_num_frames_pool failed: ") + e.what();
        return false;
    }
}

bool dai_camera_get_outputs_max_size_pool(DaiCameraNode camera, size_t* out_size) {
    if(!camera || !out_size) {
        last_error = "dai_camera_get_outputs_max_size_pool: null camera or out_size";
        return false;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        auto opt = cam->getOutputsMaxSizePool();
        if(opt.has_value()) {
            *out_size = opt.value();
            return true;
        }
        return false;
    } catch(const std::exception& e) {
        last_error = std::string("dai_camera_get_outputs_max_size_pool failed: ") + e.what();
        return false;
    }
}
DaiCameraNode dai_pipeline_create_camera(DaiPipeline pipeline, int board_socket) {
    if (!pipeline) {
        last_error = "dai_pipeline_create_camera: null pipeline";
        return nullptr;
    }
    try {
        auto pipe = static_cast<dai::Pipeline*>(pipeline);
        auto cameraBuilder = pipe->create<dai::node::Camera>();
        auto socket = static_cast<dai::CameraBoardSocket>(board_socket);
        auto camera = cameraBuilder->build(socket);
        return static_cast<DaiCameraNode>(camera.get());
    } catch (const std::exception& e) {
        last_error = std::string("dai_pipeline_create_camera failed: ") + e.what();
        return nullptr;
    }
}

DaiOutput dai_camera_request_output(DaiCameraNode camera, int width, int height, int type, int resize_mode, float fps, int enable_undistortion) {
    if (!camera) {
        last_error = "dai_camera_request_output: null camera";
        return nullptr;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        std::pair<uint32_t, uint32_t> size(static_cast<uint32_t>(width), static_cast<uint32_t>(height));
        std::optional<dai::ImgFrame::Type> opt_type = (type >= 0) ? std::optional<dai::ImgFrame::Type>(static_cast<dai::ImgFrame::Type>(type)) : std::nullopt;
        dai::ImgResizeMode resize = static_cast<dai::ImgResizeMode>(resize_mode);
        std::optional<float> opt_fps = (fps > 0.0f) ? std::optional<float>(fps) : std::nullopt;
        std::optional<bool> opt_undist = (enable_undistortion >= 0) ? std::optional<bool>(enable_undistortion != 0) : std::nullopt;
        dai::Node::Output* output = cam->requestOutput(size, opt_type, resize, opt_fps, opt_undist);
        return static_cast<DaiOutput>(output);
    } catch (const std::exception& e) {
        last_error = std::string("dai_camera_request_output failed: ") + e.what();
        return nullptr;
    }
}

DaiDataQueue dai_output_create_queue(DaiOutput output, unsigned int max_size, bool blocking) {
    if (!output) {
        last_error = "dai_output_create_queue: null output";
        return nullptr;
    }
    try {
        auto out = static_cast<dai::Node::Output*>(output);
        auto queue = out->createOutputQueue(max_size, blocking);
        return new std::shared_ptr<dai::MessageQueue>(queue);
    } catch (const std::exception& e) {
        last_error = std::string("dai_output_create_queue failed: ") + e.what();
        return nullptr;
    }
}

void dai_queue_delete(DaiDataQueue queue) {
    if(queue) {
        auto ptr = static_cast<std::shared_ptr<dai::MessageQueue>*>(queue);
        delete ptr;
    }
}

DaiImgFrame dai_queue_get_frame(DaiDataQueue queue, int timeout_ms) {
    if(!queue) {
        last_error = "dai_queue_get_frame: null queue";
        return nullptr;
    }
    try {
        auto ptr = static_cast<std::shared_ptr<dai::MessageQueue>*>(queue);
        std::shared_ptr<dai::ImgFrame> frame;
        if(timeout_ms < 0) {
            frame = (*ptr)->get<dai::ImgFrame>();
        } else {
            bool timedOut = false;
            frame = (*ptr)->get<dai::ImgFrame>(std::chrono::milliseconds(timeout_ms), timedOut);
            if(timedOut) {
                return nullptr;
            }
        }
        if(!frame) {
            return nullptr;
        }
        return new std::shared_ptr<dai::ImgFrame>(frame);
    } catch(const std::exception& e) {
        last_error = std::string("dai_queue_get_frame failed: ") + e.what();
        return nullptr;
    }
}

DaiImgFrame dai_queue_try_get_frame(DaiDataQueue queue) {
    if(!queue) {
        last_error = "dai_queue_try_get_frame: null queue";
        return nullptr;
    }
    try {
        auto ptr = static_cast<std::shared_ptr<dai::MessageQueue>*>(queue);
        auto frame = (*ptr)->tryGet<dai::ImgFrame>();
        if(!frame) {
            return nullptr;
        }
        return new std::shared_ptr<dai::ImgFrame>(frame);
    } catch(const std::exception& e) {
        last_error = std::string("dai_queue_try_get_frame failed: ") + e.what();
        return nullptr;
    }
}

// Low-level frame operations
void* dai_frame_get_data(DaiImgFrame frame) {
    if (!frame) {
        last_error = "dai_frame_get_data: null frame";
        return nullptr;
    }
    try {
        auto sharedFrame = static_cast<std::shared_ptr<dai::ImgFrame>*>(frame);
        if(!sharedFrame->get()) {
            return nullptr;
        }
        return (*sharedFrame)->getData().data();
    } catch (const std::exception& e) {
        last_error = std::string("dai_frame_get_data failed: ") + e.what();
        return nullptr;
    }
}

int dai_frame_get_width(DaiImgFrame frame) {
    if (!frame) {
        last_error = "dai_frame_get_width: null frame";
        return 0;
    }
    try {
        auto sharedFrame = static_cast<std::shared_ptr<dai::ImgFrame>*>(frame);
        if(!sharedFrame->get()) {
            return 0;
        }
        return (*sharedFrame)->getWidth();
    } catch (const std::exception& e) {
        last_error = std::string("dai_frame_get_width failed: ") + e.what();
        return 0;
    }
}

int dai_frame_get_height(DaiImgFrame frame) {
    if (!frame) {
        last_error = "dai_frame_get_height: null frame";
        return 0;
    }
    try {
        auto sharedFrame = static_cast<std::shared_ptr<dai::ImgFrame>*>(frame);
        if(!sharedFrame->get()) {
            return 0;
        }
        return (*sharedFrame)->getHeight();
    } catch (const std::exception& e) {
        last_error = std::string("dai_frame_get_height failed: ") + e.what();
        return 0;
    }
}

int dai_frame_get_type(DaiImgFrame frame) {
    if (!frame) {
        last_error = "dai_frame_get_type: null frame";
        return 0;
    }
    try {
        auto sharedFrame = static_cast<std::shared_ptr<dai::ImgFrame>*>(frame);
        if(!sharedFrame->get()) {
            return 0;
        }
        return static_cast<int>((*sharedFrame)->getType());
    } catch (const std::exception& e) {
        last_error = std::string("dai_frame_get_type failed: ") + e.what();
        return 0;
    }
}

size_t dai_frame_get_size(DaiImgFrame frame) {
    if (!frame) {
        last_error = "dai_frame_get_size: null frame";
        return 0;
    }
    try {
        auto sharedFrame = static_cast<std::shared_ptr<dai::ImgFrame>*>(frame);
        if(!sharedFrame->get()) {
            return 0;
        }
        return (*sharedFrame)->getData().size();
    } catch (const std::exception& e) {
        last_error = std::string("dai_frame_get_size failed: ") + e.what();
        return 0;
    }
}

void dai_frame_release(DaiImgFrame frame) {
    if(frame) {
        auto ptr = static_cast<std::shared_ptr<dai::ImgFrame>*>(frame);
        delete ptr;
    }
}

// Low-level utility functions  
int dai_device_get_connected_camera_sockets(DaiDevice device, int* sockets, int max_count) {
    if (!device || !sockets) {
        last_error = "dai_device_get_connected_camera_sockets: null device or sockets";
        return 0;
    }
    try {
        auto dev = static_cast<std::shared_ptr<dai::Device>*>(device);
        if(!dev->get() || !(*dev)) {
            last_error = "dai_device_get_connected_camera_sockets: invalid device";
            return 0;
        }
        auto connected = (*dev)->getConnectedCameras();
        int count = 0;
        for (const auto& socket : connected) {
            if (count >= max_count) break;
            sockets[count] = static_cast<int>(socket);
            count++;
        }
        return count;
    } catch (const std::exception& e) {
        last_error = std::string("dai_device_get_connected_camera_sockets failed: ") + e.what();
        return 0;
    }
}

const char* dai_camera_socket_name(int socket) {
    try {
        auto board_socket = static_cast<dai::CameraBoardSocket>(socket);
        static std::string name = toString(board_socket);
        return name.c_str();
    } catch (const std::exception& e) {
        last_error = std::string("dai_camera_socket_name failed: ") + e.what();
        return "UNKNOWN";
    }
}

// Error handling
const char* dai_get_last_error() {
    if(last_error.empty()) {
        return nullptr;
    }
    return last_error.c_str();
}

void dai_clear_last_error() {
    last_error.clear();
}

} // namespace dai
