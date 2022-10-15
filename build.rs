#[cfg(feature = "generate_bindings")]
extern crate bindgen;
extern crate cc;

use std::env;

fn main() {
    let mut build = cc::Build::new();

    build.include("vendor/VulkanMemoryAllocator/include");
    build.include("vendor/Vulkan-Headers/include/vulkan");
    build.include("wrapper");

    // Disable VMA_ASSERT when rust assertions are disabled
    #[cfg(not(debug_assertions))]
    build.define("NDEBUG", "");

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
    //#define VMA_HEAVY_ASSERT(expr) assert(expr)
    //#define VMA_USE_STL_CONTAINERS 1
    //#define VMA_DEDICATED_ALLOCATION 0
    //#define VMA_DEBUG_MARGIN 16
    //#define VMA_DEBUG_DETECT_CORRUPTION 1
    //#define VMA_DEBUG_INITIALIZE_ALLOCATIONS 1
    //#define VMA_DEBUG_MIN_BUFFER_IMAGE_GRANULARITY 256

    #[cfg(feature = "recording")]
    build.define("VMA_RECORDING_ENABLED", "1");

    // Add the files we build
    let source_files = ["wrapper/vma_lib.cpp"];

    for source_file in &source_files {
        build.file(&source_file);
    }

    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .flag("-Wno-nullability-completeness")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("ios") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("android") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("c++")
            .cpp(true);
    } else if target.contains("linux") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    } else if target.contains("windows") && target.contains("gnu") {
        build
            .flag("-std=c++17")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .flag("-Wno-type-limits")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    }

    build.compile("vma_cpp");

    link_vulkan();
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

#[cfg(not(feature = "link_vulkan"))]
fn link_vulkan() {}

#[cfg(feature = "generate_bindings")]
fn generate_bindings(output_file: &str) {
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./wrapper")
        .clang_arg("-I./vendor/Vulkan-Headers/include")
        .header("vendor/VulkanMemoryAllocator/include/vk_mem_alloc.h")
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .blocklist_type("__darwin_.*")
        .allowlist_function("vma.*")
        .allowlist_function("PFN_vma.*")
        .allowlist_type("Vma.*")
        .parse_callbacks(Box::new(FixPumiceTypes))
        // custom definition to allow rust code to pass in null function pointers so that they get loaded by VMA
        .blocklist_type("VmaVulkanFunctions")
        .blocklist_type("Vk.*")
        .blocklist_type("PFN_vk.*")
        .raw_line(META)
        .raw_line("use pumice::vk::*;")
        .raw_line(PFN_DEFINITIONS)
        .trust_clang_mangling(false)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings!");

    bindings
        .write_to_file(std::path::Path::new(output_file))
        .expect("Unable to write bindings!");
}

#[cfg(not(feature = "generate_bindings"))]
fn generate_bindings(_: &str) {}

#[cfg(feature = "generate_bindings")]
#[derive(Debug)]
struct FixPumiceTypes;

#[cfg(feature = "generate_bindings")]
impl bindgen::callbacks::ParseCallbacks for FixPumiceTypes {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        original_item_name.strip_prefix("Vk").map(ToOwned::to_owned)
    }

    // When ignoring `Vk` types, bindgen loses derives for some types. Quick workaround.
    fn add_derives(&self, name: &str) -> Vec<String> {
        if name.starts_with("VmaAllocationInfo") || name.starts_with("VmaDefragmentationStats") {
            vec!["Debug".into(), "Copy".into(), "Clone".into()]
        } else {
            vec![]
        }
    }
}

#[cfg(feature = "generate_bindings")]
const META: &str = r#"#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]"#;

// at this time pumice does not generate type aliases for all function's function pointers so this must be done manually
#[cfg(feature = "generate_bindings")]
const PFN_DEFINITIONS: &str = r#"
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(
    instance: pumice::vk10::Instance,
    p_name: *const std::os::raw::c_char,
) -> pumice::vk10::PfnVoidFunction;

pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    p_name: *const std::os::raw::c_char,
) -> pumice::vk10::PfnVoidFunction;

pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
    physical_device: pumice::vk10::PhysicalDevice,
    p_properties: *mut pumice::vk10::PhysicalDeviceProperties,
);

pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
    physical_device: pumice::vk10::PhysicalDevice,
    p_memory_properties: *mut pumice::vk10::PhysicalDeviceMemoryProperties,
);

pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    p_allocate_info: *const pumice::vk10::MemoryAllocateInfo,
    p_allocator: *const pumice::vk10::AllocationCallbacks,
    p_memory: *mut pumice::vk10::DeviceMemory,
) -> pumice::vk10::Result;

pub type PFN_vkFreeMemory = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    memory: pumice::vk10::DeviceMemory,
    p_allocator: *const pumice::vk10::AllocationCallbacks,
);

pub type PFN_vkMapMemory = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    memory: pumice::vk10::DeviceMemory,
    offset: pumice::vk10::DeviceSize,
    size: pumice::vk10::DeviceSize,
    flags: pumice::vk10::MemoryMapFlags,
    pp_data: *mut *mut std::os::raw::c_void,
) -> pumice::vk10::Result;

pub type PFN_vkUnmapMemory =
    unsafe extern "system" fn(device: pumice::vk10::Device, memory: pumice::vk10::DeviceMemory);

pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    memory_range_count: u32,
    p_memory_ranges: *const pumice::vk10::MappedMemoryRange,
) -> pumice::vk10::Result;

pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    memory_range_count: u32,
    p_memory_ranges: *const pumice::vk10::MappedMemoryRange,
) -> pumice::vk10::Result;

pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    buffer: pumice::vk10::Buffer,
    memory: pumice::vk10::DeviceMemory,
    memory_offset: pumice::vk10::DeviceSize,
) -> pumice::vk10::Result;

pub type PFN_vkBindImageMemory = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    image: pumice::vk10::Image,
    memory: pumice::vk10::DeviceMemory,
    memory_offset: pumice::vk10::DeviceSize,
) -> pumice::vk10::Result;

pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    buffer: pumice::vk10::Buffer,
    p_memory_requirements: *mut pumice::vk10::MemoryRequirements,
);

pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    image: pumice::vk10::Image,
    p_memory_requirements: *mut pumice::vk10::MemoryRequirements,
);

pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    p_create_info: *const pumice::vk10::BufferCreateInfo,
    p_allocator: *const pumice::vk10::AllocationCallbacks,
    p_buffer: *mut pumice::vk10::Buffer,
) -> pumice::vk10::Result;

pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    buffer: pumice::vk10::Buffer,
    p_allocator: *const pumice::vk10::AllocationCallbacks,
);

pub type PFN_vkCreateImage = unsafe extern "system" fn(
        device: pumice::vk10::Device,
        p_create_info: *const pumice::vk10::ImageCreateInfo,
        p_allocator: *const pumice::vk10::AllocationCallbacks,
        p_image: *mut pumice::vk10::Image,
) -> pumice::vk10::Result;

pub type PFN_vkDestroyImage = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    image: pumice::vk10::Image,
    p_allocator: *const pumice::vk10::AllocationCallbacks,
);

pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(
    command_buffer: pumice::vk10::CommandBuffer,
    src_buffer: pumice::vk10::Buffer,
    dst_buffer: pumice::vk10::Buffer,
    region_count: u32,
    p_regions: *const pumice::vk10::BufferCopy,
);

pub type PFN_vkGetBufferMemoryRequirements2KHR = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    p_info: *const pumice::vk11::BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut pumice::vk11::MemoryRequirements2,
);

pub type PFN_vkGetImageMemoryRequirements2KHR = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    p_info: *const pumice::vk11::ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut pumice::vk11::MemoryRequirements2,
);

pub type PFN_vkBindBufferMemory2KHR = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    bind_info_count: u32,
    p_bind_infos: *const pumice::vk11::BindBufferMemoryInfo,
) -> pumice::vk10::Result;

pub type PFN_vkBindImageMemory2KHR = unsafe extern "system" fn(
    device: pumice::vk10::Device,
    bind_info_count: u32,
    p_bind_infos: *const pumice::vk11::BindImageMemoryInfo,
) -> pumice::vk10::Result;

pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = unsafe extern "system" fn(
    physical_device: pumice::vk10::PhysicalDevice,
    p_memory_properties: *mut pumice::vk11::PhysicalDeviceMemoryProperties2,
);

// using the version promoted to Vulkan 1.1
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;

#[doc = " \\brief Pointers to some Vulkan functions - a subset used by the library."]
#[doc = ""]
#[doc = "Used in VmaAllocatorCreateInfo::pVulkanFunctions."]
#[doc = "Fields are wrapped in option to allow rust code to pass null function pointers so that they get loaded by VMA."]
#[repr(C)]
pub struct VmaVulkanFunctions {
    #[doc = " Required when using VMA_DYNAMIC_VULKAN_FUNCTIONS."]
    pub vkGetInstanceProcAddr: Option<PFN_vkGetInstanceProcAddr>,
    #[doc = " Required when using VMA_DYNAMIC_VULKAN_FUNCTIONS."]
    pub vkGetDeviceProcAddr: Option<PFN_vkGetDeviceProcAddr>,
    pub vkGetPhysicalDeviceProperties: Option<PFN_vkGetPhysicalDeviceProperties>,
    pub vkGetPhysicalDeviceMemoryProperties: Option<PFN_vkGetPhysicalDeviceMemoryProperties>,
    pub vkAllocateMemory: Option<PFN_vkAllocateMemory>,
    pub vkFreeMemory: Option<PFN_vkFreeMemory>,
    pub vkMapMemory: Option<PFN_vkMapMemory>,
    pub vkUnmapMemory: Option<PFN_vkUnmapMemory>,
    pub vkFlushMappedMemoryRanges: Option<PFN_vkFlushMappedMemoryRanges>,
    pub vkInvalidateMappedMemoryRanges: Option<PFN_vkInvalidateMappedMemoryRanges>,
    pub vkBindBufferMemory: Option<PFN_vkBindBufferMemory>,
    pub vkBindImageMemory: Option<PFN_vkBindImageMemory>,
    pub vkGetBufferMemoryRequirements: Option<PFN_vkGetBufferMemoryRequirements>,
    pub vkGetImageMemoryRequirements: Option<PFN_vkGetImageMemoryRequirements>,
    pub vkCreateBuffer: Option<PFN_vkCreateBuffer>,
    pub vkDestroyBuffer: Option<PFN_vkDestroyBuffer>,
    pub vkCreateImage: Option<PFN_vkCreateImage>,
    pub vkDestroyImage: Option<PFN_vkDestroyImage>,
    pub vkCmdCopyBuffer: Option<PFN_vkCmdCopyBuffer>,
    #[doc = " Fetch \"vkGetBufferMemoryRequirements2\" on Vulkan >= 1.1, fetch \"vkGetBufferMemoryRequirements2KHR\" when using VK_KHR_dedicated_allocation extension."]
    pub vkGetBufferMemoryRequirements2KHR: Option<PFN_vkGetBufferMemoryRequirements2KHR>,
    #[doc = " Fetch \"vkGetImageMemoryRequirements 2\" on Vulkan >= 1.1, fetch \"vkGetImageMemoryRequirements2KHR\" when using VK_KHR_dedicated_allocation extension."]
    pub vkGetImageMemoryRequirements2KHR: Option<PFN_vkGetImageMemoryRequirements2KHR>,
    #[doc = " Fetch \"vkBindBufferMemory2\" on Vulkan >= 1.1, fetch \"vkBindBufferMemory2KHR\" when using VK_KHR_bind_memory2 extension."]
    pub vkBindBufferMemory2KHR: Option<PFN_vkBindBufferMemory2KHR>,
    #[doc = " Fetch \"vkBindImageMemory2\" on Vulkan >= 1.1, fetch \"vkBindImageMemory2KHR\" when using VK_KHR_bind_memory2 extension."]
    pub vkBindImageMemory2KHR: Option<PFN_vkBindImageMemory2KHR>,
    pub vkGetPhysicalDeviceMemoryProperties2KHR: Option<PFN_vkGetPhysicalDeviceMemoryProperties2KHR>,
}"#;
