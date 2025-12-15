#include "wrapper.h"
#include "depthai/depthai.hpp"
#include "depthai/build/version.hpp"
#include "XLink/XLink.h"
#include "XLink/XLinkPublicDefines.h"
#include <chrono>
#include <cstring>
#include <cstdlib>
#include <limits>
#include <memory>
#include <mutex>
#include <string>

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

namespace daic {

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
        auto pipeline = new dai::Pipeline();
        return static_cast<DaiPipeline>(pipeline);
    } catch (const std::exception& e) {
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
// Keep in sync with Rust-side NodeKind discriminants.
enum class DaiNodeKind : int {
    Camera = 1,
    StereoDepth = 2,
    ImageAlign = 3,
    RGBD = 4,
};

DaiNode dai_pipeline_create_node(DaiPipeline pipeline, int kind) {
    if (!pipeline) {
        last_error = "dai_pipeline_create_node: null pipeline";
        return nullptr;
    }
    try {
        auto pipe = static_cast<dai::Pipeline*>(pipeline);
        const auto k = static_cast<DaiNodeKind>(kind);
        switch (k) {
            case DaiNodeKind::Camera: {
                auto node = pipe->create<dai::node::Camera>();
                return static_cast<DaiNode>(node.get());
            }
            case DaiNodeKind::StereoDepth: {
                auto node = pipe->create<dai::node::StereoDepth>();
                return static_cast<DaiNode>(node.get());
            }
            case DaiNodeKind::ImageAlign: {
                auto node = pipe->create<dai::node::ImageAlign>();
                return static_cast<DaiNode>(node.get());
            }
            case DaiNodeKind::RGBD: {
                auto node = pipe->create<dai::node::RGBD>();
                return static_cast<DaiNode>(node.get());
            }
            default:
                last_error = "dai_pipeline_create_node: unknown kind";
                return nullptr;
        }
    } catch (const std::exception& e) {
        last_error = std::string("dai_pipeline_create_node failed: ") + e.what();
        return nullptr;
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
    return last_error.c_str();
}

void dai_clear_last_error() {
    last_error.clear();
}

} // namespace daic
