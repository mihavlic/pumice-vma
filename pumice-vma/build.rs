extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=vendor/VulkanMemoryAllocator/include");
    println!("cargo:rerun-if-changed=vendor/Vulkan-Headers/include/vulkan");
    println!("cargo:rerun-if-changed=wrapper");

    let mut build = cc::Build::new();

    build.include("vendor/VulkanMemoryAllocator/include");
    build.include("vendor/Vulkan-Headers/include/vulkan");
    build.include("wrapper");

    // Disable VMA_ASSERT when rust assertions are disabled
    #[cfg(not(debug_assertions))]
    build.define("NDEBUG", "");
    // #[cfg(debug_assertions)]
    // build.define("VMA_HEAVY_ASSERT", "assert");

    // We want to use the loader in pumice, instead of requiring us to link
    // in vulkan.dll/.dylib in addition to pumice. This is especially important
    // for MoltenVK, where there is no default installation path, unlike
    // Linux (pkconfig) and Windows (VULKAN_SDK environment variable).
    build.define("VMA_STATIC_VULKAN_FUNCTIONS", "0");

    // This prevents VMA from trying to fetch any remaining pointers
    // that are still null after using the loader in pumice, which can
    // cause linker errors.
    build.define("VMA_DYNAMIC_VULKAN_FUNCTIONS", "0");

    // TODO: Add some configuration options under crate features
    //#define VMA_DEDICATED_ALLOCATION 0
    //#define VMA_DEBUG_MARGIN 16
    //#define VMA_DEBUG_DETECT_CORRUPTION 1
    //#define VMA_DEBUG_INITIALIZE_ALLOCATIONS 1
    //#define VMA_DEBUG_MIN_BUFFER_IMAGE_GRANULARITY 256

    // Add the files we build
    let source_files = ["wrapper/vma_lib.cpp"];

    for source_file in &source_files {
        build.file(&source_file);
    }

    build
        .flag("-std=c++17")
        // disable all compiler warnings, as VMA has wontfix for anything regarding them
        // https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator/issues/182
        .flag("-w")
        .cpp_link_stdlib("stdc++")
        .warnings(false)
        .cpp(true);

    build.compile("vma_cpp");

    #[cfg(feature = "link_vulkan")]
    link_vulkan();

    #[cfg(feature = "generate_bindings")]
    generate_bindings("src/ffi.rs");
}

#[cfg(feature = "link_vulkan")]
fn link_vulkan() {
    use std::path::PathBuf;
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        if let Ok(vulkan_sdk) = env::var("VULKAN_SDK") {
            let mut vulkan_sdk_path = PathBuf::from(vulkan_sdk);

            if target.contains("x86_64") {
                vulkan_sdk_path.push("Lib");
            } else {
                vulkan_sdk_path.push("Lib32");
            }

            println!(
                "cargo:rustc-link-search=native={}",
                vulkan_sdk_path.to_str().unwrap()
            );
        }

        println!("cargo:rustc-link-lib=dylib=vulkan-1");
    } else {
        if target.contains("apple") {
            if let Ok(vulkan_sdk) = env::var("VULKAN_SDK") {
                let mut vulkan_sdk_path = PathBuf::from(vulkan_sdk);
                vulkan_sdk_path.push("macOS/lib");
                println!(
                    "cargo:rustc-link-search=native={}",
                    vulkan_sdk_path.to_str().unwrap()
                );
            } else {
                let lib_path = "wrapper/macOS/lib";
                println!("cargo:rustc-link-search=native={}", lib_path);
            }

            println!("cargo:rustc-link-lib=dylib=vulkan");
        }
    }
}

#[cfg(feature = "generate_bindings")]
fn generate_bindings(output_file: &str) {
    let options = generator::BuildOptions {
        includes: &["wrapper".into(), "vendor/Vulkan-Headers/include".into()],
        source_header: "vendor/VulkanMemoryAllocator/include/vk_mem_alloc.h".into(),
        output_path: output_file.into(),
    };

    generator::write_bindings(&options).unwrap();
}
