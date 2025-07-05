# daic-rs

A Rust wrapper for Luxonis DepthAI-Core

## Environement Setuo

### Linux

```sh
# Install dependencies

sudo apt -y install libclang-dev pkg-config cmake ninja-build python3 autoconf automake autoconf-archive libudev-dev libtool clang libssl-dev
```

### Windows

Make sure to have Visual Studio 2022 installed with C++ development tools and CMake.

```powershell
# Install dependencies

```

## Build



## Archive: Reminder on how to build depthai-core

### Linux

```sh
git clone --recurse-submodules https://github.com/luxonis/depthai-core.git

export VCPKG_CMAKE_GENERATOR=Ninja && export CMAKE_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu && export CMAKE_INCLUDE_PATH=/usr/include && cmake --fresh  -S . -B build -DCMAKE_C_COMPILER=/usr/bin/gcc -DCMAKE_CXX_COMPILER=/usr/bin/g++ -DCMAKE_MAKE_PROGRAM=/usr/bin/ninja -DCMAKE_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu -DDEPTHAI_OPENCV_SUPPORT=OFF -DCMAKE_INCLUDE_PATH=/usr/include -G Ninja
```

### Windows

```powershell

git clone --recurse-submodules https://github.com/luxonis/depthai-core.git
cmake --fresh -S . -B build -D'BUILD_SHARED_LIBS=ON' -'DDEPTHAI_OPENCV_SUPPORT=OFF' -G 'Visual Studio 17 2022'
cmake --build build --parallel [num CPU cores]

```
