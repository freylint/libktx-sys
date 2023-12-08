use std::env;
use std::path::{Path, PathBuf};

use cmake::Config;
use git2::build::RepoBuilder;

fn main() {
    // Git repo information
    let tag = "v4.2.1";
    let mut src_dest = env::var("OUT_DIR").unwrap();
    src_dest.push_str("/KTX-Software");

    // Build config
    #[cfg(debug_assertions)]
    let build_type = "Debug";

    #[cfg(not(debug_assertions))]
    let build_type = "Release";

    let vulkan_sdk_path = std::env::var("VULKAN_SDK").expect("VULKAN_SDK not found");
    let vulkan_include_path = Path::new(&vulkan_sdk_path).join("include");

    // Clone sources if nessecary
    if !Path::new(&src_dest).exists() {
        clone_repo(&src_dest, tag).expect("Failed to clone KTX-Software repository");
    }

    // Build dependencies
    build_ktx(&src_dest, build_type).expect("Failed to build KTX-Software");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args([
            format!("-I{}/include", src_dest),
            format!("-I{}", vulkan_include_path.display()),
        ])
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn clone_repo(dest: impl Into<PathBuf>, tag: &str) -> Result<(), git2::Error> {
    // Clone repo
    let repo = RepoBuilder::new()
        .clone(
            "https://github.com/KhronosGroup/KTX-Software.git",
            &dest.into(),
        )
        .expect("Failed to clone KTX-Software repository");

    // Change to the upstream version
    // TODO print the tag
    let obj = repo.revparse_single(tag).expect("Can't parse tag");

    repo.checkout_tree(&obj, None)
        .expect("Failed to checkout tag");
    repo.set_head_detached(obj.id())
        .expect("Failed to set HEAD to tag");

    Ok(())
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
        // TODO make cross platform
        .define("CMAKE_CXX_FLAGS", "/EHsc /WX- /wd4996")
        .build();

    // Tell cargo to tell rustc to link the system library
    println!("cargo:rustc-link-search=native={}/lib", dest.display());
    println!("cargo:rustc-link-lib=static=ktx");

    Ok(())
}
