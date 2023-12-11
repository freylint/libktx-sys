# Rust bindings for libktx

This crate provides Rust bindings for the [libktx](https://github.khronos.org/KTX-Software/libktx/index.html) library.

This crate is currently in very early development and is not yet ready
for production use. Each release tracks the latest stable release of libktx at
the time of implementation. This may be ameliorated in the future.
See [Planned Features](#planned-features) for a list of features required before
I will consider this crate ready for production use.

## Building

This crate inherits the dependencies of the libktx library.
You will need to have the following installed alongsige the standard Rust toolchain:

- CMake
- LLVM (With the `LIBCLANG_PATH` environment variable set)
- Vulkan SDK (With the `VULKAN_SDK` environment variable set)

Note: If you wish to change the targetted library verion or regenerate the bindings,
re-enable the `bindgen` feature in the `Cargo.toml` file. This will regenerate the
bindings.

## Planned Features

- Seperate vulkan and opengl bindings through features
- Stop compiling unused libktx components
- Add proper Build testing for all platforms
- Only expose nessecary parts of the vulkan API
- Fix issues with generated documentation
- Work around need to have the vulkan SDK and LibClang installed
- Avoid regenergating bindings on every build
- Build Postprocressor to clean up generated bindings. (Breaking and 1.0 release)

## Crates.io Stewardship Notice

I am aware of the libktx-rs and libktx-rs-sys crates on crate.io.
The source code for this library has been archived, and I am considering it
abandoned. I'll be maintaining this as a sys crate alone, with the burden of
maintaining the higher level bindings left to the community.
