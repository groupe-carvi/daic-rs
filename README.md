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
	- Builds DepthAI-Core and its dependencies (in `depthai-sys/builds/`).
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
- Build artifacts for native code live under `depthai-sys/builds/`.

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

This section is generated from the native DepthAI-Core C++ examples vendored in this repo under `depthai-sys/builds/depthai-core/examples/cpp`.

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

## API overview

### Device platforms

`depthai-rs` supports multiple DepthAI hardware platforms:

- `DevicePlatform::Rvc2` - RVC2-based devices (OAK-D, OAK-D-Lite, etc.)
- `DevicePlatform::Rvc3` - RVC3-based devices
- `DevicePlatform::Rvc4` - RVC4-based devices (latest generation)

Query the device platform:

```rust
let device = Device::new()?;
let platform = device.platform()?;
```

### Device features

```rust
// Query connected cameras
let cameras = device.connected_cameras()?;

// Control IR laser dot projector (on supported devices)
device.set_ir_laser_dot_projector_intensity(0.3)?;

// Check if device is still connected
if device.is_connected() {
    // Device is still connected, but note that it could disconnect between
    // this check and any subsequent operations, so always handle errors.
}
```

### Device ownership

DepthAI device connections are typically exclusive. `depthai-rs` mirrors the common C++ pattern of sharing one device connection:

- `Device::new()` opens/returns a device handle.
- `Device::clone()` / `Device::try_clone()` creates another handle to the same underlying connection.
- `Pipeline::with_device(&device)` binds a pipeline to an existing device connection (recommended).
- `Pipeline::start()` starts the pipeline using its associated device connection.

### Creating nodes

`depthai-rs` provides multiple ways to create device nodes:

#### Generic API (type-safe)

```rust
// Nodes without parameters
let stereo = pipeline.create::<StereoDepthNode>()?;
let rgbd = pipeline.create::<RgbdNode>()?;

// Nodes with parameters
let camera = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
```

#### By C++ class name

```rust
let node = pipeline.create_node("dai::node::StereoDepth")?;
```

#### Composite nodes

Use the `#[depthai_composite]` macro to bundle multiple nodes:

```rust
#[depthai_composite]
pub struct CameraStereoBundle {
    pub left: CameraNode,
    pub right: CameraNode,
    pub stereo: StereoDepthNode,
}

impl CameraStereoBundle {
    pub fn new(pipeline: &Pipeline) -> Result<Self> {
        let left = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamB)?;
        let right = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamC)?;
        let stereo = pipeline.create::<StereoDepthNode>()?;
        
        // Link nodes
        left.raw()?.link(&stereo.left()?)?;
        right.raw()?.link(&stereo.right()?)?;
        
        Ok(Self { left, right, stereo })
    }
}

// Use as a regular node
let bundle = pipeline.create::<CameraStereoBundle>()?;
```

### Host nodes

`depthai-rs` supports custom processing nodes written in Rust:

#### HostNode

Synchronous processing node using the `#[depthai_host_node]` macro:

```rust
#[depthai_host_node]
struct FrameLogger;

impl FrameLogger {
    fn process(&mut self, group: &MessageGroup) -> Option<Buffer> {
        if let Ok(Some(frame)) = group.get_frame("in") {
            println!("Frame: {}x{}", frame.width(), frame.height());
        }
        None
    }
}

let host = pipeline.create_host_node(FrameLogger)?;
```

#### ThreadedHostNode

Asynchronous processing node with its own thread using `#[depthai_threaded_host_node]`:

```rust
#[depthai_threaded_host_node]
struct FrameProcessor {
    input: Input,
}

impl FrameProcessor {
    fn run(&mut self, ctx: &ThreadedHostNodeContext) {
        while ctx.is_running() {
            if let Ok(frame) = self.input.get_frame() {
                // Process frame
            }
        }
    }
}

let host = pipeline.create_threaded_host_node(|node| {
    let input = node.create_input(Some("in"))?;
    Ok(FrameProcessor { input })
})?;
```

#### RerunHostNode (optional rerun feature)

Visualize data streams using Rerun:

```rust
let host = pipeline.create_with::<RerunHostNode, _>(RerunHostNodeConfig {
    viewer: RerunViewer::Web(RerunWebConfig {
        // Don't auto-open browser in remote/container environments
        open_browser: false,
        ..Default::default()
    }),
    ..Default::default()
})?;
out.link(&host.input("in")?)?;
```

Requires the `rerun` feature and Tokio runtime support.

### Node linking

Link nodes by output to input, with optional port names:

```rust
// Simple linking
camera_out.link(&stereo.left()?)?;

// With explicit port names
depth_out.link_to(&align, Some("input"))?;
color_out.link_to(&align, Some("inputAlignTo"))?;
```

### Camera configuration

Configure camera outputs with detailed options:

```rust
let out = camera.request_output(CameraOutputConfig {
    size: (640, 400),
    frame_type: Some(ImageFrameType::RGB888i),
    resize_mode: ResizeMode::Crop,
    fps: Some(30.0),
    enable_undistortion: Some(true),
})?;
```

Supported frame types include: `RGB888i`, `GRAY8`, `NV12`, and more.

### Stereo depth

Configure stereo depth processing:

```rust
let stereo = pipeline.create::<StereoDepthNode>()?;
stereo.set_default_profile_preset(StereoPresetMode::Robotics);
stereo.set_left_right_check(true);
stereo.set_subpixel(true);
stereo.enable_distortion_correction(true);
```

### RGBD and point clouds

Generate aligned RGB-D data and point clouds:

```rust
let rgbd = pipeline.create::<RgbdNode>()?;
rgbd.set_depth_unit(DepthUnit::Meter);
rgbd.build()?;

// Link color and depth inputs
color_out.link_to(rgbd.as_node(), Some("inColorSync"))?;
depth_out.link_to(rgbd.as_node(), Some("inDepthSync"))?;

// Get outputs
let q_pcl = rgbd.as_node().output("pcl")?.create_queue(2, false)?;
let q_rgbd = rgbd.as_node().output("rgbd")?.create_queue(2, false)?;

// Retrieve data
if let Some(pcl) = q_pcl.try_next_pointcloud()? {
    for point in pcl.points() {
        // Access point.x, point.y, point.z, point.r, point.g, point.b
    }
}
```

## Procedural macros

`depthai-rs` provides several procedural macros to simplify node creation:

### `#[native_node_wrapper]`

Wraps native DepthAI nodes with type-safe Rust interfaces:

```rust
#[native_node_wrapper(
    native = "dai::node::Camera",
    inputs(inputControl, mockIsp),
    outputs(raw)
)]
pub struct CameraNode {
    node: crate::pipeline::Node,
}
```

### `#[depthai_host_node]`

Creates synchronous host nodes:

```rust
#[depthai_host_node]
struct MyProcessor;

impl MyProcessor {
    fn process(&mut self, group: &MessageGroup) -> Option<Buffer> {
        // Process messages
        None
    }
}
```

### `#[depthai_threaded_host_node]`

Creates asynchronous threaded host nodes:

```rust
#[depthai_threaded_host_node]
struct MyThreadedProcessor {
    input: Input,
}

impl MyThreadedProcessor {
    fn run(&mut self, ctx: &ThreadedHostNodeContext) {
        // Run in dedicated thread
    }
}
```

### `#[depthai_composite]`

Bundles multiple nodes into a composite node:

```rust
#[depthai_composite]
pub struct MyComposite {
    pub camera: CameraNode,
    pub processor: StereoDepthNode,
}
```

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
- Prefer `Pipeline::with_device(&device)` so you don’t accidentally open two connections.

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
