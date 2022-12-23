use std::{
    mem::{self, ManuallyDrop},
    ops::Deref,
    sync::Arc,
};

use crate::ffi::{self};
use pumice::{vk, DeviceWrapper, InstanceWrapper, VulkanResult};

trait IntoVulkanResult {
    fn into_result<T>(self, val: T) -> VulkanResult<T>;
}

impl IntoVulkanResult for vk::Result {
    fn into_result<T>(self, val: T) -> VulkanResult<T> {
        pumice::new_result(val, self)
    }
}

#[derive(Clone)]
pub struct AllocatorCreateInfo2<'a> {
    pub instance: &'a InstanceWrapper,
    pub device: &'a DeviceWrapper,
    pub physical_device: vk::PhysicalDevice,
    pub flags: ffi::AllocatorCreateFlags,
    pub preferred_large_heap_block_size: u64,
    pub allocation_callbacks: Option<&'a vk::AllocationCallbacks>,
    pub device_memory_callbacks: Option<&'a ffi::DeviceMemoryCallbacks>,
    pub heap_size_limit: Option<&'a [u64]>,
    pub vulkan_api_version: u32,
    pub external_memory_handle_types: Option<&'a [ffi::ExternalMemoryHandleTypeFlags]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefragmentationStatus {
    Success,
    Incomplete,
}

macro_rules! ffi_copy {
    ($e:expr) => {
        ManuallyDrop::into_inner($e.unsafe_copy())
    };
}

#[derive(Clone)]
pub struct AllocatorArc(Arc<ffi::Allocator>);

impl Drop for AllocatorArc {
    fn drop(&mut self) {
        if Arc::strong_count(&self.0) == 1 {
            unsafe {
                ffi::Allocator::destroy(self.0.as_ref().to_owned());
            }
        }
    }
}

impl Deref for AllocatorArc {
    type Target = Arc<ffi::Allocator>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ffi::Allocator {
    pub unsafe fn new_arc(info: &AllocatorCreateInfo2) -> VulkanResult<AllocatorArc> {
        Self::new(info).map(|alloc| AllocatorArc(Arc::new(alloc)))
    }
    pub unsafe fn new(info: &AllocatorCreateInfo2) -> VulkanResult<Self> {
        let instance = &(*info.instance.table());
        let device = &(*info.device.table());

        #[rustfmt::skip]
        let vulkan_functions = ffi::VulkanFunctions {
            vkGetInstanceProcAddr: instance.get_instance_proc_addr,
            vkGetDeviceProcAddr: device.get_device_proc_addr,
            vkGetPhysicalDeviceProperties: instance.get_physical_device_properties,
            vkGetPhysicalDeviceMemoryProperties: instance.get_physical_device_memory_properties,
            vkAllocateMemory: device.allocate_memory,
            vkFreeMemory: device.free_memory,
            vkMapMemory: device.map_memory,
            vkUnmapMemory: device.unmap_memory,
            vkFlushMappedMemoryRanges: device.flush_mapped_memory_ranges,
            vkInvalidateMappedMemoryRanges: device.invalidate_mapped_memory_ranges,
            vkBindBufferMemory: device.bind_buffer_memory,
            vkBindImageMemory: device.bind_image_memory,
            vkGetBufferMemoryRequirements: device.get_buffer_memory_requirements,
            vkGetImageMemoryRequirements: device.get_image_memory_requirements,
            vkCreateBuffer: device.create_buffer,
            vkDestroyBuffer: device.destroy_buffer,
            vkCreateImage: device.create_image,
            vkDestroyImage: device.destroy_image,
            vkCmdCopyBuffer: device.cmd_copy_buffer,
            #[cfg(feature = "VK_VERSION_1_1")]
            vkGetBufferMemoryRequirements2KHR: device.get_buffer_memory_requirements_2,
            #[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_dedicated_allocation"))]
            vkGetBufferMemoryRequirements2KHR: device.get_buffer_memory_requirements_2_khr,
            #[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_dedicated_allocation")))]
            vkGetBufferMemoryRequirements2KHR: None,
            #[cfg(feature = "VK_VERSION_1_1")]
            vkGetImageMemoryRequirements2KHR: device.get_image_memory_requirements_2,
            #[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_dedicated_allocation"))]
            vkGetImageMemoryRequirements2KHR: device.get_image_memory_requirements_2_khr,
            #[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_dedicated_allocation")))]
            vkGetImageMemoryRequirements2KHR: None,
            #[cfg(feature = "VK_VERSION_1_1")]
            vkBindBufferMemory2KHR: device.bind_buffer_memory_2,
            #[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_bind_memory2"))]
            vkBindBufferMemory2KHR: device.bind_buffer_memory_2_khr,
            #[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_bind_memory2")))]
            vkBindBufferMemory2KHR: None,
            #[cfg(feature = "VK_VERSION_1_1")]
            vkBindImageMemory2KHR: device.bind_image_memory_2,
            #[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_bind_memory2"))]
            vkBindImageMemory2KHR: device.bind_image_memory_2_khr,
            #[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_bind_memory2")))]
            vkBindImageMemory2KHR: None,
            #[cfg(feature = "VK_VERSION_1_1")]
            vkGetPhysicalDeviceMemoryProperties2KHR: instance.get_physical_device_memory_properties_2,
            #[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_get_physical_device_properties2"))]
            vkGetPhysicalDeviceMemoryProperties2KHR: instance.get_physical_device_memory_properties_2_khr,
            #[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_get_physical_device_properties2")))]
            vkGetPhysicalDeviceMemoryProperties2KHR: None,
            #[cfg(feature = "VK_VERSION_1_3")]
            vkGetDeviceBufferMemoryRequirements: device.get_device_buffer_memory_requirements,
            #[cfg(all(not(feature = "VK_VERSION_1_3"), all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4")))]
            vkGetDeviceBufferMemoryRequirements: device.get_device_buffer_memory_requirements_khr,
            #[cfg(all(not(feature = "VK_VERSION_1_3"), not(all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4"))))]
            vkGetDeviceBufferMemoryRequirements: None,
            #[cfg(feature = "VK_VERSION_1_3")]
            vkGetDeviceImageMemoryRequirements: device.get_device_image_memory_requirements,
            #[cfg(all(not(feature = "VK_VERSION_1_3"), all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4")))]
            vkGetDeviceImageMemoryRequirements: device.get_device_image_memory_requirements_khr,
            #[cfg(all(not(feature = "VK_VERSION_1_3"), not(all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4"))))]
            vkGetDeviceImageMemoryRequirements: None,
        };

        macro_rules! opt2ptr {
            ($ref:expr) => {
                match $ref {
                    Some(t) => t as *const _,
                    None => ::std::ptr::null(),
                }
            };
        }

        let memory_properties = info
            .instance
            .get_physical_device_memory_properties(info.physical_device);

        let create_info = ffi::AllocatorCreateInfo {
            flags: info.flags,
            physical_device: info.physical_device,
            device: info.device.handle(),
            preferred_large_heap_block_size: info.preferred_large_heap_block_size,
            allocation_callbacks: opt2ptr!(info.allocation_callbacks),
            device_memory_callbacks: opt2ptr!(info.device_memory_callbacks),
            heap_size_limit: match info.heap_size_limit {
                Some(limit) => {
                    assert_eq!(limit.len() as u32, memory_properties.memory_heap_count,);
                    limit.as_ptr()
                }
                None => std::ptr::null(),
            },
            vulkan_functions: &vulkan_functions,
            instance: info.instance.handle(),
            vulkan_api_version: info.vulkan_api_version,
            type_external_memory_handle_types: match info.external_memory_handle_types {
                Some(types) => {
                    assert_eq!(types.len() as u32, memory_properties.memory_type_count);
                    types.as_ptr()
                }
                None => std::ptr::null(),
            },
        };

        let mut out: ffi::Allocator = mem::zeroed();
        ffi::vmaCreateAllocator(&create_info, &mut out).into_result(out)
    }
    pub unsafe fn get_allocator_info(&self) -> ffi::AllocatorInfo {
        let mut out: ffi::AllocatorInfo = mem::zeroed();
        ffi::vmaGetAllocatorInfo(self.clone(), &mut out);
        out
    }
    pub unsafe fn get_physical_device_properties(&self) -> &vk::PhysicalDeviceProperties {
        let mut out: *const vk::PhysicalDeviceProperties = mem::zeroed();
        ffi::vmaGetPhysicalDeviceProperties(self.clone(), &mut out);
        &*out
    }
    pub unsafe fn get_memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        let mut out: *const vk::PhysicalDeviceMemoryProperties = mem::zeroed();
        ffi::vmaGetMemoryProperties(self.clone(), &mut out);
        &*out
    }
    pub unsafe fn get_memory_type_properties(
        &self,
        memory_type_index: u32,
    ) -> vk::MemoryPropertyFlags {
        let mut out: vk::MemoryPropertyFlags = mem::zeroed();
        ffi::vmaGetMemoryTypeProperties(self.clone(), memory_type_index, &mut out);
        out
    }
    pub unsafe fn set_current_frame_index(&self, frame_index: u32) {
        ffi::vmaSetCurrentFrameIndex(self.clone(), frame_index);
    }
    pub unsafe fn calculate_statistics(&self) -> ffi::TotalStatistics {
        let mut out: ffi::TotalStatistics = mem::zeroed();
        ffi::vmaCalculateStatistics(self.clone(), &mut out);
        out
    }
    pub unsafe fn get_heap_budgets(&self) -> Vec<ffi::Budget> {
        let properties = self.get_memory_properties();
        let mut out: Vec<ffi::Budget> = vec![mem::zeroed(); properties.memory_heap_count as usize];
        ffi::vmaGetHeapBudgets(self.clone(), out.as_mut_ptr());
        out
    }
    pub unsafe fn find_memory_type_index(
        &self,
        memory_type_bits: u32,
        allocation_create_info: &ffi::AllocationCreateInfo,
    ) -> VulkanResult<u32> {
        let mut out: u32 = mem::zeroed();
        ffi::vmaFindMemoryTypeIndex(
            self.clone(),
            memory_type_bits,
            allocation_create_info,
            &mut out,
        )
        .into_result(out)
    }
    pub unsafe fn find_memory_type_index_for_buffer_info(
        &self,
        buffer_create_info: &vk::BufferCreateInfo,
        allocation_create_info: &ffi::AllocationCreateInfo,
    ) -> VulkanResult<u32> {
        let mut out: u32 = mem::zeroed();
        ffi::vmaFindMemoryTypeIndexForBufferInfo(
            self.clone(),
            buffer_create_info,
            allocation_create_info,
            &mut out,
        )
        .into_result(out)
    }
    pub unsafe fn find_memory_type_index_for_image_info(
        &self,
        image_create_info: &vk::ImageCreateInfo,
        allocation_create_info: &ffi::AllocationCreateInfo,
    ) -> VulkanResult<u32> {
        let mut out: u32 = mem::zeroed();
        ffi::vmaFindMemoryTypeIndexForImageInfo(
            self.clone(),
            image_create_info,
            allocation_create_info,
            &mut out,
        )
        .into_result(out)
    }
    pub unsafe fn create_pool(&self, create_info: &ffi::PoolCreateInfo) -> VulkanResult<ffi::Pool> {
        let mut out: ffi::Pool = mem::zeroed();
        ffi::vmaCreatePool(self.clone(), create_info, &mut out).into_result(out)
    }
    pub unsafe fn destroy_pool(&self, pool: ffi::Pool) {
        ffi::vmaDestroyPool(self.clone(), pool);
    }
    pub unsafe fn get_pool_statistics(&self, pool: ffi::Pool) -> ffi::Statistics {
        let mut out: ffi::Statistics = mem::zeroed();
        ffi::vmaGetPoolStatistics(self.clone(), pool, &mut out);
        out
    }
    pub unsafe fn calculate_pool_statistics(&self, pool: ffi::Pool) -> ffi::DetailedStatistics {
        let mut out: ffi::DetailedStatistics = mem::zeroed();
        ffi::vmaCalculatePoolStatistics(self.clone(), pool, &mut out);
        out
    }
    pub unsafe fn check_pool_corruption(&self, pool: ffi::Pool) -> VulkanResult<()> {
        ffi::vmaCheckPoolCorruption(self.clone(), pool).into_result(())
    }
    pub unsafe fn get_pool_name(&self, pool: ffi::Pool) -> Option<&'static std::ffi::CStr> {
        let mut out: *const std::ffi::c_char = mem::zeroed();
        ffi::vmaGetPoolName(self.clone(), pool, &mut out);
        if out.is_null() {
            None
        } else {
            Some(std::ffi::CStr::from_ptr(out))
        }
    }
    pub unsafe fn set_pool_name(&self, pool: ffi::Pool, name: Option<&'static std::ffi::CStr>) {
        let ptr = match name {
            Some(name) => name.as_ptr(),
            None => std::ptr::null(),
        };
        ffi::vmaSetPoolName(self.clone(), pool, ptr);
    }
    pub unsafe fn allocate_memory(
        &self,
        memory_requirements: &vk::MemoryRequirements,
        create_info: &ffi::AllocationCreateInfo,
    ) -> VulkanResult<(ffi::Allocation, ffi::AllocationInfo)> {
        let mut out1: ffi::Allocation = mem::zeroed();
        let mut out2: ffi::AllocationInfo = mem::zeroed();
        ffi::vmaAllocateMemory(
            self.clone(),
            memory_requirements,
            create_info,
            &mut out1,
            &mut out2,
        )
        .into_result((out1, out2))
    }
    pub unsafe fn allocate_memory_pages(
        &self,
        memory_requirements: &vk::MemoryRequirements,
        create_info: &ffi::AllocationCreateInfo,
        allocation_count: usize,
    ) -> VulkanResult<(Vec<ffi::Allocation>, Vec<ffi::AllocationInfo>)> {
        let mut allocations: Vec<ffi::Allocation> = vec![mem::zeroed(); allocation_count];
        let mut allocation_info: Vec<ffi::AllocationInfo> = vec![mem::zeroed(); allocation_count];
        ffi::vmaAllocateMemoryPages(
            self.clone(),
            memory_requirements,
            create_info,
            allocation_count,
            allocations.as_mut_ptr(),
            allocation_info.as_mut_ptr(),
        )
        .into_result((allocations, allocation_info))
    }
    pub unsafe fn allocate_memory_pages_into(
        &self,
        memory_requirements: &vk::MemoryRequirements,
        create_info: &ffi::AllocationCreateInfo,
        allocations: &mut [ffi::Allocation],
        allocation_info: &mut [ffi::AllocationInfo],
    ) -> VulkanResult<()> {
        assert!(allocations.len() == allocation_info.len());

        ffi::vmaAllocateMemoryPages(
            self.clone(),
            memory_requirements,
            create_info,
            allocations.len(),
            allocations.as_mut_ptr(),
            allocation_info.as_mut_ptr(),
        )
        .into_result(())
    }
    pub unsafe fn allocate_memory_for_buffer(
        &self,
        buffer: vk::Buffer,
        create_info: &ffi::AllocationCreateInfo,
    ) -> VulkanResult<(ffi::Allocation, ffi::AllocationInfo)> {
        let mut allocation: ffi::Allocation = mem::zeroed();
        let mut allocation_info: ffi::AllocationInfo = mem::zeroed();
        ffi::vmaAllocateMemoryForBuffer(
            self.clone(),
            buffer,
            create_info,
            &mut allocation,
            &mut allocation_info,
        )
        .into_result((allocation, allocation_info))
    }
    pub unsafe fn allocate_memory_for_image(
        &self,
        image: vk::Image,
        create_info: &ffi::AllocationCreateInfo,
    ) -> VulkanResult<(ffi::Allocation, ffi::AllocationInfo)> {
        let mut allocation: ffi::Allocation = mem::zeroed();
        let mut allocation_info: ffi::AllocationInfo = mem::zeroed();
        ffi::vmaAllocateMemoryForImage(
            self.clone(),
            image,
            create_info,
            &mut allocation,
            &mut allocation_info,
        )
        .into_result((allocation, allocation_info))
    }
    pub unsafe fn free_memory(&self, allocation: ffi::Allocation) {
        ffi::vmaFreeMemory(self.clone(), allocation);
    }
    pub unsafe fn free_memory_pages(&self, allocations: &[ffi::Allocation]) {
        ffi::vmaFreeMemoryPages(self.clone(), allocations.len(), allocations.as_ptr());
    }
    pub unsafe fn get_allocation_info(&self, allocation: ffi::Allocation) -> ffi::AllocationInfo {
        let mut out: ffi::AllocationInfo = mem::zeroed();
        ffi::vmaGetAllocationInfo(self.clone(), allocation, &mut out);
        out
    }
    pub unsafe fn set_allocation_user_data(
        &self,
        allocation: ffi::Allocation,
        user_data: *mut std::ffi::c_void,
    ) {
        ffi::vmaSetAllocationUserData(self.clone(), allocation, user_data);
    }
    pub unsafe fn set_allocation_name(
        &self,
        allocation: ffi::Allocation,
        name: Option<&'static std::ffi::CStr>,
    ) {
        let ptr = match name {
            Some(name) => name.as_ptr(),
            None => std::ptr::null(),
        };
        ffi::vmaSetAllocationName(self.clone(), allocation, ptr);
    }
    pub unsafe fn get_allocation_memory_properties(
        &self,
        allocation: ffi::Allocation,
    ) -> vk::MemoryPropertyFlags {
        let mut out: vk::MemoryPropertyFlags = mem::zeroed();
        ffi::vmaGetAllocationMemoryProperties(self.clone(), allocation, &mut out);
        out
    }
    pub unsafe fn map_memory(&self, allocation: ffi::Allocation) -> VulkanResult<*mut u8> {
        let mut out: *mut std::ffi::c_void = mem::zeroed();
        ffi::vmaMapMemory(self.clone(), allocation, &mut out).into_result(out as *mut u8)
    }
    pub unsafe fn unmap_memory(&self, allocation: ffi::Allocation) {
        ffi::vmaUnmapMemory(self.clone(), allocation);
    }
    pub unsafe fn flush_allocation(
        &self,
        allocation: ffi::Allocation,
        offset: u64,
        size: u64,
    ) -> VulkanResult<()> {
        ffi::vmaFlushAllocation(self.clone(), allocation, offset, size).into_result(())
    }
    pub unsafe fn invalidate_allocation(
        &self,
        allocation: ffi::Allocation,
        offset: u64,
        size: u64,
    ) -> VulkanResult<()> {
        ffi::vmaInvalidateAllocation(self.clone(), allocation, offset, size).into_result(())
    }
    pub unsafe fn flush_allocations(
        &self,
        allocations: &[ffi::Allocation],
        offsets: &[u64],
        sizes: &[u64],
    ) -> VulkanResult<()> {
        assert!(allocations.len() == offsets.len() && offsets.len() == sizes.len());
        ffi::vmaFlushAllocations(
            self.clone(),
            allocations.len() as u32,
            allocations.as_ptr(),
            offsets.as_ptr(),
            sizes.as_ptr(),
        )
        .into_result(())
    }
    pub unsafe fn invalidate_allocations(
        &self,
        allocations: &[ffi::Allocation],
        offsets: &[u64],
        sizes: &[u64],
    ) -> VulkanResult<()> {
        assert!(allocations.len() == offsets.len() && offsets.len() == sizes.len());
        ffi::vmaInvalidateAllocations(
            self.clone(),
            allocations.len() as u32,
            allocations.as_ptr(),
            offsets.as_ptr(),
            sizes.as_ptr(),
        )
        .into_result(())
    }
    pub unsafe fn check_corruption(&self, memory_type_bits: u32) -> VulkanResult<()> {
        ffi::vmaCheckCorruption(self.clone(), memory_type_bits).into_result(())
    }
    pub unsafe fn begin_defragmentation(
        &self,
        info: &ffi::DefragmentationInfo,
    ) -> VulkanResult<ffi::DefragmentationContext> {
        let mut context: ffi::DefragmentationContext = mem::zeroed();
        ffi::vmaBeginDefragmentation(self.clone(), info, &mut context).into_result(context)
    }
    pub unsafe fn end_defragmentation(
        &self,
        context: ffi::DefragmentationContext,
    ) -> ffi::DefragmentationStats {
        let mut stats: ffi::DefragmentationStats = mem::zeroed();
        ffi::vmaEndDefragmentation(self.clone(), context, &mut stats);
        stats
    }
    pub unsafe fn begin_defragmentation_pass(
        &self,
        context: ffi::DefragmentationContext,
    ) -> (DefragmentationStatus, ffi::DefragmentationPassMoveInfo) {
        let mut pass_info: ffi::DefragmentationPassMoveInfo = mem::zeroed();
        let result = ffi::vmaBeginDefragmentationPass(self.clone(), context, &mut pass_info);

        let status = match result {
            vk::Result::SUCCESS => DefragmentationStatus::Success,
            vk::Result::INCOMPLETE => DefragmentationStatus::Incomplete,
            _ => unreachable!(),
        };
        (status, pass_info)
    }
    pub unsafe fn end_defragmentation_pass(
        &self,
        context: ffi::DefragmentationContext,
        pass_info: &mut ffi::DefragmentationPassMoveInfo,
    ) -> DefragmentationStatus {
        let result = ffi::vmaEndDefragmentationPass(self.clone(), context, pass_info);
        match result {
            vk::Result::SUCCESS => DefragmentationStatus::Success,
            vk::Result::INCOMPLETE => DefragmentationStatus::Incomplete,
            _ => unreachable!(),
        }
    }
    pub unsafe fn bind_buffer_memory(
        &self,
        allocation: ffi::Allocation,
        buffer: vk::Buffer,
    ) -> VulkanResult<()> {
        ffi::vmaBindBufferMemory(self.clone(), allocation, buffer).into_result(())
    }
    pub unsafe fn bind_buffer_memory2(
        &self,
        allocation: ffi::Allocation,
        allocation_local_offset: u64,
        buffer: vk::Buffer,
        pnext: *const std::ffi::c_void,
    ) -> VulkanResult<()> {
        ffi::vmaBindBufferMemory2(
            self.clone(),
            allocation,
            allocation_local_offset,
            buffer,
            pnext,
        )
        .into_result(())
    }
    pub unsafe fn bind_image_memory(
        &self,
        allocation: ffi::Allocation,
        image: vk::Image,
    ) -> VulkanResult<()> {
        ffi::vmaBindImageMemory(self.clone(), allocation, image).into_result(())
    }
    pub unsafe fn bind_image_memory2(
        &self,
        allocation: ffi::Allocation,
        allocation_local_offset: u64,
        image: vk::Image,
        pnext: *const std::ffi::c_void,
    ) -> VulkanResult<()> {
        ffi::vmaBindImageMemory2(
            self.clone(),
            allocation,
            allocation_local_offset,
            image,
            pnext,
        )
        .into_result(())
    }
    pub unsafe fn create_buffer(
        &self,
        bufer_create_info: &vk::BufferCreateInfo,
        allocation_create_info: &ffi::AllocationCreateInfo,
    ) -> VulkanResult<(vk::Buffer, ffi::Allocation, ffi::AllocationInfo)> {
        let mut buffer: vk::Buffer = mem::zeroed();
        let mut allocation: ffi::Allocation = mem::zeroed();
        let mut allocation_info: ffi::AllocationInfo = mem::zeroed();
        ffi::vmaCreateBuffer(
            self.clone(),
            bufer_create_info,
            allocation_create_info,
            &mut buffer,
            &mut allocation,
            &mut allocation_info,
        )
        .into_result((buffer, allocation, allocation_info))
    }
    pub unsafe fn create_buffer_with_alignment(
        &self,
        bufer_create_info: &vk::BufferCreateInfo,
        allocation_create_info: &ffi::AllocationCreateInfo,
        min_alignment: u64,
    ) -> VulkanResult<(vk::Buffer, ffi::Allocation, ffi::AllocationInfo)> {
        let mut buffer: vk::Buffer = mem::zeroed();
        let mut allocation: ffi::Allocation = mem::zeroed();
        let mut allocation_info: ffi::AllocationInfo = mem::zeroed();
        ffi::vmaCreateBufferWithAlignment(
            self.clone(),
            bufer_create_info,
            allocation_create_info,
            min_alignment,
            &mut buffer,
            &mut allocation,
            &mut allocation_info,
        )
        .into_result((buffer, allocation, allocation_info))
    }
    pub unsafe fn create_aliasing_buffer(
        &self,
        allocation: ffi::Allocation,
        buffer_create_info: &vk::BufferCreateInfo,
    ) -> VulkanResult<vk::Buffer> {
        let mut buffer: vk::Buffer = mem::zeroed();
        ffi::vmaCreateAliasingBuffer(self.clone(), allocation, buffer_create_info, &mut buffer)
            .into_result(buffer)
    }
    pub unsafe fn create_aliasing_buffer2(
        &self,
        allocation: ffi::Allocation,
        allocation_local_offset: u64,
        buffer_create_info: &vk::BufferCreateInfo,
    ) -> VulkanResult<vk::Buffer> {
        let mut buffer: vk::Buffer = mem::zeroed();
        ffi::vmaCreateAliasingBuffer2(
            self.clone(),
            allocation,
            allocation_local_offset,
            buffer_create_info,
            &mut buffer,
        )
        .into_result(buffer)
    }
    pub unsafe fn destroy_buffer(&self, buffer: vk::Buffer, allocation: ffi::Allocation) {
        ffi::vmaDestroyBuffer(self.clone(), buffer, allocation);
    }
    pub unsafe fn create_image(
        &self,
        bufer_create_info: &vk::ImageCreateInfo,
        allocation_create_info: &ffi::AllocationCreateInfo,
    ) -> VulkanResult<(vk::Image, ffi::Allocation, ffi::AllocationInfo)> {
        let mut image: vk::Image = mem::zeroed();
        let mut allocation: ffi::Allocation = mem::zeroed();
        let mut allocation_info: ffi::AllocationInfo = mem::zeroed();
        ffi::vmaCreateImage(
            self.clone(),
            bufer_create_info,
            allocation_create_info,
            &mut image,
            &mut allocation,
            &mut allocation_info,
        )
        .into_result((image, allocation, allocation_info))
    }
    pub unsafe fn create_aliasing_image(
        &self,
        allocation: ffi::Allocation,
        image_create_info: &vk::ImageCreateInfo,
    ) -> VulkanResult<vk::Image> {
        let mut image: vk::Image = mem::zeroed();
        ffi::vmaCreateAliasingImage(self.clone(), allocation, image_create_info, &mut image)
            .into_result(image)
    }
    pub unsafe fn create_aliasing_image2(
        &self,
        allocation: ffi::Allocation,
        allocation_local_offset: u64,
        image_create_info: &vk::ImageCreateInfo,
    ) -> VulkanResult<vk::Image> {
        let mut image: vk::Image = mem::zeroed();
        ffi::vmaCreateAliasingImage2(
            self.clone(),
            allocation,
            allocation_local_offset,
            image_create_info,
            &mut image,
        )
        .into_result(image)
    }
    pub unsafe fn destroy_image(&self, image: vk::Image, allocation: ffi::Allocation) {
        ffi::vmaDestroyImage(self.clone(), image, allocation);
    }
    pub fn build_stats_string(&self, detailed_map: bool) -> String {
        unsafe {
            let mut out: *mut std::ffi::c_char = mem::zeroed();
            ffi::vmaBuildStatsString(self.clone(), &mut out, detailed_map as vk::Bool32);
            let string = std::ffi::CStr::from_ptr(out).to_str().unwrap().to_owned();
            ffi::vmaFreeStatsString(self.clone(), out);
            string
        }
    }
    pub unsafe fn destroy(this: Self) {
        ffi::vmaDestroyAllocator(this);
    }
}

impl ffi::VirtualBlock {
    pub fn new(info: &ffi::VirtualBlockCreateInfo) -> Self {
        unsafe {
            let mut out: ffi::VirtualBlock = mem::zeroed();
            ffi::vmaCreateVirtualBlock(info, &mut out);
            out
        }
    }
    pub fn is_empty(&self) -> bool {
        unsafe { ffi::vmaIsVirtualBlockEmpty(ffi_copy!(self.clone())) == vk::TRUE }
    }
    pub fn get_allocation_info(
        &self,
        allocation: ffi::VirtualAllocation,
    ) -> ffi::VirtualAllocationInfo {
        unsafe {
            let mut out: ffi::VirtualAllocationInfo = mem::zeroed();
            ffi::vmaGetVirtualAllocationInfo(ffi_copy!(self.clone()), allocation, &mut out);
            out
        }
    }
    pub fn allocate(
        &self,
        create_info: ffi::VirtualAllocationCreateInfo,
    ) -> VulkanResult<(ffi::VirtualAllocation, u64)> {
        unsafe {
            let mut out1: ffi::VirtualAllocation = mem::zeroed();
            let mut out2: u64 = mem::zeroed();
            ffi::vmaVirtualAllocate(ffi_copy!(self.clone()), &create_info, &mut out1, &mut out2)
                .into_result((out1, out2))
        }
    }
    pub fn free(&self, allocation: ffi::VirtualAllocation) {
        unsafe {
            ffi::vmaVirtualFree(ffi_copy!(self.clone()), allocation);
        }
    }
    pub fn clear(&self) {
        unsafe {
            ffi::vmaClearVirtualBlock(ffi_copy!(self.clone()));
        }
    }
    pub fn set_allocation_user_data(
        &self,
        allocation: ffi::VirtualAllocation,
        user_data: *mut std::ffi::c_void,
    ) {
        unsafe {
            ffi::vmaSetVirtualAllocationUserData(ffi_copy!(self.clone()), allocation, user_data);
        }
    }
    pub fn get_statistics(&self) -> ffi::Statistics {
        unsafe {
            let mut out: ffi::Statistics = mem::zeroed();
            ffi::vmaGetVirtualBlockStatistics(ffi_copy!(self.clone()), &mut out);
            out
        }
    }
    pub fn calculate_statistics(&self) -> ffi::DetailedStatistics {
        unsafe {
            let mut out: ffi::DetailedStatistics = mem::zeroed();
            ffi::vmaCalculateVirtualBlockStatistics(ffi_copy!(self.clone()), &mut out);
            out
        }
    }
    pub fn build_stats_string(&self, detailed_map: bool) -> String {
        unsafe {
            let mut out: *mut std::ffi::c_char = mem::zeroed();
            ffi::vmaBuildVirtualBlockStatsString(
                ffi_copy!(self.clone()),
                &mut out,
                detailed_map as vk::Bool32,
            );
            let string = std::ffi::CStr::from_ptr(out).to_str().unwrap().to_owned();
            ffi::vmaFreeVirtualBlockStatsString(ffi_copy!(self.clone()), out);
            string
        }
    }
}

impl Drop for ffi::VirtualBlock {
    fn drop(&mut self) {
        unsafe {
            ffi::vmaDestroyVirtualBlock(ffi_copy!(self));
        }
    }
}
