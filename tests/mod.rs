extern crate pumice;
extern crate pumice_vma;

use pumice::{
    loader::{
        tables::{DeviceTable, EntryTable, InstanceTable},
        InstanceLoader,
    },
    DeviceWrapper,
};

#[cfg(feature = "tests_debug_callback")]
unsafe extern "system" fn vulkan_debug_callback(
    _: pumice::vk::DebugReportFlagsEXT,
    _: pumice::vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const std::os::raw::c_char,
    p_message: *const std::os::raw::c_char,
    _: *mut c_void,
) -> u32 {
    println!("{:?}", ::std::ffi::CStr::from_ptr(p_message));
    pumice::vk::FALSE
}

pub struct TestHarness {
    // if vulkan is loaded with libloading, the lifetime of the loaded library's memory is tied to this object so it must be preserved
    pub loader: pumice::loader::EntryLoader,
    // these tables store the function pointers to all the vulkan commands
    pub tables: Box<(EntryTable, InstanceTable, DeviceTable)>,
    pub entry: pumice::EntryWrapper,
    pub instance: pumice::InstanceWrapper,
    pub device: pumice::DeviceWrapper,
    pub physical_device: pumice::vk::PhysicalDevice,
    #[cfg(feature = "tests_debug_callback")]
    pub debug_callback: pumice::vk::DebugReportCallbackEXT,
}

impl Drop for TestHarness {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_device(None);
            #[cfg(feature = "tests_debug_callback")]
            self.instance
                .destroy_debug_report_callback_ext(self.debug_callback, None);
            self.instance.destroy_instance(None);
        }
    }
}
impl TestHarness {
    pub fn new() -> Self {
        let app_name = ::std::ffi::CString::new("vk-mem testing").unwrap();
        let api_version = pumice::vk::make_api_version(0, 1, 0, 0);

        #[allow(unused_mut)]
        let mut config = pumice::util::ApiLoadConfig::new(api_version);
        #[cfg(feature = "tests_debug_callback")]
        config.add_extension(pumice::extensions::ext_debug_report::EXT_DEBUG_REPORT_EXTENSION_NAME);

        let app_info = pumice::vk::ApplicationInfo {
            p_application_name: app_name.as_ptr(),
            application_version: 0,
            p_engine_name: app_name.as_ptr(),
            engine_version: 0,
            api_version,
            ..Default::default()
        };

        let layer_names = [pumice::cstr!("VK_LAYER_KHRONOS_validation").as_ptr()];

        let extension_names_raw = config.get_instance_extensions();
        let create_info = pumice::vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_layer_names: layer_names.as_ptr(),
            enabled_layer_count: layer_names.len() as _,
            pp_enabled_extension_names: extension_names_raw.as_ptr(),
            enabled_extension_count: extension_names_raw.len() as _,
            ..Default::default()
        };

        let mut tables = Box::new((
            EntryTable::new_empty(),
            InstanceTable::new_empty(),
            DeviceTable::new_empty(),
        ));

        let loader =
            unsafe { pumice::loader::EntryLoader::new().expect("Failed to create entry loader") };

        tables.0.load(&loader);

        let entry = unsafe { pumice::EntryWrapper::new(&tables.0) };

        let instance_handle = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        };

        let instance_loader = unsafe { InstanceLoader::new(instance_handle, &loader) };
        tables.1.load(&instance_loader, &config);
        // loading device commands from the instance for greater flexibility, this may go through additional dispatch so is not as performant
        tables.2.load(&instance_loader, &config);

        let instance = unsafe { pumice::InstanceWrapper::new(instance_handle, &tables.1) };

        #[cfg(feature = "tests_debug_callback")]
        let debug_callback = unsafe {
            let debug_info = pumice::vk::DebugReportCallbackCreateInfoEXT {
                flags: pumice::vk::DebugReportFlagsEXT::ERROR
                    | pumice::vk::DebugReportFlagsEXT::WARNING
                    | pumice::vk::DebugReportFlagsEXT::PERFORMANCE_WARNING,
                pfn_callback: Some(vulkan_debug_callback),
                ..Default::default()
            };
            instance
                .create_debug_report_callback_ext(&debug_info, None)
                .expect("Debug report callback creation error")
        };

        let (physical_devices, _) = unsafe {
            instance
                .enumerate_physical_devices(None)
                .expect("Physical device enumeration error")
        };

        let (physical_device, queue_family_index) = unsafe {
            physical_devices
                .iter()
                .map(|physical_device| {
                    instance
                        .get_physical_device_queue_family_properties(*physical_device, None)
                        .iter()
                        .enumerate()
                        .filter_map(|(index, _)| Some((*physical_device, index)))
                        .nth(0)
                })
                .filter_map(|v| v)
                .nth(0)
                .expect("Couldn't find suitable device.")
        };

        let priorities = [1.0];

        let queue_info = [pumice::vk::DeviceQueueCreateInfo {
            queue_family_index: queue_family_index as _,
            p_queue_priorities: priorities.as_ptr(),
            queue_count: priorities.len() as _,
            ..Default::default()
        }];

        let device_create_info = pumice::vk::DeviceCreateInfo {
            p_queue_create_infos: queue_info.as_ptr(),
            queue_create_info_count: queue_info.len() as _,
            ..Default::default()
        };

        let device_handle = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .unwrap()
        };

        let device = unsafe { DeviceWrapper::new(device_handle, &tables.2) };

        TestHarness {
            loader,
            tables,
            entry,
            instance,
            device,
            physical_device,
            #[cfg(feature = "tests_debug_callback")]
            debug_callback,
        }
    }

    pub fn create_allocator(&self) -> pumice_vma::Allocator {
        let create_info = pumice_vma::AllocatorCreateInfo::new(
            &self.instance,
            &self.device,
            &self.physical_device,
        );
        pumice_vma::Allocator::new(create_info).unwrap()
    }
}

#[test]
fn create_harness() {
    let _ = TestHarness::new();
}

#[test]
fn create_allocator() {
    let harness = TestHarness::new();
    let _ = harness.create_allocator();
}

#[test]
fn create_gpu_buffer() {
    let harness = TestHarness::new();
    let allocator = harness.create_allocator();
    let allocation_info =
        pumice_vma::AllocationCreateInfo::new().usage(pumice_vma::MemoryUsage::GpuOnly);

    unsafe {
        let (buffer, allocation, allocation_info) = allocator
            .create_buffer(
                &pumice::vk::BufferCreateInfo {
                    size: 16 * 1024,
                    usage: pumice::vk::BufferUsageFlags::VERTEX_BUFFER
                        | pumice::vk::BufferUsageFlags::TRANSFER_DST,
                    ..Default::default()
                },
                &allocation_info,
            )
            .unwrap();
        assert_eq!(allocation_info.get_mapped_data(), std::ptr::null_mut());
        allocator.destroy_buffer(buffer, allocation);
    }
}

#[test]
fn create_cpu_buffer_preferred() {
    let harness = TestHarness::new();
    let allocator = harness.create_allocator();
    let allocation_info = pumice_vma::AllocationCreateInfo::new()
        .required_flags(pumice::vk::MemoryPropertyFlags::HOST_VISIBLE)
        .preferred_flags(
            pumice::vk::MemoryPropertyFlags::HOST_COHERENT
                | pumice::vk::MemoryPropertyFlags::HOST_CACHED,
        )
        .flags(pumice_vma::AllocationCreateFlags::MAPPED);
    unsafe {
        let (buffer, allocation, allocation_info) = allocator
            .create_buffer(
                &pumice::vk::BufferCreateInfo {
                    size: 16 * 1024,
                    usage: pumice::vk::BufferUsageFlags::VERTEX_BUFFER
                        | pumice::vk::BufferUsageFlags::TRANSFER_DST,
                    ..Default::default()
                },
                &allocation_info,
            )
            .unwrap();
        assert_ne!(allocation_info.get_mapped_data(), std::ptr::null_mut());
        allocator.destroy_buffer(buffer, allocation);
    }
}

#[test]
fn create_gpu_buffer_pool() {
    let harness = TestHarness::new();
    let allocator = harness.create_allocator();

    let buffer_info = pumice::vk::BufferCreateInfo {
        size: 16 * 1024,
        usage: pumice::vk::BufferUsageFlags::UNIFORM_BUFFER
            | pumice::vk::BufferUsageFlags::TRANSFER_DST,
        ..Default::default()
    };

    let mut allocation_info = pumice_vma::AllocationCreateInfo::new()
        .required_flags(pumice::vk::MemoryPropertyFlags::HOST_VISIBLE)
        .preferred_flags(
            pumice::vk::MemoryPropertyFlags::HOST_COHERENT
                | pumice::vk::MemoryPropertyFlags::HOST_CACHED,
        )
        .flags(pumice_vma::AllocationCreateFlags::MAPPED);
    unsafe {
        let memory_type_index = allocator
            .find_memory_type_index_for_buffer_info(&buffer_info, &allocation_info)
            .unwrap();

        // Create a pool that can have at most 2 blocks, 128 MiB each.
        let pool_info = pumice_vma::PoolCreateInfo::new()
            .memory_type_index(memory_type_index)
            .block_size(128 * 1024 * 1024)
            .max_block_count(2);

        let pool = allocator.create_pool(&pool_info).unwrap();
        allocation_info = allocation_info.pool(pool.clone());

        let (buffer, allocation, allocation_info) = allocator
            .create_buffer(&buffer_info, &allocation_info)
            .unwrap();
        assert_ne!(allocation_info.get_mapped_data(), std::ptr::null_mut());
        allocator.destroy_buffer(buffer, allocation);
        allocator.destroy_pool(pool);
    }
}

#[test]
fn test_gpu_stats() {
    let harness = TestHarness::new();
    let allocator = harness.create_allocator();
    let allocation_info =
        pumice_vma::AllocationCreateInfo::new().usage(pumice_vma::MemoryUsage::GpuOnly);

    unsafe {
        let stats_1 = allocator.calculate_stats().unwrap();
        assert_eq!(stats_1.total.blockCount, 0);
        assert_eq!(stats_1.total.allocationCount, 0);
        assert_eq!(stats_1.total.usedBytes, 0);

        let (buffer, allocation, _allocation_info) = allocator
            .create_buffer(
                &pumice::vk::BufferCreateInfo {
                    size: 16 * 1024,
                    usage: pumice::vk::BufferUsageFlags::VERTEX_BUFFER
                        | pumice::vk::BufferUsageFlags::TRANSFER_DST,
                    ..Default::default()
                },
                &allocation_info,
            )
            .unwrap();

        let stats_2 = allocator.calculate_stats().unwrap();
        assert_eq!(stats_2.total.blockCount, 1);
        assert_eq!(stats_2.total.allocationCount, 1);
        assert_eq!(stats_2.total.usedBytes, 16 * 1024);

        allocator.destroy_buffer(buffer, allocation);

        let stats_3 = allocator.calculate_stats().unwrap();
        assert_eq!(stats_3.total.blockCount, 1);
        assert_eq!(stats_3.total.allocationCount, 0);
        assert_eq!(stats_3.total.usedBytes, 0);
    }
}

#[test]
fn test_stats_string() {
    let harness = TestHarness::new();
    let allocator = harness.create_allocator();

    let allocation_info =
        pumice_vma::AllocationCreateInfo::new().usage(pumice_vma::MemoryUsage::GpuOnly);

    unsafe {
        let stats_1 = allocator.build_stats_string(true).unwrap();
        assert!(stats_1.len() > 0);

        let (buffer, allocation, _allocation_info) = allocator
            .create_buffer(
                &pumice::vk::BufferCreateInfo {
                    size: 16 * 1024,
                    usage: pumice::vk::BufferUsageFlags::VERTEX_BUFFER
                        | pumice::vk::BufferUsageFlags::TRANSFER_DST,
                    ..Default::default()
                },
                &allocation_info,
            )
            .unwrap();

        let stats_2 = allocator.build_stats_string(true).unwrap();
        assert!(stats_2.len() > 0);
        assert_ne!(stats_1, stats_2);

        allocator.destroy_buffer(buffer, allocation);

        let stats_3 = allocator.build_stats_string(true).unwrap();
        assert!(stats_3.len() > 0);
        assert_ne!(stats_3, stats_1);
        assert_ne!(stats_3, stats_2);
    }
}
