#include "depthai/pipeline/node/ImageFilters.hpp"

#include <stdexcept>

namespace dai::node {

namespace {
[[noreturn]] void throw_not_available(const char* name) {
    throw std::runtime_error(std::string(name) + " is unavailable because DepthAI was built without OpenCV support.");
}
}  // namespace

std::shared_ptr<ImageFilters> ImageFilters::build(Node::Output&, ImageFiltersPresetMode) {
    throw_not_available("ImageFilters");
}

std::shared_ptr<ImageFilters> ImageFilters::build(ImageFiltersPresetMode) {
    throw_not_available("ImageFilters");
}

void ImageFilters::run() {
    throw_not_available("ImageFilters");
}

void ImageFilters::setRunOnHost(bool runOnHost) {
    runOnHostVar = runOnHost;
}

bool ImageFilters::runOnHost() const {
    return runOnHostVar;
}

void ImageFilters::setDefaultProfilePreset(ImageFiltersPresetMode) {}

std::shared_ptr<ToFDepthConfidenceFilter> ToFDepthConfidenceFilter::build(Node::Output&, Node::Output&, ImageFiltersPresetMode) {
    throw_not_available("ToFDepthConfidenceFilter");
}

std::shared_ptr<ToFDepthConfidenceFilter> ToFDepthConfidenceFilter::build(ImageFiltersPresetMode) {
    throw_not_available("ToFDepthConfidenceFilter");
}

void ToFDepthConfidenceFilter::run() {
    throw_not_available("ToFDepthConfidenceFilter");
}

void ToFDepthConfidenceFilter::setRunOnHost(bool runOnHost) {
    runOnHostVar = runOnHost;
}

bool ToFDepthConfidenceFilter::runOnHost() const {
    return runOnHostVar;
}

}  // namespace dai::node
