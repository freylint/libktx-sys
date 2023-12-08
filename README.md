# Rust bindings for libktx

This crate provides Rust bindings for the [libktx](https://github.khronos.org/KTX-Software/libktx/index.html) library.

This crate is currently in very early development and is not yet ready
for production use.

See [Planned Features](#planned-features) for a list of features required before
I will consider this crate ready for production use.

## Building

This crate inherits the dependencies of the libktx library.
You will need to have the following installed alongsige the standard Rust toolchain:

- CMake
- LLVM (With the `LIBCLANG_PATH` environment variable set)
- Vulkan SDK (With the `VULKAN_SDK` environment variable set)

## Planned Features

- Seperate vulkan and opengl bindings through features
- Stop compiling unused libktx components
