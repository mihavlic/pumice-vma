use pumice::{
    loader::{
        tables::{DeviceTable, EntryTable, InstanceTable},
        InstanceLoader,
    },
    vk, DeviceWrapper,
};
use pumice_vma as vma;

#[cfg(feature = "VK_EXT_debug_report")]
unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const std::os::raw::c_char,
    p_message: *const std::os::raw::c_char,
    _: *mut std::ffi::c_void,
) -> u32 {
    println!("{:?}", ::std::ffi::CStr::from_ptr(p_message));
    vk::FALSE
}

pub struct TestHarness {
    // these tables store the function pointers to all the vulkan commands
    pub tables: Box<(EntryTable, InstanceTable, DeviceTable)>,
    pub entry: pumice::EntryWrapper,
    pub instance: pumice::InstanceWrapper,
    pub device: pumice::DeviceWrapper,
    pub physical_device: vk::PhysicalDevice,
    #[cfg(feature = "VK_EXT_debug_report")]
    pub debug_callback: vk::DebugReportCallbackEXT,
    // if vulkan is loaded with libloading, the lifetime of the loaded library's memory is tied to this object so it must be preserved until the end
    pub loader: pumice::loader::EntryLoader,
}

impl Drop for TestHarness {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_device(None);
            #[cfg(feature = "VK_EXT_debug_report")]
            self.instance
                .destroy_debug_report_callback_ext(self.debug_callback, None);
            self.instance.destroy_instance(None);
        }
    }
}
impl TestHarness {
    pub fn new() -> Self {
        let app_name = ::std::ffi::CString::new("vk-mem testing").unwrap();
        let api_version = vk::make_api_version(0, 1, 0, 0);

        #[allow(unused_mut)]
        let mut config = pumice::util::ApiLoadConfig::new(api_version);
        #[cfg(feature = "VK_EXT_debug_report")]
        config.add_extension(pumice::extensions::ext_debug_report::EXT_DEBUG_REPORT_EXTENSION_NAME);

        let app_info = vk::ApplicationInfo {
            p_application_name: app_name.as_ptr(),
            application_version: 0,
            p_engine_name: app_name.as_ptr(),
            engine_version: 0,
            api_version,
            ..Default::default()
        };

        let layer_names = [pumice::cstr!("VK_LAYER_KHRONOS_validation").as_ptr()];

        let extension_names_raw = config.get_instance_extensions();
        let create_info = vk::InstanceCreateInfo {
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

        #[cfg(feature = "VK_EXT_debug_report")]
        let debug_callback = unsafe {
            let debug_info = vk::DebugReportCallbackCreateInfoEXT {
                flags: vk::DebugReportFlagsEXT::ERROR
                    | vk::DebugReportFlagsEXT::WARNING
                    | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING,
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

        let queue_info = [vk::DeviceQueueCreateInfo {
            queue_family_index: queue_family_index as _,
            p_queue_priorities: priorities.as_ptr(),
            queue_count: priorities.len() as _,
            ..Default::default()
        }];

        let device_create_info = vk::DeviceCreateInfo {
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
            #[cfg(feature = "VK_EXT_debug_report")]
            debug_callback,
        }
    }

    pub fn create_allocator(&self) -> vma::AllocatorArc {
        unsafe {
            let create_info = vma::AllocatorCreateInfo2 {
                instance: &self.instance,
                device: &self.device,
                physical_device: self.physical_device,
                flags: vma::AllocatorCreateFlags::empty(),
                preferred_large_heap_block_size: 1024 * 1024 * 32, // 32 MiB
                allocation_callbacks: None,
                device_memory_callbacks: None,
                heap_size_limit: None,
                vulkan_api_version: vk::API_VERSION_1_0,
                external_memory_handle_types: None,
            };

            vma::Allocator::new_arc(&create_info).unwrap()
        }
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

    unsafe {
        let (buffer, allocation, _allocation_info) = allocator
            .create_buffer(
                &vk::BufferCreateInfo {
                    size: 16 * 1024,
                    usage: vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
                    ..Default::default()
                },
                &vma::AllocationCreateInfo {
                    usage: vma::MemoryUsage::AutoPreferDevice,
                    ..Default::default()
                },
            )
            .unwrap();

        allocator.destroy_buffer(buffer, allocation);
    }
}

#[test]
fn create_cpu_buffer_preferred() {
    let harness = TestHarness::new();
    let allocator = harness.create_allocator();

    unsafe {
        let (buffer, allocation, allocation_info) = allocator
            .create_buffer(
                &vk::BufferCreateInfo {
                    size: 16 * 1024,
                    usage: vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
                    ..Default::default()
                },
                &vma::AllocationCreateInfo {
                    required_flags: vk::MemoryPropertyFlags::HOST_VISIBLE,
                    preferred_flags: vk::MemoryPropertyFlags::HOST_COHERENT
                        | vk::MemoryPropertyFlags::HOST_CACHED,
                    flags: vma::AllocationCreateFlags::MAPPED,
                    ..Default::default()
                },
            )
            .unwrap();
        assert_ne!(allocation_info.mapped_data, std::ptr::null_mut());
        allocator.destroy_buffer(buffer, allocation);
    }
}

#[test]
fn create_gpu_buffer_pool() {
    let harness = TestHarness::new();
    let allocator = harness.create_allocator();

    let buffer_info = vk::BufferCreateInfo {
        size: 16 * 1024,
        usage: vk::BufferUsageFlags::UNIFORM_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
        ..Default::default()
    };

    let mut allocation_info = vma::AllocationCreateInfo {
        required_flags: vk::MemoryPropertyFlags::HOST_VISIBLE,
        preferred_flags: vk::MemoryPropertyFlags::HOST_COHERENT
            | vk::MemoryPropertyFlags::HOST_CACHED,
        flags: vma::AllocationCreateFlags::MAPPED,
        ..Default::default()
    };

    unsafe {
        let memory_type_index = allocator
            .find_memory_type_index_for_buffer_info(&buffer_info, &allocation_info)
            .unwrap();

        // Create a pool that can have at most 2 blocks, 128 MiB each.
        let pool_info = vma::PoolCreateInfo {
            memory_type_index: memory_type_index,
            block_size: 128 * 1024 * 1024,
            max_block_count: 2,
            ..Default::default()
        };

        let pool = allocator.create_pool(&pool_info).unwrap();
        allocation_info.pool = pool;

        let (buffer, allocation, allocation_info) = allocator
            .create_buffer(&buffer_info, &allocation_info)
            .unwrap();

        assert_ne!(allocation_info.mapped_data, std::ptr::null_mut());
        allocator.destroy_buffer(buffer, allocation);
        allocator.destroy_pool(pool);
    }
}

#[test]
fn test_gpu_budget() {
    let harness = TestHarness::new();
    let allocator = harness.create_allocator();

    fn sum_budgets(budgets: Vec<vma::Budget>) -> vma::Budget {
        // Budget is plain old data, this is ok
        let mut budget: vma::Budget = unsafe { std::mem::zeroed() };
        for b in budgets {
            budget.statistics.block_count += b.statistics.block_count;
            budget.statistics.allocation_count += b.statistics.allocation_count;
            budget.statistics.block_bytes += b.statistics.block_bytes;
            budget.statistics.allocation_bytes += b.statistics.allocation_bytes;
            budget.usage += b.usage;
            budget.budget += b.budget;
        }
        budget
    }

    unsafe {
        let budget = sum_budgets(allocator.get_heap_budgets());
        assert_eq!(budget.statistics.allocation_bytes, 0);
        assert_eq!(budget.statistics.allocation_count, 0);
        assert_eq!(budget.statistics.block_bytes, 0);
        assert_eq!(budget.statistics.block_count, 0);

        let (buffer, allocation, _allocation_info) = allocator
            .create_buffer(
                &vk::BufferCreateInfo {
                    size: 16 * 1024,
                    usage: vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
                    ..Default::default()
                },
                &vma::AllocationCreateInfo {
                    usage: vma::MemoryUsage::AutoPreferDevice,
                    ..Default::default()
                },
            )
            .unwrap();

        let budget = sum_budgets(allocator.get_heap_budgets());
        assert_eq!(budget.statistics.allocation_bytes, 16 * 1024);
        assert_eq!(budget.statistics.allocation_count, 1);

        allocator.destroy_buffer(buffer, allocation);

        let budget = sum_budgets(allocator.get_heap_budgets());
        assert_eq!(budget.statistics.block_count, 1);
    }
}

#[test]
fn test_stats_string() {
    let harness = TestHarness::new();
    let allocator = harness.create_allocator();

    unsafe {
        let stats_1 = allocator.build_stats_string(true);
        assert!(!stats_1.is_empty());

        let (buffer, allocation, _allocation_info) = allocator
            .create_buffer(
                &vk::BufferCreateInfo {
                    size: 16 * 1024,
                    usage: vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
                    ..Default::default()
                },
                &vma::AllocationCreateInfo {
                    usage: vma::MemoryUsage::AutoPreferDevice,
                    ..Default::default()
                },
            )
            .unwrap();

        let stats_2 = allocator.build_stats_string(true);
        assert!(stats_2.len() > 0);
        assert_ne!(stats_1, stats_2);

        allocator.destroy_buffer(buffer, allocation);

        let stats_3 = allocator.build_stats_string(true);
        assert!(stats_3.len() > 0);
        assert_ne!(stats_3, stats_1);
        assert_ne!(stats_3, stats_2);
    }
}
