#![allow(dead_code)]

use std::{
    env,
    path::{Path, PathBuf},
};

use bindgen::Bindings;
use cmake::Config;

const SOURCE_DIR: &str = "vendor/KTX-Software";
fn get_vk_inc_dir() -> String {
    env::var("VULKAN_SDK").expect("VULKAN_SDK not found")
}

fn main() {
    // Build config
    #[cfg(debug_assertions)]
    let build_type = "Debug";

    #[cfg(not(debug_assertions))]
    let build_type = "Release";

    // Build dependencies
    let out = build_ktx(SOURCE_DIR, build_type);
    link_ktx(out);

    // Generate bindings
    #[cfg(feature = "bindgen")]
    gen_bindings()
        .write_to_file("src/ffi.rs")
        .expect("Couldn't write bindings!");
}

fn build_ktx(srcs: impl AsRef<Path>, build_type: &'_ str) -> PathBuf {
    Config::new(srcs.as_ref())
        .define("KTX_FEATURE_STATIC_LIBRARY", "ON")
        .define("KTX_FEATURE_TOOLS", "OFF")
        .define("KTX_FEATURE_LOADTEST", "OFF")
        .define("KTX_FEATURE_TESTS", "OFF")
        .define("CMAKE_BUILD_TYPE", build_type)
        .define("CMAKE_CXX_STANDARD", "17")
        .define("CMAKE_CXX_FLAGS", get_flags())
        .define("SUPPORT_SOFTWARE_ETC_UNPACK", "OFF")
        .build()
}

fn link_ktx(dest: impl AsRef<Path>) {
    // Tell cargo to tell rustc to link the system library
    println!(
        "cargo:rustc-link-search=native={}/lib",
        dest.as_ref().display()
    );
    println!("cargo:rustc-link-lib=static=ktx");
}

fn gen_bindings() -> Bindings {
    bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args(get_clang_args())
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
}

fn get_clang_args() -> Vec<String> {
    let vulkan_include_path = Path::new(&get_vk_inc_dir()).join("include");
    [
        format!("-I{}/include", SOURCE_DIR),
        format!("-I{}/lib", SOURCE_DIR),
        format!("-I{}", vulkan_include_path.display()),
    ]
    .to_vec()
}

// TODO support non-msvc compilers on windows
fn get_flags() -> &'static str {
    if cfg!(target_os = "windows") {
        if env::var("TARGET").unwrap().contains("msvc") {
            "/EHsc /WX- /wd4996"
        } else {
            ""
        }
    } else {
        // Other OS
        "-Wno-error -Wno-error=deprecated-declarations -Wno-error=unused-function -Wno-error=unused-variable -Wno-error=unused-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-value -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-result -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error=unused-function -Wno-error=unused-label -Wno-error=unused-local-typedefs -Wno-error=unused-macros -Wno-error=unused-parameter -Wno-error=unused-result -Wno-error=unused-value -Wno-error=unused-variable -Wno-error=unused-but-set-parameter -Wno-error=unused-but-set-variable -Wno-error="
    }
}
