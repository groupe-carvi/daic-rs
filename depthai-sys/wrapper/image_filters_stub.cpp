#include "depthai/pipeline/node/ImageFilters.hpp"

#include <stdexcept>

// These are fallback stubs for builds where DepthAI is built without OpenCV support.
// When linking against a depthai-core that *does* provide these symbols (e.g. static
// libdepthai-core.a), we must not create strong duplicate definitions.
//
// On ELF platforms (Linux), marking them weak allows the real definitions to win.
#if defined(__ELF__) && (defined(__GNUC__) || defined(__clang__))
#    define DEPTHAI_RS_WEAK __attribute__((weak))
#else
#    define DEPTHAI_RS_WEAK
#endif

namespace dai::node {

namespace {
[[noreturn]] void throw_not_available(const char* name) {
    throw std::runtime_error(std::string(name) + " is unavailable because DepthAI was built without OpenCV support.");
}
}  // namespace

DEPTHAI_RS_WEAK std::shared_ptr<ImageFilters> ImageFilters::build(Node::Output&, ImageFiltersPresetMode) {
    throw_not_available("ImageFilters");
}

DEPTHAI_RS_WEAK std::shared_ptr<ImageFilters> ImageFilters::build(ImageFiltersPresetMode) {
    throw_not_available("ImageFilters");
}

DEPTHAI_RS_WEAK void ImageFilters::run() {
    throw_not_available("ImageFilters");
}

DEPTHAI_RS_WEAK void ImageFilters::setRunOnHost(bool runOnHost) {
    runOnHostVar = runOnHost;
}

DEPTHAI_RS_WEAK bool ImageFilters::runOnHost() const {
    return runOnHostVar;
}

DEPTHAI_RS_WEAK void ImageFilters::setDefaultProfilePreset(ImageFiltersPresetMode) {}

DEPTHAI_RS_WEAK std::shared_ptr<ToFDepthConfidenceFilter> ToFDepthConfidenceFilter::build(Node::Output&, Node::Output&, ImageFiltersPresetMode) {
    throw_not_available("ToFDepthConfidenceFilter");
}

DEPTHAI_RS_WEAK std::shared_ptr<ToFDepthConfidenceFilter> ToFDepthConfidenceFilter::build(ImageFiltersPresetMode) {
    throw_not_available("ToFDepthConfidenceFilter");
}

DEPTHAI_RS_WEAK void ToFDepthConfidenceFilter::run() {
    throw_not_available("ToFDepthConfidenceFilter");
}

DEPTHAI_RS_WEAK void ToFDepthConfidenceFilter::setRunOnHost(bool runOnHost) {
    runOnHostVar = runOnHost;
}

DEPTHAI_RS_WEAK bool ToFDepthConfidenceFilter::runOnHost() const {
    return runOnHostVar;
}

}  // namespace dai::node
