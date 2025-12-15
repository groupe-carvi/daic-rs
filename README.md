# daic-rs

Experimental Rust bindings + safe-ish wrapper for Luxonis **DepthAI-Core v3**.

- High-level crate: `daic-rs` (Rust API)
- Low-level crate: `daic-sys` (builds DepthAI-Core and exposes an FFI surface via `autocxx`)

> [!CAUTION]
> This project is experimental and in active development. APIs and behavior can change.

> [!WARNING]
> DepthAI-Core itself does not provide strong API stability guarantees yet. This repo currently targets **DepthAI-Core `v3.2.1`**.

## What’s in this repo

### Crates

- `daic-sys`
	- Builds DepthAI-Core and its dependencies (in `daic-sys/builds/`).
	- Compiles a small C++ wrapper (`daic-sys/wrapper/wrapper.cpp`) and generates Rust bindings using `autocxx`.
- `daic-rs`
	- Safe(-er) Rust wrapper types like `Device`, `Pipeline`, typed camera helpers, and a generic node API.

### Repository layout (high-level)

```
daic-sys/            # FFI crate (build script + wrapper)
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

## Build

From the repo root:

```bash
cargo build
```

Notes:

- The first build can take a while because DepthAI-Core is fetched/built and dependencies are prepared.
- Build artifacts for native code live under `daic-sys/builds/`.

## Run examples

```bash
cargo run --example pipeline_creation
cargo run --example camera
cargo run --example camera_output
```

## API overview

### Device ownership

DepthAI device connections are typically exclusive. `daic-rs` mirrors the common C++ pattern of sharing one device connection:

- `Device::new()` opens/returns a device handle.
- `Device::clone()` / `Device::try_clone()` creates another handle to the same underlying connection.
- `Pipeline::with_device(&device)` binds a pipeline to an existing device connection (recommended).
- `Pipeline::start_default()` starts the pipeline using its internally-held device.

### Generic node linking

The generic node API supports linking by explicit port names *or* by choosing a compatible default when you omit port names.
For example, `StereoDepth` expects inputs named `"left"` and `"right"`:

```rust
let stereo = pipeline.create_node(NodeKind::StereoDepth)?;
left_camera.as_node().link(None, None, &stereo, None, Some("left"))?;
right_camera.as_node().link(None, None, &stereo, None, Some("right"))?;
```

## Environment variables (advanced)

`daic-sys` exposes a few environment variables that affect native builds:

- `DEPTHAI_CORE_ROOT`: override the DepthAI-Core checkout directory.
- `DAIC_SYS_LINK_SHARED=1`: prefer linking against `libdepthai-core.so` (otherwise static is preferred).
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

By default, the build prefers static linking where possible. If you opt into shared linking (`DAIC_SYS_LINK_SHARED=1`) you may need to ensure the runtime loader can find the shared libraries.

## Hardware integration tests

There is a `hit` feature flag intended for hardware integration testing:

```bash
cargo test --features hit
```

## License

See `LICENSE`.
