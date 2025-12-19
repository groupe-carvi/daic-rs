#pragma once

// autocxx entrypoint header.
//
// Historically this file duplicated the C ABI declarations because including DepthAI headers
// directly in an autocxx-parsed translation unit can be fragile (heavy templates, transitive
// dependencies, etc.).
//
// The C ABI surface is now fully defined in `wrapper.h` using only opaque handles and POD types,
// so we can include it directly and avoid duplication.

#include "wrapper.h"

#ifdef __cplusplus
#include "depthai/depthai.hpp"
#endif
