extern crate pumice;
extern crate pumice_vma;

/*
use pumice::extensions::DebugReport;
use pumice::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use std::os::raw::{c_char, c_void};

fn extension_names() -> Vec<*const i8> {
    vec![DebugReport::name().as_ptr()]
}

unsafe extern "system" fn vulkan_debug_callback(
    _: pumice::vk::DebugReportFlagsEXT,
    _: pumice::vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const c_char,
    p_message: *const c_char,
    _: *mut c_void,
) -> u32 {
    println!("{:?}", ::std::ffi::CStr::from_ptr(p_message));
    pumice::vk::FALSE
}

fn verify_result(result: pumice::vk::Result) {
    match result {
        pumice::vk::Result::SUCCESS => {
            // Success
        }
        _ => {
            panic!(format!("Vulkan/Allocator error occurred! {}", result));
        }
    }
}
*/

fn main() {
    println!("TODO - port sample from VMA");
}
