use std::path::{Path, PathBuf};

use bindgen::Bindings;
use cmake::Config;

const SOURCE_DIR: &str = "vendor/KTX-Software";
const VK_INC_DIR: &str = &std::env::var("VULKAN_SDK").expect("VULKAN_SDK not found");

fn main() {
    // Build config
    #[cfg(debug_assertions)]
    let build_type = "Debug";

    #[cfg(not(debug_assertions))]
    let build_type = "Release";

    // Build dependencies
    build_ktx(SOURCE_DIR, build_type).expect("Failed to build KTX-Software");

    // Generate bindings
    gen_bindings()
        .write_to_file("src/ffi.rs")
        .expect("Couldn't write bindings!");
}

fn build_ktx(
    srcs: impl Into<PathBuf>,
    build_type: &'_ str,
) -> Result<(), Box<dyn std::error::Error>> {
    let srcs: PathBuf = srcs.into();

    let dest = Config::new(srcs.as_path())
        .define("KTX_FEATURE_STATIC_LIBRARY", "ON")
        .define("CMAKE_BUILD_TYPE", build_type)
        .define("CMAKE_CXX_STANDARD", "17")
        .define("CMAKE_CXX_FLAGS", get_flags())
        // TODO Re-enable this feature when patent expires.
        .define("SUPPORT_SOFTWARE_ETC_UNPACK", "OFF")
        .build();

    // Tell cargo to tell rustc to link the system library
    println!("cargo:rustc-link-search=native={}/lib", dest.display());
    println!("cargo:rustc-link-lib=static=ktx");

    Ok(())
}

fn gen_bindings() -> Bindings {
    let vulkan_include_path = Path::new(VK_INC_DIR).join("include");

    bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args([
            format!("-I{}/include", SOURCE_DIR),
            format!("-I{}/lib", SOURCE_DIR),
            format!("-I{}", vulkan_include_path.display()),
        ])
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
}

// TODO support non-msvc compilers on windows
fn get_flags() -> &'static str {
    if cfg!(target_os = "windows") {
        if std::env::var("TARGET").unwrap().contains("msvc") {
            "/EHsc /WX- /wd4996"
        } else {
            ""
        }
    } else {
        // Other OS
        "-Wno-error -Wno-error=deprecated-declarations -Wno-error=unused-function -Wno-error=unused-variable -Wno-error=unused-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-value -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-result -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error="
    }
}
