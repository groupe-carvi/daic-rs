#include "wrapper.h"
#include "depthai/build/version.hpp"
#include <chrono>
#include <cstring>
#include <cstdlib>
#include <memory>
#include <string>

// Global error storage
static std::string last_error = "";

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

char* dai_std_string_to_cstring(const std::string& str) {
    size_t len = str.length();
    char* result = static_cast<char*>(malloc(len + 1));
    if (result) {
        strcpy(result, str.c_str());
    }
    return result;
}

void dai_std_string_destroy(const std::string* str) {
    if (str) {
        delete str;
    }
}

std::string* dai_create_std_string(const char* cstr) {
    if (!cstr) return new std::string();
    return new std::string(cstr);
}

const char* dai_std_string_c_str(const std::string* str) {
    if (!str) return nullptr;
    return str->c_str();
}

size_t dai_std_string_length(const std::string* str) {
    if (!str) return 0;
    return str->length();
}

// Low-level device operations - direct pointer manipulation
DaiDevice dai_device_new() {
    try {
        dai_clear_last_error();
        auto device = new dai::Device();
        return static_cast<DaiDevice>(device);
    } catch (const std::exception& e) {
        last_error = std::string("dai_device_new failed: ") + e.what();
        return nullptr;
    }
}

void dai_device_delete(DaiDevice device) {
    if (device) {
        auto dev = static_cast<dai::Device*>(device);
        delete dev;
    }
}

bool dai_device_is_closed(DaiDevice device) {
    if (!device) {
        last_error = "dai_device_is_closed: null device";
        return true;
    }
    try {
        auto dev = static_cast<dai::Device*>(device);
        return dev->isClosed();
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
        auto dev = static_cast<dai::Device*>(device);
        dev->close();
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

void dai_pipeline_delete(DaiPipeline pipeline) {
    if (pipeline) {
        auto pipe = static_cast<dai::Pipeline*>(pipeline);
        delete pipe;
    }
}

bool dai_pipeline_start(DaiPipeline pipeline, DaiDevice device) {
    if (!pipeline || !device) {
        last_error = "dai_pipeline_start: null pipeline or device";
        return false;
    }
    try {
        auto pipe = static_cast<dai::Pipeline*>(pipeline);
        auto dev = static_cast<dai::Device*>(device);
        dev->startPipeline(*pipe);
        return true;
    } catch (const std::exception& e) {
        last_error = std::string("dai_pipeline_start failed: ") + e.what();
        return false;
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

bool dai_node_link(DaiNode from, const char* out_group, const char* out_name, DaiNode to, const char* in_group, const char* in_name) {
    if (!from || !to) {
        last_error = "dai_node_link: null from/to";
        return false;
    }
    if (!out_name || !in_name) {
        last_error = "dai_node_link: null out_name/in_name";
        return false;
    }
    try {
        auto fromNode = static_cast<dai::Node*>(from);
        auto toNode = static_cast<dai::Node*>(to);
        const auto on = std::string(out_name);
        const auto in = std::string(in_name);

        dai::Node::Output* out = nullptr;
        if(out_group) {
            out = fromNode->getOutputRef(std::string(out_group), on);
        } else {
            out = fromNode->getOutputRef(on);
        }

        dai::Node::Input* input = nullptr;
        if(in_group) {
            input = toNode->getInputRef(std::string(in_group), in);
        } else {
            input = toNode->getInputRef(in);
        }
        if(!out || !input) {
            last_error = "dai_node_link: output or input not found";
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
    if (!out_name || !in_name) {
        last_error = "dai_node_unlink: null out_name/in_name";
        return false;
    }
    try {
        auto fromNode = static_cast<dai::Node*>(from);
        auto toNode = static_cast<dai::Node*>(to);
        const auto on = std::string(out_name);
        const auto in = std::string(in_name);

        dai::Node::Output* out = nullptr;
        if(out_group) {
            out = fromNode->getOutputRef(std::string(out_group), on);
        } else {
            out = fromNode->getOutputRef(on);
        }

        dai::Node::Input* input = nullptr;
        if(in_group) {
            input = toNode->getInputRef(std::string(in_group), in);
        } else {
            input = toNode->getInputRef(in);
        }
        if(!out || !input) {
            last_error = "dai_node_unlink: output or input not found";
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
dai::Node::Output* dai_camera_request_full_resolution_output(DaiCameraNode camera) {
    if (!camera) {
        last_error = "dai_camera_request_full_resolution_output: null camera";
        return nullptr;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        dai::Node::Output* output = cam->requestFullResolutionOutput();
        return output;
    } catch (const std::exception& e) {
        last_error = std::string("dai_camera_request_full_resolution_output failed: ") + e.what();
        return nullptr;
    }
}
dai::Node::Output* dai_camera_request_output_capability(DaiCameraNode camera, const dai::Capability* capability, int on_host) {
    if (!camera || !capability) {
        last_error = "dai_camera_request_output_capability: null camera or capability";
        return nullptr;
    }
    try {
        auto cam = static_cast<dai::node::Camera*>(camera);
        bool host = (on_host != 0);
        dai::Node::Output* output = cam->requestOutput(*capability, host);
        return output;
    } catch (const std::exception& e) {
        last_error = std::string("dai_camera_request_output_capability failed: ") + e.what();
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

dai::Node::Output* dai_camera_request_output(DaiCameraNode camera, int width, int height, int type, int resize_mode, float fps, int enable_undistortion) {
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
        return output;
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
        auto dev = static_cast<dai::Device*>(device);
        auto connected = dev->getConnectedCameras();
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
