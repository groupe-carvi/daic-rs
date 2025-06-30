# daic-rs
A Rust wrapper for Luxonis DepthAI-Core

# Requirements

```sh
# Install dependencies

sudo apt -y install libclang-dev cmake ninja-build python3 autoconf automake autoconf-archive libudev-dev libtool clang
```

# Build

# Archive: This is the initial working build script for DepthAI-Core.

```sh
git clone --recurse-submodules https://github.com/luxonis/depthai-core.git

export VCPKG_CMAKE_GENERATOR=Ninja && export CMAKE_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu && export CMAKE_INCLUDE_PATH=/usr/include && cmake --fresh  -S . -B build -DCMAKE_C_COMPILER=/usr/bin/gcc -DCMAKE_CXX_COMPILER=/usr/bin/g++ -DCMAKE_MAKE_PROGRAM=/usr/bin/ninja -DCMAKE_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu -DDEPTHAI_OPENCV_SUPPORT=OFF -DCMAKE_INCLUDE_PATH=/usr/include -G Ninja
```
