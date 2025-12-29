# depthai-rs

Experimental Rust bindings + safe-ish wrapper for Luxonis **DepthAI-Core v3**.

- High-level crate: `depthai-rs` (Rust API)
- Low-level crate: `depthai-sys` (builds DepthAI-Core and exposes an FFI surface via `autocxx`)

> [!CAUTION]
> This project is experimental and in active development. APIs and behavior can change.

> [!WARNING]
> DepthAI-Core itself does not provide strong API stability guarantees yet. This repo currently targets **DepthAI-Core `v3.2.1`**.

## What’s in this repo

### Crates

- `depthai-sys`
	- Builds DepthAI-Core and its dependencies (in `target/dai-build/`).
	- Compiles a small C++ wrapper (`depthai-sys/wrapper/wrapper.cpp`) and generates Rust bindings using `autocxx`.
- `depthai-rs`
	- Safe(-er) Rust wrapper types like `Device`, `Pipeline`, typed camera helpers, and a generic node API.

### Repository layout (high-level)

```
depthai-sys/            # FFI crate (build script + wrapper)
	build.rs           # clones/builds DepthAI-Core (Linux) or downloads prebuilt (Windows)
	wrapper/           # C ABI functions used by Rust
src/                 # Rust API (`Device`, `Pipeline`, nodes, camera helpers)
examples/            # runnable examples
tests/               # tests (some are ignored unless you enable hardware testing)
```

## Supported platforms

- Linux: primarily **Debian/Ubuntu**-like systems (today).
- Windows: intended to use prebuilt DepthAI-Core artifacts.

If you’re on another distro/OS, it may still work, but you may need to adjust packages and toolchain paths.

## Prerequisites

### Linux (Ubuntu/Debian)

Install build tooling used by `autocxx` + CMake builds:

```bash
sudo apt -y install \
	clang libclang-dev \
	cmake ninja-build pkg-config \
	python3 \
	autoconf automake autoconf-archive libtool \
	libudev-dev libssl-dev \
	nasm \
	libdw-dev libelf-dev
```

Optional (not required for core builds, but useful for local OpenCV tooling):

```bash
sudo apt -y install libopencv-dev
```

#### USB permissions (recommended)

Some DepthAI devices require udev rules so you can access them without running as root.
If you hit permission errors (or see the device only under `sudo`), consult the official DepthAI/DepthAI-Core docs for the recommended udev rules for your device.

### Windows

Install:

- LLVM/Clang (for `autocxx`/libclang)
- Visual Studio Build Tools (C++ workload)
- CMake (if not already installed)

Example (PowerShell):

```powershell
winget install -e --id LLVM.LLVM
```

## Features

### Default features

- `rerun`: Enables Rerun visualization support with `RerunHostNode`. Adds Tokio runtime and web viewer dependencies.

### Optional features

- `hit`: Hardware Integration Tests - enable with `cargo test --features hit` when you have a physical device connected.

### DepthAI-Core version selection (advanced)

`depthai-sys` can download/build different **DepthAI-Core** versions. Because this crate links native code (FFI), only a small set of versions is supported at any given time.

Select **exactly one** of the following Cargo features:

- `latest` (default; maps to the latest supported tag)
- `v3-2-1` (maps to tag `v3.2.1`)
- `v3-2-0` (maps to tag `v3.2.0`)
- `v3-1-0` (maps to tag `v3.1.0`)

Note: Cargo feature names can’t contain dots (`.`), so version selectors use hyphens.

Examples:

```bash
# Build against DepthAI-Core v3.2.0
cargo build --features v3-2-0

# Run an example against DepthAI-Core v3.1.0
cargo run --example pipeline_creation --features v3-1-0

# Explicitly select latest (usually unnecessary; it's the default)
cargo build --features latest
```

If you enable more than one of these, the build will fail early with an explicit error.

To build without the rerun feature:

```bash
cargo build --no-default-features
```

## Build

From the repo root:

```bash
cargo build
```

Notes:

- The first build can take a while because DepthAI-Core is fetched/built and dependencies are prepared.
- Build artifacts for native code are cached under `target/dai-build/<depthai-core-tag>/`.
	(So if you switch between `--features v3-2-1` and `--features v3-2-0`, each version keeps its own build cache.)

## Run examples

```bash
# Basic examples
cargo run --example pipeline_creation
cargo run --example camera
cargo run --example composite_node

# Host node examples
cargo run --example host_node
cargo run --example threaded_host_node

# Rerun visualization examples (requires rerun feature)
cargo run --example rerun_host_node --features rerun
cargo run --example rgbd_rerun --features rerun
```

## DepthAI feature support

This section is generated from the native DepthAI-Core C++ examples vendored in this repo under `target/dai-build/depthai-core/examples/cpp`.

- ✅ in the **Supported** column means `depthai-rs` currently wraps enough of that feature/node to build and run *at least one* equivalent pipeline.
- A blank cell means it’s not yet wrapped/exposed in the Rust API (even if DepthAI-Core supports it).

### Feature support matrix
<!-- BEGIN depthai-feature-matrix -->
| DepthAI Feature | Nodes referenced (approx) | Supported | Rust reference |
|---|---|:---:|---|
| `AprilTags` | `AprilTag`, `Camera`, `ImageManip`, `ThreadedHostNode` |  |  |
| `Benchmark` | `BenchmarkIn`, `BenchmarkOut`, `Camera`, `NeuralNetwork` |  |  |
| `Camera` |  `Camera`, `ImageManip`, `Script` | ✅ | `examples/camera.rs`, `examples/host_node.rs`, `examples/threaded_host_node.rs`, `examples/rerun_host_node.rs`, `examples/rgbd_rerun.rs`, `examples/composite_node.rs` |
| `DetectionNetwork` | `Camera`, `DetectionNetwork`, `ReplayVideo`, `StereoDepth` |  |  |
| `DynamicCalibration` | `Camera`, `DynamicCalibration`, `StereoDepth` |  |  |
| `Events` | `Camera`, `DetectionNetwork` |  |  |
| `FeatureTracker` | `Camera`, `FeatureTracker`, `ImageManip` |  |  |
| `HostNodes` | `Camera`, `CustomNode`, `CustomThreadedNode`, `Display`, `HostCamera`, `HostNode`, `ImageManip`, `ReplayVideo` | ✅ | `examples/host_node.rs`, `examples/threaded_host_node.rs`, `examples/rerun_host_node.rs` |
| `IMU` | `IMU` |  |  |
| `ImageAlign` | `Camera`, `ImageAlign`, `StereoDepth`, `Sync` | ✅ | `examples/rgbd_rerun.rs` |
| `ImageManip` | `Camera`, `Display`, `ImageManip` |  |  |
| `Misc/AutoReconnect` | `Camera` |  |  |
| `Misc/Projectors` | `Camera` | ✅ | `Device::set_ir_dot_projector_intensity` |
| `ModelZoo` | — |  |  |
| `NeuralDepth` | `Camera`, `ImageAlign`, `NeuralDepth`, `RGBD`, `Sync` |  |  |
| `NeuralNetwork` | `Camera`, `NeuralNetwork` |  |  |
| `ObjectTracker` | `Camera`, `DetectionNetwork`, `ObjectTracker`, `ReplayVideo`, `SpatialDetectionNetwork`, `StereoDepth` |  |  |
| `RGBD` | `Camera`, `ImageAlign`, `RGBD`, `StereoDepth`, `ThreadedHostNode` | ✅ | `examples/rgbd_rerun.rs` |
| `RVC2/EdgeDetector` | `Camera`, `EdgeDetector` |  |  |
| `RVC2/ImageAlign` | `Camera`, `ImageAlign`, `Sync` |  |  |
| `RVC2/NNArchive` | `Camera`, `DetectionNetwork`, `NeuralNetwork` |  |  |
| `RVC2/SystemLogger` | `SystemLogger` |  |  |
| `RVC2/Thermal` | `Camera`, `DetectionNetwork`, `ImageAlign`, `Sync`, `Thermal` |  |  |
| `RVC2/ToF` | `Camera`, `ImageAlign`, `Sync`, `ToF` |  |  |
| `RVC2/VSLAM` | `BasaltVIO`, `Camera`, `FeatureTracker`, `IMU`, `RTABMapSLAM`, `RTABMapVIO`, `StereoDepth` |  |  |
| `RecordReplay` | `Camera`, `Display`, `IMU`, `RecordMetadataOnly`, `RecordVideo`, `ReplayMetadataOnly`, `ReplayVideo`, `VideoEncoder` |  |  |
| `Script` | `Camera`, `Script` |  |  |
| `SpatialDetectionNetwork` | `Camera`, `HostNode`, `SpatialDetectionNetwork`, `StereoDepth` |  |  |
| `SpatialLocationCalculator` | `Camera`, `SpatialLocationCalculator`, `StereoDepth` |  |  |
| `StereoDepth` | `Camera`, `StereoDepth` | ✅ | `examples/rgbd_rerun.rs` |
| `Sync` | `Camera`, `Sync` |  |  |
| `VideoEncoder` | `Camera`, `CustomNode`, `VideoEncoder` |  |  |
| `Visualizer` | `Camera`, `DetectionNetwork`, `HostNode`, `VideoEncoder` |  |  |
| `Warp` | `Camera`, `Warp` |  |  |
| `utility` | — |  |  |
<!-- END depthai-feature-matrix -->

### Node support matrix
<!-- BEGIN depthai-node-matrix -->
| Native Node | Example area(s) | Supported |
|---|---|:---:|
| `AprilTag` | `AprilTags` |  |
| `BasaltVIO` | `RVC2/VSLAM` |  |
| `BenchmarkIn` | `Benchmark` |  |
| `BenchmarkOut` | `Benchmark` |  |
| `Camera` | `AprilTags`, `Benchmark`, `Camera`, `DetectionNetwork`, `DynamicCalibration`, `Events`, … (+25) | ✅ |
| `CustomNode` | `HostNodes`, `VideoEncoder` | ✅ |
| `CustomThreadedNode` | `HostNodes` | ✅ |
| `DetectionNetwork` | `DetectionNetwork`, `Events`, `ObjectTracker`, `RVC2/NNArchive`, `RVC2/Thermal`, `Visualizer` |  |
| `Display` | `HostNodes`, `ImageManip`, `RecordReplay` |  |
| `DynamicCalibration` | `DynamicCalibration` |  |
| `EdgeDetector` | `RVC2/EdgeDetector` |  |
| `FeatureTracker` | `FeatureTracker`, `RVC2/VSLAM` |  |
| `HostCamera` | `HostNodes` |  |
| `HostNode` | `HostNodes`, `SpatialDetectionNetwork`, `Visualizer` | ✅ |
| `IMU` | `IMU`, `RVC2/VSLAM`, `RecordReplay` |  |
| `ImageAlign` | `ImageAlign`, `NeuralDepth`, `RGBD`, `RVC2/ImageAlign`, `RVC2/Thermal`, `RVC2/ToF` | ✅ |
| `ImageManip` | `AprilTags`, `Camera`, `FeatureTracker`, `HostNodes`, `ImageManip` |  |
| `NeuralDepth` | `NeuralDepth` |  |
| `NeuralNetwork` | `Benchmark`, `NeuralNetwork`, `RVC2/NNArchive` |  |
| `ObjectTracker` | `ObjectTracker` |  |
| `RGBD` | `NeuralDepth`, `RGBD` | ✅ |
| `RTABMapSLAM` | `RVC2/VSLAM` |  |
| `RTABMapVIO` | `RVC2/VSLAM` |  |
| `RecordMetadataOnly` | `RecordReplay` |  |
| `RecordVideo` | `RecordReplay` |  |
| `ReplayMetadataOnly` | `RecordReplay` |  |
| `ReplayVideo` | `DetectionNetwork`, `HostNodes`, `ObjectTracker`, `RecordReplay` |  |
| `Script` | `Camera`, `Script` |  |
| `SpatialDetectionNetwork` | `ObjectTracker`, `SpatialDetectionNetwork` |  |
| `SpatialLocationCalculator` | `SpatialLocationCalculator` |  |
| `StereoDepth` | `DetectionNetwork`, `DynamicCalibration`, `ImageAlign`, `ObjectTracker`, `RGBD`, `RVC2/VSLAM`, … (+3) | ✅ |
| `Sync` | `ImageAlign`, `NeuralDepth`, `RVC2/ImageAlign`, `RVC2/Thermal`, `RVC2/ToF`, `Sync` |  |
| `SystemLogger` | `RVC2/SystemLogger` |  |
| `Thermal` | `RVC2/Thermal` |  |
| `ThreadedHostNode` | `AprilTags`, `RGBD` | ✅ |
| `ToF` | `RVC2/ToF` |  |
| `VideoEncoder` | `RecordReplay`, `VideoEncoder`, `Visualizer` |  |
| `Warp` | `Warp` |  |
<!-- END depthai-node-matrix -->

## Environment variables (advanced)

`depthai-sys` exposes a few environment variables that affect native builds:

- `DEPTHAI_CORE_ROOT`: override the DepthAI-Core checkout directory.
- `DEPTHAI_SYS_LINK_SHARED=1`: prefer linking against `libdepthai-core.so` (otherwise static is preferred).
- `DEPTHAI_OPENCV_SUPPORT=1`: enable DepthAI-Core OpenCV support (if available).
- `DEPTHAI_DYNAMIC_CALIBRATION_SUPPORT=1`: toggle DepthAI-Core dynamic calibration support.
- `DEPTHAI_ENABLE_EVENTS_MANAGER=1`: toggle DepthAI-Core events manager.

## Troubleshooting

### “No available devices (… connected, but in use)”

This usually means another process already owns the device connection.

- Close other DepthAI apps (including Python scripts) and try again.
- Prefer `Pipeline::new().with_device(&device).build()?` so you don’t accidentally open two connections.

### Clang/libclang errors while building bindings

Make sure `clang` and `libclang-dev` are installed on Linux, and that LLVM is installed on Windows.

### Missing native libraries at runtime

By default, the build prefers static linking where possible. If you opt into shared linking (`DEPTHAI_SYS_LINK_SHARED=1`) you may need to ensure the runtime loader can find the shared libraries.

## Hardware integration tests

There is a `hit` feature flag intended for hardware integration testing:

```bash
cargo test --features hit
```

## License

See `LICENSE`.
