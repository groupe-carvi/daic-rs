# daic-rs

A Rust wrapper for Luxonis [DepthAI-Core V3](https://github.com/luxonis/depthai-core).

> [!CAUTION]  
> The crate is still in active development and may not be usable.

Our core goals with daic-rs are:
- [x] To make it 'Battery Included' by resolving any native dependancies for the crate users.
- [ ] Generates a clean raw rust binding inside the daic-sys crate using CXX and auto CXX.
- [ ] Create a rust friendly and safe API inside the daic-rs crate.
- [ ] Use alternative library or tools from the rust ecosystem where we can. (Like [kornia](https://github.com/kornia/kornia) and [rerun](https://github.com/rerun-io/rerun) instead of OpenCV for the examples).

> [!WARNING]
> ### About DepthAI-Core API Unstability
> As your can read in the DepthAI-Core repository disclaimer, luxonis don't yet provides API stability guaranties. It means that there could be some breaking changes to DepthAI-Core API that could impact daic-rs own API down the road and so we cannot guaranty any stability neither.
> The current version of the crate currently target [DepthAI-Core v3.2.1](https://github.com/luxonis/depthai-core/tree/v3.2.1).
> 
> <ins>We will try to follow the latest release offering a Windows prebuilt binary for the moment</ins>.

## Environment Setup

### Linux

> [!WARNING]
> For Linux system we only support debian based distributions for the moment and will investigate more as we go.

Since the crate is pulling and building Depth-AI dynamic library from source, you will need to make sure that your system has the required dependancies.

```sh
# Install dependencies

sudo apt -y install libclang-dev pkg-config cmake ninja-build python3 autoconf automake autoconf-archive libudev-dev libtool clang libssl-dev
```

### Windows

For Windows we use the prebuilt binary of DepthAI-Core found in the repository release section.

```powershell
# Install dependencies

# Install clang
winget install -e --id LLVM.LLVM

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
