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

/// Description of a Allocator to be created.
#[derive(Clone)]
pub struct AllocatorCreateInfo2<'a> {
    /// Handle to Vulkan instance object.
    ///
    /// Starting from version 3.0.0 this member is no longer optional, it must be set!
    pub instance: &'a InstanceWrapper,
    /// Vulkan device.
    /// It must be valid throughout whole lifetime of created allocator.
    pub device: &'a DeviceWrapper,
    /// Vulkan physical device.
    /// It must be valid throughout whole lifetime of created allocator.
    pub physical_device: vk::PhysicalDevice,
    /// Flags for created allocator. Use `VmaAllocatorCreateFlagBits` enum.
    pub flags: ffi::AllocatorCreateFlags,
    /// Preferred size of a single `VkDeviceMemory` block to be allocated from large heaps > 1 GiB. Optional.
    /// Set to 0 to use default, which is currently 256 MiB.
    pub preferred_large_heap_block_size: u64,
    /// Custom `CPU` memory allocation callbacks. Optional.
    /// Optional, can be null. When specified, will also be used for all CPU-side memory allocations.
    pub allocation_callbacks: Option<&'a vk::AllocationCallbacks>,
    /// Informative callbacks for `vkAllocateMemory`, `vkFreeMemory`. Optional.
    /// Optional, can be null.
    pub device_memory_callbacks: Option<&'a ffi::DeviceMemoryCallbacks>,
    /// Either null or a pointer to an array of limits on maximum number of bytes that can be allocated out of particular Vulkan memory heap.
    ///
    /// If not NULL, it must be a pointer to an array of
    /// `VkPhysicalDeviceMemoryProperties::memoryHeapCount` elements, defining limit on
    /// maximum number of bytes that can be allocated out of particular Vulkan memory
    /// heap.
    ///
    /// Any of the elements may be equal to `VK_WHOLE_SIZE`, which means no limit on that
    /// heap. This is also the default in case of `pHeapSizeLimit` = NULL.
    ///
    /// If there is a limit defined for a heap:
    ///
    /// - If user tries to allocate more memory from that heap using this allocator,
    /// the allocation fails with `VK_ERROR_OUT_OF_DEVICE_MEMORY`.
    /// - If the limit is smaller than heap size reported in `VkMemoryHeap::size`, the
    /// value of this limit will be reported instead when using vmaGetMemoryProperties().
    ///
    /// Warning! Using this feature may not be equivalent to installing a `GPU` with
    /// smaller amount of memory, because graphics driver doesn't necessary fail new
    /// allocations with `VK_ERROR_OUT_OF_DEVICE_MEMORY` result when memory capacity is
    /// exceeded. It may return success and just silently migrate some device memory
    /// blocks to system RAM. This driver behavior can also be controlled using
    /// VK_AMD_memory_overallocation_behavior extension.
    pub heap_size_limit: Option<&'a [u64]>,
    /// Optional. The highest version of Vulkan that the application is designed to use.
    ///
    /// It must be a value in the format as created by macro `VK_MAKE_VERSION` or a constant like: `VK_API_VERSION_1_1`, `VK_API_VERSION_1_0`.
    /// The patch version number specified is ignored. Only the major and minor versions are considered.
    /// It must be less or equal (preferably equal) to value as passed to `vkCreateInstance` as `VkApplicationInfo::apiVersion`.
    /// Only versions 1.0, 1.1, 1.2, 1.3 are supported by the current implementation.
    /// Leaving it initialized to zero is equivalent to `VK_API_VERSION_1_0`.
    pub vulkan_api_version: u32,
    /// Either null or a pointer to an array of external memory handle types for each Vulkan memory type.
    ///
    /// If not NULL, it must be a pointer to an array of `VkPhysicalDeviceMemoryProperties::memoryTypeCount`
    /// elements, defining external memory handle types of particular Vulkan memory type,
    /// to be passed using `VkExportMemoryAllocateInfoKHR`.
    ///
    /// Any of the elements may be equal to 0, which means not to use `VkExportMemoryAllocateInfoKHR` on this memory type.
    /// This is also the default in case of `pTypeExternalMemoryHandleTypes` = NULL.
    pub external_memory_handle_types: Option<&'a [ffi::ExternalMemoryHandleTypeFlags]>,
}

/// Whether there is still some defragmentation work left to be done
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefragmentationStatus {
    Success,
    Incomplete,
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
    /// Creates a new allocator wrapped in an Arc and automatic destruction on drop
    pub unsafe fn new_arc(info: &AllocatorCreateInfo2) -> VulkanResult<AllocatorArc> {
        Self::new(info).map(|alloc| AllocatorArc(Arc::new(alloc)))
    }
    /// Creates the `Allocator` object.
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
    /// Returns information about existing `VmaAllocator` object - handle to Vulkan device etc.
    ///
    /// It might be useful if you want to keep just the `VmaAllocator` handle and fetch other required handles to
    /// `VkPhysicalDevice`, `VkDevice` etc. every time using this function.
    pub unsafe fn get_allocator_info(&self) -> ffi::AllocatorInfo {
        let mut out: ffi::AllocatorInfo = mem::zeroed();
        ffi::vmaGetAllocatorInfo(self.clone(), &mut out);
        out
    }
    /// PhysicalDeviceProperties are fetched from physicalDevice by the allocator.
    /// You can access it here, without fetching it again on your own.
    pub unsafe fn get_physical_device_properties(&self) -> &vk::PhysicalDeviceProperties {
        let mut out: *const vk::PhysicalDeviceProperties = mem::zeroed();
        ffi::vmaGetPhysicalDeviceProperties(self.clone(), &mut out);
        &*out
    }
    /// PhysicalDeviceMemoryProperties are fetched from physicalDevice by the allocator.
    /// You can access it here, without fetching it again on your own.
    pub unsafe fn get_memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        let mut out: *const vk::PhysicalDeviceMemoryProperties = mem::zeroed();
        ffi::vmaGetMemoryProperties(self.clone(), &mut out);
        &*out
    }
    /// Given Memory Type Index, returns Property Flags of this memory type.
    ///
    /// This is just a convenience function. Same information can be obtained using
    /// vmaGetMemoryProperties().
    pub unsafe fn get_memory_type_properties(
        &self,
        memory_type_index: u32,
    ) -> vk::MemoryPropertyFlags {
        let mut out: vk::MemoryPropertyFlags = mem::zeroed();
        ffi::vmaGetMemoryTypeProperties(self.clone(), memory_type_index, &mut out);
        out
    }
    /// Sets index of the current frame.
    pub unsafe fn set_current_frame_index(&self, frame_index: u32) {
        ffi::vmaSetCurrentFrameIndex(self.clone(), frame_index);
    }
    /// Retrieves statistics from current state of the Allocator.
    ///
    /// This function is called "calculate" not "get" because it has to traverse all
    /// internal data structures, so it may be quite slow. Use it for debugging purposes.
    /// For faster but more brief statistics suitable to be called every frame or every allocation,
    /// use vmaGetHeapBudgets().
    ///
    /// Note that when using allocator from multiple threads, returned information may immediately
    /// become outdated.
    pub unsafe fn calculate_statistics(&self) -> ffi::TotalStatistics {
        let mut out: ffi::TotalStatistics = mem::zeroed();
        ffi::vmaCalculateStatistics(self.clone(), &mut out);
        out
    }
    /// Retrieves information about current memory usage and budget for all memory heaps.
    ///
    /// - `allocator`
    /// - *out* `pBudgets` Must point to array with number of elements at least equal to number of memory heaps in physical device used.
    ///
    /// This function is called "get" not "calculate" because it is very fast, suitable to be called
    /// every frame or every allocation. For more detailed statistics use vmaCalculateStatistics().
    ///
    /// Note that when using allocator from multiple threads, returned information may immediately
    /// become outdated.
    pub unsafe fn get_heap_budgets(&self) -> Vec<ffi::Budget> {
        let properties = self.get_memory_properties();
        let mut out: Vec<ffi::Budget> = vec![mem::zeroed(); properties.memory_heap_count as usize];
        ffi::vmaGetHeapBudgets(self.clone(), out.as_mut_ptr());
        out
    }
    /// Helps to find memoryTypeIndex, given memoryTypeBits and VmaAllocationCreateInfo.
    ///
    /// This algorithm tries to find a memory type that:
    ///
    /// - Is allowed by memoryTypeBits.
    /// - Contains all the flags from pAllocationCreateInfo->requiredFlags.
    /// - Matches intended usage.
    /// - Has as many flags from pAllocationCreateInfo->preferredFlags as possible.
    ///
    /// \return Returns `VK_ERROR_FEATURE_NOT_PRESENT` if not found. Receiving such result
    /// from this function or any other allocating function probably means that your
    /// device doesn't support any memory type with requested features for the specific
    /// type of resource you want to use it for. Please check parameters of your
    /// resource, like image layout (OPTIMAL versus LINEAR) or mip level count.
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
    /// Helps to find memoryTypeIndex, given VkBufferCreateInfo and VmaAllocationCreateInfo.
    ///
    /// It can be useful e.g. to determine value to be used as VmaPoolCreateInfo::memoryTypeIndex.
    /// It internally creates a temporary, dummy buffer that never has memory bound.
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
    /// Helps to find memoryTypeIndex, given VkImageCreateInfo and VmaAllocationCreateInfo.
    ///
    /// It can be useful e.g. to determine value to be used as VmaPoolCreateInfo::memoryTypeIndex.
    /// It internally creates a temporary, dummy image that never has memory bound.
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
    /// Allocates Vulkan device memory and creates `VmaPool` object.
    ///
    /// - `allocator` Allocator object.
    /// - `pCreateInfo` Parameters of pool to create.
    /// - *out* `pPool` Handle to created pool.
    pub unsafe fn create_pool(&self, create_info: &ffi::PoolCreateInfo) -> VulkanResult<ffi::Pool> {
        let mut out: ffi::Pool = mem::zeroed();
        ffi::vmaCreatePool(self.clone(), create_info, &mut out).into_result(out)
    }
    /// Destroys `VmaPool` object and frees Vulkan device memory.
    pub unsafe fn destroy_pool(&self, pool: ffi::Pool) {
        ffi::vmaDestroyPool(self.clone(), pool);
    }
    /// Retrieves statistics of existing `VmaPool` object.
    ///
    /// - `allocator` Allocator object.
    /// - `pool` Pool object.
    /// - *out* `pPoolStats` Statistics of specified pool.
    pub unsafe fn get_pool_statistics(&self, pool: ffi::Pool) -> ffi::Statistics {
        let mut out: ffi::Statistics = mem::zeroed();
        ffi::vmaGetPoolStatistics(self.clone(), pool, &mut out);
        out
    }
    /// Retrieves detailed statistics of existing`VmaPool` object.
    ///
    /// - `allocator` Allocator object.
    /// - `pool` Pool object.
    /// - *out* `pPoolStats` Statistics of specified pool.
    pub unsafe fn calculate_pool_statistics(&self, pool: ffi::Pool) -> ffi::DetailedStatistics {
        let mut out: ffi::DetailedStatistics = mem::zeroed();
        ffi::vmaCalculatePoolStatistics(self.clone(), pool, &mut out);
        out
    }
    /// Checks magic number in margins around all allocations in given memory pool in search for corruptions.
    ///
    /// Corruption detection is enabled only when `VMA_DEBUG_DETECT_CORRUPTION` macro is defined to nonzero,
    /// `VMA_DEBUG_MARGIN` is defined to nonzero and the pool is created in memory type that is
    /// `HOST_VISIBLE` and `HOST_COHERENT`. For more information, see [Corruption detection](@ref debugging_memory_usage_corruption_detection).
    ///
    /// Possible return values:
    ///
    /// - `VK_ERROR_FEATURE_NOT_PRESENT` - corruption detection is not enabled for specified pool.
    /// - `VK_SUCCESS` - corruption detection has been performed and succeeded.
    /// - `VK_ERROR_UNKNOWN` - corruption detection has been performed and found memory corruptions around one of the allocations.
    /// `VMA_ASSERT` is also fired in that case.
    /// - Other value: Error returned by Vulkan, e.g. memory mapping failure.
    pub unsafe fn check_pool_corruption(&self, pool: ffi::Pool) -> VulkanResult<()> {
        ffi::vmaCheckPoolCorruption(self.clone(), pool).into_result(())
    }
    /// Retrieves name of a custom pool.
    ///
    /// After the call `ppName` is either null or points to an internally-owned null-terminated string
    /// containing name of the pool that was previously set. The pointer becomes invalid when the pool is
    /// destroyed or its name is changed using vmaSetPoolName().
    pub unsafe fn get_pool_name(&self, pool: ffi::Pool) -> Option<&'static std::ffi::CStr> {
        let mut out: *const std::ffi::c_char = mem::zeroed();
        ffi::vmaGetPoolName(self.clone(), pool, &mut out);
        if out.is_null() {
            None
        } else {
            Some(std::ffi::CStr::from_ptr(out))
        }
    }
    /// Sets name of a custom pool.
    ///
    /// `pName` can be either null or pointer to a null-terminated string with new name for the pool.
    /// Function makes internal copy of the string, so it can be changed or freed immediately after this call.
    pub unsafe fn set_pool_name(&self, pool: ffi::Pool, name: Option<&'static std::ffi::CStr>) {
        let ptr = match name {
            Some(name) => name.as_ptr(),
            None => std::ptr::null(),
        };
        ffi::vmaSetPoolName(self.clone(), pool, ptr);
    }
    /// General purpose memory allocation.
    ///
    /// - `allocator`
    /// - `pVkMemoryRequirements`
    /// - `pCreateInfo`
    /// - *out* `pAllocation` Handle to allocated memory.
    /// - *out* `pAllocationInfo` Optional. Information about allocated memory. It can be later fetched using function vmaGetAllocationInfo().
    ///
    /// You should free the memory using vmaFreeMemory() or vmaFreeMemoryPages().
    ///
    /// It is recommended to use vmaAllocateMemoryForBuffer(), vmaAllocateMemoryForImage(),
    /// vmaCreateBuffer(), vmaCreateImage() instead whenever possible.
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
    /// General purpose memory allocation for multiple allocation objects at once.
    ///
    /// - `allocator` Allocator object.
    /// - `pVkMemoryRequirements` Memory requirements for each allocation.
    /// - `pCreateInfo` Creation parameters for each allocation.
    /// - `allocationCount` Number of allocations to make.
    /// - *out* `pAllocations` Pointer to array that will be filled with handles to created allocations.
    /// - *out* `pAllocationInfo` Optional. Pointer to array that will be filled with parameters of created allocations.
    ///
    /// You should free the memory using vmaFreeMemory() or vmaFreeMemoryPages().
    ///
    /// Word "pages" is just a suggestion to use this function to allocate pieces of memory needed for sparse binding.
    /// It is just a general purpose allocation function able to make multiple allocations at once.
    /// It may be internally optimized to be more efficient than calling vmaAllocateMemory() `allocationCount` times.
    ///
    /// All allocations are made using same parameters. All of them are created out of the same memory pool and type.
    /// If any allocation fails, all allocations already made within this function call are also freed, so that when
    /// returned result is not `VK_SUCCESS`, `pAllocation` array is always entirely filled with `VK_NULL_HANDLE`.
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
    /// Allocates memory suitable for given `VkBuffer`.
    ///
    /// - `allocator`
    /// - `buffer`
    /// - `pCreateInfo`
    /// - *out* `pAllocation` Handle to allocated memory.
    /// - *out* `pAllocationInfo` Optional. Information about allocated memory. It can be later fetched using function vmaGetAllocationInfo().
    ///
    /// It only creates`VmaAllocation.` To bind the memory to the buffer, use vmaBindBufferMemory().
    ///
    /// This is a special-purpose function. In most cases you should use vmaCreateBuffer().
    ///
    /// You must free the allocation using vmaFreeMemory() when no longer needed.
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
    /// Allocates memory suitable for given `VkImage`.
    ///
    /// - `allocator`
    /// - `image`
    /// - `pCreateInfo`
    /// - *out* `pAllocation` Handle to allocated memory.
    /// - *out* `pAllocationInfo` Optional. Information about allocated memory. It can be later fetched using function vmaGetAllocationInfo().
    ///
    /// It only creates`VmaAllocation.` To bind the memory to the buffer, use vmaBindImageMemory().
    ///
    /// This is a special-purpose function. In most cases you should use vmaCreateImage().
    ///
    /// You must free the allocation using vmaFreeMemory() when no longer needed.
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
    /// Frees memory previously allocated using vmaAllocateMemory(), vmaAllocateMemoryForBuffer(), or vmaAllocateMemoryForImage().
    ///
    /// Passing `VK_NULL_HANDLE` as `allocation` is valid. Such function call is just skipped.
    pub unsafe fn free_memory(&self, allocation: ffi::Allocation) {
        ffi::vmaFreeMemory(self.clone(), allocation);
    }
    /// Frees memory and destroys multiple allocations.
    ///
    /// Word "pages" is just a suggestion to use this function to free pieces of memory used for sparse binding.
    /// It is just a general purpose function to free memory and destroy allocations made using e.g. vmaAllocateMemory(),
    /// vmaAllocateMemoryPages() and other functions.
    /// It may be internally optimized to be more efficient than calling vmaFreeMemory() `allocationCount` times.
    ///
    /// Allocations in `pAllocations` array can come from any memory pools and types.
    /// Passing `VK_NULL_HANDLE` as elements of `pAllocations` array is valid. Such entries are just skipped.
    pub unsafe fn free_memory_pages(&self, allocations: &[ffi::Allocation]) {
        ffi::vmaFreeMemoryPages(self.clone(), allocations.len(), allocations.as_ptr());
    }
    /// Returns current information about specified allocation.
    ///
    /// Current parameters of given allocation are returned in `pAllocationInfo`.
    ///
    /// Although this function doesn't lock any mutex, so it should be quite efficient,
    /// you should avoid calling it too often.
    /// You can retrieve same VmaAllocationInfo structure while creating your resource, from function
    /// vmaCreateBuffer(), vmaCreateImage(). You can remember it if you are sure parameters don't change
    /// (e.g. due to defragmentation).
    pub unsafe fn get_allocation_info(&self, allocation: ffi::Allocation) -> ffi::AllocationInfo {
        let mut out: ffi::AllocationInfo = mem::zeroed();
        ffi::vmaGetAllocationInfo(self.clone(), allocation, &mut out);
        out
    }
    /// Sets pUserData in given allocation to new value.
    ///
    /// The value of pointer `pUserData` is copied to allocation's `pUserData`.
    /// It is opaque, so you can use it however you want - e.g.
    /// as a pointer, ordinal number or some handle to you own data.
    pub unsafe fn set_allocation_user_data(
        &self,
        allocation: ffi::Allocation,
        user_data: *mut std::ffi::c_void,
    ) {
        ffi::vmaSetAllocationUserData(self.clone(), allocation, user_data);
    }
    /// Sets pName in given allocation to new value.
    ///
    /// `pName` must be either null, or pointer to a null-terminated string. The function
    /// makes local copy of the string and sets it as allocation's `pName`. String
    /// passed as pName doesn't need to be valid for whole lifetime of the allocation -
    /// you can free it after this call. String previously pointed by allocation's
    /// `pName` is freed from memory.
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
    /// Given an allocation, returns Property Flags of its memory type.
    ///
    /// This is just a convenience function. Same information can be obtained using
    /// vmaGetAllocationInfo() + vmaGetMemoryProperties().
    pub unsafe fn get_allocation_memory_properties(
        &self,
        allocation: ffi::Allocation,
    ) -> vk::MemoryPropertyFlags {
        let mut out: vk::MemoryPropertyFlags = mem::zeroed();
        ffi::vmaGetAllocationMemoryProperties(self.clone(), allocation, &mut out);
        out
    }
    /// Maps memory represented by given allocation and returns pointer to it.
    ///
    /// Maps memory represented by given allocation to make it accessible to `CPU` code.
    /// When succeeded, `*ppData` contains pointer to first byte of this memory.
    ///
    /// If the allocation is part of a bigger `VkDeviceMemory` block, returned pointer is
    /// correctly offsetted to the beginning of region assigned to this particular allocation.
    /// Unlike the result of `vkMapMemory`, it points to the allocation, not to the beginning of the whole block.
    /// You should not add VmaAllocationInfo::offset to it!** Mapping is internally reference-counted and synchronized, so despite raw Vulkan
    /// function `vkMapMemory()` cannot be used to map same block of `VkDeviceMemory`
    /// multiple times simultaneously, it is safe to call this function on allocations
    /// assigned to the same memory block. Actual Vulkan memory will be mapped on first
    /// mapping and unmapped on last unmapping.
    ///
    /// If the function succeeded, you must call vmaUnmapMemory() to unmap the
    /// allocation when mapping is no longer needed or before freeing the allocation, at
    /// the latest.
    ///
    /// It also safe to call this function multiple times on the same allocation. You
    /// must call vmaUnmapMemory() same number of times as you called vmaMapMemory().
    ///
    /// It is also safe to call this function on allocation created with
    /// `VMA_ALLOCATION_CREATE_MAPPED_BIT` flag. Its memory stays mapped all the time.
    /// You must still call vmaUnmapMemory() same number of times as you called
    /// vmaMapMemory(). You must not call vmaUnmapMemory() additional time to free the
    /// "0-th" mapping made automatically due to `VMA_ALLOCATION_CREATE_MAPPED_BIT` flag.
    ///
    /// This function fails when used on allocation made in memory type that is not
    /// `HOST_VISIBLE`.
    ///
    /// This function doesn't automatically flush or invalidate caches.
    /// If the allocation is made from a memory types that is not `HOST_COHERENT`,
    /// you also need to use vmaInvalidateAllocation() / vmaFlushAllocation(), as required by Vulkan specification.
    pub unsafe fn map_memory(&self, allocation: ffi::Allocation) -> VulkanResult<*mut u8> {
        let mut out: *mut std::ffi::c_void = mem::zeroed();
        ffi::vmaMapMemory(self.clone(), allocation, &mut out).into_result(out as *mut u8)
    }
    /// Unmaps memory represented by given allocation, mapped previously using vmaMapMemory().
    ///
    /// For details, see description of vmaMapMemory().
    ///
    /// This function doesn't automatically flush or invalidate caches.
    /// If the allocation is made from a memory types that is not `HOST_COHERENT`,
    /// you also need to use vmaInvalidateAllocation() / vmaFlushAllocation(), as required by Vulkan specification.
    pub unsafe fn unmap_memory(&self, allocation: ffi::Allocation) {
        ffi::vmaUnmapMemory(self.clone(), allocation);
    }
    /// Flushes memory of given allocation.
    ///
    /// Calls `vkFlushMappedMemoryRanges()` for memory associated with given range of given allocation.
    /// It needs to be called after writing to a mapped memory for memory types that are not `HOST_COHERENT`.
    /// Unmap operation doesn't do that automatically.
    ///
    /// - `offset` must be relative to the beginning of allocation.
    /// - `size` can be `VK_WHOLE_SIZE`. It means all memory from `offset` the the end of given allocation.
    /// - `offset` and `size` don't have to be aligned.
    /// They are internally rounded down/up to multiply of `nonCoherentAtomSize`.
    /// - If `size` is 0, this call is ignored.
    /// - If memory type that the `allocation` belongs to is not `HOST_VISIBLE` or it is `HOST_COHERENT`,
    /// this call is ignored.
    ///
    /// Warning! `offset` and `size` are relative to the contents of given `allocation`.
    /// If you mean whole allocation, you can pass 0 and `VK_WHOLE_SIZE`, respectively.
    /// Do not pass allocation's offset as `offset`!!!
    ///
    /// This function returns the `VkResult` from `vkFlushMappedMemoryRanges` if it is
    /// called, otherwise `VK_SUCCESS`.
    pub unsafe fn flush_allocation(
        &self,
        allocation: ffi::Allocation,
        offset: u64,
        size: u64,
    ) -> VulkanResult<()> {
        ffi::vmaFlushAllocation(self.clone(), allocation, offset, size).into_result(())
    }
    /// Invalidates memory of given allocation.
    ///
    /// Calls `vkInvalidateMappedMemoryRanges()` for memory associated with given range of given allocation.
    /// It needs to be called before reading from a mapped memory for memory types that are not `HOST_COHERENT`.
    /// Map operation doesn't do that automatically.
    ///
    /// - `offset` must be relative to the beginning of allocation.
    /// - `size` can be `VK_WHOLE_SIZE`. It means all memory from `offset` the the end of given allocation.
    /// - `offset` and `size` don't have to be aligned.
    /// They are internally rounded down/up to multiply of `nonCoherentAtomSize`.
    /// - If `size` is 0, this call is ignored.
    /// - If memory type that the `allocation` belongs to is not `HOST_VISIBLE` or it is `HOST_COHERENT`,
    /// this call is ignored.
    ///
    /// Warning! `offset` and `size` are relative to the contents of given `allocation`.
    /// If you mean whole allocation, you can pass 0 and `VK_WHOLE_SIZE`, respectively.
    /// Do not pass allocation's offset as `offset`!!!
    ///
    /// This function returns the `VkResult` from `vkInvalidateMappedMemoryRanges` if
    /// it is called, otherwise `VK_SUCCESS`.
    pub unsafe fn invalidate_allocation(
        &self,
        allocation: ffi::Allocation,
        offset: u64,
        size: u64,
    ) -> VulkanResult<()> {
        ffi::vmaInvalidateAllocation(self.clone(), allocation, offset, size).into_result(())
    }
    /// Flushes memory of given set of allocations.
    ///
    /// Calls `vkFlushMappedMemoryRanges()` for memory associated with given ranges of given allocations.
    /// For more information, see documentation of vmaFlushAllocation().
    ///
    /// - `allocator`
    /// - `allocationCount`
    /// - `allocations`
    /// - `offsets` If not null, it must point to an array of offsets of regions to flush, relative to the beginning of respective allocations. Null means all ofsets are zero.
    /// - `sizes` If not null, it must point to an array of sizes of regions to flush in respective allocations. Null means `VK_WHOLE_SIZE` for all allocations.
    ///
    /// This function returns the `VkResult` from `vkFlushMappedMemoryRanges` if it is
    /// called, otherwise `VK_SUCCESS`.
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
    /// Invalidates memory of given set of allocations.
    ///
    /// Calls `vkInvalidateMappedMemoryRanges()` for memory associated with given ranges of given allocations.
    /// For more information, see documentation of vmaInvalidateAllocation().
    ///
    /// - `allocator`
    /// - `allocationCount`
    /// - `allocations`
    /// - `offsets` If not null, it must point to an array of offsets of regions to flush, relative to the beginning of respective allocations. Null means all ofsets are zero.
    /// - `sizes` If not null, it must point to an array of sizes of regions to flush in respective allocations. Null means `VK_WHOLE_SIZE` for all allocations.
    ///
    /// This function returns the `VkResult` from `vkInvalidateMappedMemoryRanges` if it is
    /// called, otherwise `VK_SUCCESS`.
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
    /// Checks magic number in margins around all allocations in given memory types (in both default and custom pools) in search for corruptions.
    ///
    /// - `allocator`
    /// - `memoryTypeBits` Bit mask, where each bit set means that a memory type with that index should be checked.
    ///
    /// Corruption detection is enabled only when `VMA_DEBUG_DETECT_CORRUPTION` macro is defined to nonzero,
    /// `VMA_DEBUG_MARGIN` is defined to nonzero and only for memory types that are
    /// `HOST_VISIBLE` and `HOST_COHERENT`. For more information, see [Corruption detection](@ref debugging_memory_usage_corruption_detection).
    ///
    /// Possible return values:
    ///
    /// - `VK_ERROR_FEATURE_NOT_PRESENT` - corruption detection is not enabled for any of specified memory types.
    /// - `VK_SUCCESS` - corruption detection has been performed and succeeded.
    /// - `VK_ERROR_UNKNOWN` - corruption detection has been performed and found memory corruptions around one of the allocations.
    /// `VMA_ASSERT` is also fired in that case.
    /// - Other value: Error returned by Vulkan, e.g. memory mapping failure.
    pub unsafe fn check_corruption(&self, memory_type_bits: u32) -> VulkanResult<()> {
        ffi::vmaCheckCorruption(self.clone(), memory_type_bits).into_result(())
    }
    /// Begins defragmentation process.
    ///
    /// - `allocator` Allocator object.
    /// - `pInfo` Structure filled with parameters of defragmentation.
    /// - *out* `pContext` Context object that must be passed to vmaEndDefragmentation() to finish defragmentation.
    /// \returns
    /// - `VK_SUCCESS` if defragmentation can begin.
    /// - `VK_ERROR_FEATURE_NOT_PRESENT` if defragmentation is not supported.
    ///
    /// For more information about defragmentation, see documentation chapter:
    /// [Defragmentation](@ref defragmentation).
    pub unsafe fn begin_defragmentation(
        &self,
        info: &ffi::DefragmentationInfo,
    ) -> VulkanResult<ffi::DefragmentationContext> {
        let mut context: ffi::DefragmentationContext = mem::zeroed();
        ffi::vmaBeginDefragmentation(self.clone(), info, &mut context).into_result(context)
    }
    /// Ends defragmentation process.
    ///
    /// - `allocator` Allocator object.
    /// - `context` Context object that has been created by vmaBeginDefragmentation().
    /// - *out* `pStats` Optional stats for the defragmentation. Can be null.
    ///
    /// Use this function to finish defragmentation started by vmaBeginDefragmentation().
    pub unsafe fn end_defragmentation(
        &self,
        context: ffi::DefragmentationContext,
    ) -> ffi::DefragmentationStats {
        let mut stats: ffi::DefragmentationStats = mem::zeroed();
        ffi::vmaEndDefragmentation(self.clone(), context, &mut stats);
        stats
    }
    /// Starts single defragmentation pass.
    ///
    /// - `allocator` Allocator object.
    /// - `context` Context object that has been created by vmaBeginDefragmentation().
    /// - *out* `pPassInfo` Computed information for current pass.
    /// \returns
    /// - `VK_SUCCESS` if no more moves are possible. Then you can omit call to vmaEndDefragmentationPass() and simply end whole defragmentation.
    /// - `VK_INCOMPLETE` if there are pending moves returned in `pPassInfo`. You need to perform them, call vmaEndDefragmentationPass(),
    /// and then preferably try another pass with vmaBeginDefragmentationPass().
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
    /// Ends single defragmentation pass.
    ///
    /// - `allocator` Allocator object.
    /// - `context` Context object that has been created by vmaBeginDefragmentation().
    /// - `pPassInfo` Computed information for current pass filled by vmaBeginDefragmentationPass() and possibly modified by you.
    ///
    /// Returns `VK_SUCCESS` if no more moves are possible or `VK_INCOMPLETE` if more defragmentations are possible.
    ///
    /// Ends incremental defragmentation pass and commits all defragmentation moves from `pPassInfo`.
    /// After this call:
    ///
    /// - Allocations at `pPassInfo[i].srcAllocation` that had `pPassInfo[i].operation ==``VMA_DEFRAGMENTATION_MOVE_OPERATION_COPY`
    /// (which is the default) will be pointing to the new destination place.
    /// - Allocation at `pPassInfo[i].srcAllocation` that had `pPassInfo[i].operation ==``VMA_DEFRAGMENTATION_MOVE_OPERATION_DESTROY`
    /// will be freed.
    ///
    /// If no more moves are possible you can end whole defragmentation.
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
    /// Binds buffer to allocation.
    ///
    /// Binds specified buffer to region of memory represented by specified allocation.
    /// Gets `VkDeviceMemory` handle and offset from the allocation.
    /// If you want to create a buffer, allocate memory for it and bind them together separately,
    /// you should use this function for binding instead of standard `vkBindBufferMemory()`,
    /// because it ensures proper synchronization so that when a `VkDeviceMemory` object is used by multiple
    /// allocations, calls to `vkBind*Memory()` or `vkMapMemory()` won't happen from multiple threads simultaneously
    /// (which is illegal in Vulkan).
    ///
    /// It is recommended to use function vmaCreateBuffer() instead of this one.
    pub unsafe fn bind_buffer_memory(
        &self,
        allocation: ffi::Allocation,
        buffer: vk::Buffer,
    ) -> VulkanResult<()> {
        ffi::vmaBindBufferMemory(self.clone(), allocation, buffer).into_result(())
    }
    /// Binds buffer to allocation with additional parameters.
    ///
    /// - `allocator`
    /// - `allocation`
    /// - `allocationLocalOffset` Additional offset to be added while binding, relative to the beginning of the `allocation`. Normally it should be 0.
    /// - `buffer`
    /// - `pNext` A chain of structures to be attached to `VkBindBufferMemoryInfoKHR` structure used internally. Normally it should be null.
    ///
    /// This function is similar to vmaBindBufferMemory(), but it provides additional parameters.
    ///
    /// If `pNext` is not null, `VmaAllocator` object must have been created with `VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT` flag
    /// or with VmaAllocatorCreateInfo::vulkanApiVersion `>= VK_API_VERSION_1_1`. Otherwise the call fails.
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
    /// Binds image to allocation.
    ///
    /// Binds specified image to region of memory represented by specified allocation.
    /// Gets `VkDeviceMemory` handle and offset from the allocation.
    /// If you want to create an image, allocate memory for it and bind them together separately,
    /// you should use this function for binding instead of standard `vkBindImageMemory()`,
    /// because it ensures proper synchronization so that when a `VkDeviceMemory` object is used by multiple
    /// allocations, calls to `vkBind*Memory()` or `vkMapMemory()` won't happen from multiple threads simultaneously
    /// (which is illegal in Vulkan).
    ///
    /// It is recommended to use function vmaCreateImage() instead of this one.
    pub unsafe fn bind_image_memory(
        &self,
        allocation: ffi::Allocation,
        image: vk::Image,
    ) -> VulkanResult<()> {
        ffi::vmaBindImageMemory(self.clone(), allocation, image).into_result(())
    }
    /// Binds image to allocation with additional parameters.
    ///
    /// - `allocator`
    /// - `allocation`
    /// - `allocationLocalOffset` Additional offset to be added while binding, relative to the beginning of the `allocation`. Normally it should be 0.
    /// - `image`
    /// - `pNext` A chain of structures to be attached to `VkBindImageMemoryInfoKHR` structure used internally. Normally it should be null.
    ///
    /// This function is similar to vmaBindImageMemory(), but it provides additional parameters.
    ///
    /// If `pNext` is not null, `VmaAllocator` object must have been created with `VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT` flag
    /// or with VmaAllocatorCreateInfo::vulkanApiVersion `>= VK_API_VERSION_1_1`. Otherwise the call fails.
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
    /// Creates a new `VkBuffer`, allocates and binds memory for it.
    ///
    /// - `allocator`
    /// - `pBufferCreateInfo`
    /// - `pAllocationCreateInfo`
    /// - *out* `pBuffer` Buffer that was created.
    /// - *out* `pAllocation` Allocation that was created.
    /// - *out* `pAllocationInfo` Optional. Information about allocated memory. It can be later fetched using function vmaGetAllocationInfo().
    ///
    /// This function automatically:
    ///
    /// -# Creates buffer.
    /// -# Allocates appropriate memory for it.
    /// -# Binds the buffer with the memory.
    ///
    /// If any of these operations fail, buffer and allocation are not created,
    /// returned value is negative error code, `*pBuffer` and `*pAllocation` are null.
    ///
    /// If the function succeeded, you must destroy both buffer and allocation when you
    /// no longer need them using either convenience function vmaDestroyBuffer() or
    /// separately, using `vkDestroyBuffer()` and vmaFreeMemory().
    ///
    /// If `VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT` flag was used,
    /// VK_KHR_dedicated_allocation extension is used internally to query driver whether
    /// it requires or prefers the new buffer to have dedicated allocation. If yes,
    /// and if dedicated allocation is possible
    /// (#VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT is not used), it creates dedicated
    /// allocation for this buffer, just like when using
    /// `VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT.`
    ///
    /// This function creates a new `VkBuffer`. Sub-allocation of parts of one large buffer,
    /// although recommended as a good practice, is out of scope of this library and could be implemented
    /// by the user as a higher-level logic on top of VMA.
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
    /// Creates a buffer with additional minimum alignment.
    ///
    /// Similar to vmaCreateBuffer() but provides additional parameter `minAlignment` which allows to specify custom,
    /// minimum alignment to be used when placing the buffer inside a larger memory block, which may be needed e.g.
    /// for interop with OpenGL.
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
    /// Creates a new `VkBuffer`, binds already created memory for it.
    ///
    /// - `allocator`
    /// - `allocation` Allocation that provides memory to be used for binding new buffer to it.
    /// - `pBufferCreateInfo`
    /// - *out* `pBuffer` Buffer that was created.
    ///
    /// This function automatically:
    ///
    /// -# Creates buffer.
    /// -# Binds the buffer with the supplied memory.
    ///
    /// If any of these operations fail, buffer is not created,
    /// returned value is negative error code and `*pBuffer` is null.
    ///
    /// If the function succeeded, you must destroy the buffer when you
    /// no longer need it using `vkDestroyBuffer()`. If you want to also destroy the corresponding
    /// allocation you can use convenience function vmaDestroyBuffer().
    ///
    /// There is a new version of this function augmented with parameter `allocationLocalOffset` - see vmaCreateAliasingBuffer2().
    pub unsafe fn create_aliasing_buffer(
        &self,
        allocation: ffi::Allocation,
        buffer_create_info: &vk::BufferCreateInfo,
    ) -> VulkanResult<vk::Buffer> {
        let mut buffer: vk::Buffer = mem::zeroed();
        ffi::vmaCreateAliasingBuffer(self.clone(), allocation, buffer_create_info, &mut buffer)
            .into_result(buffer)
    }
    /// Creates a new `VkBuffer`, binds already created memory for it.
    ///
    /// - `allocator`
    /// - `allocation` Allocation that provides memory to be used for binding new buffer to it.
    /// - `allocationLocalOffset` Additional offset to be added while binding, relative to the beginning of the allocation. Normally it should be 0.
    /// - `pBufferCreateInfo`
    /// - *out* `pBuffer` Buffer that was created.
    ///
    /// This function automatically:
    ///
    /// -# Creates buffer.
    /// -# Binds the buffer with the supplied memory.
    ///
    /// If any of these operations fail, buffer is not created,
    /// returned value is negative error code and `*pBuffer` is null.
    ///
    /// If the function succeeded, you must destroy the buffer when you
    /// no longer need it using `vkDestroyBuffer()`. If you want to also destroy the corresponding
    /// allocation you can use convenience function vmaDestroyBuffer().
    ///
    /// This is a new version of the function augmented with parameter `allocationLocalOffset`.
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
    /// Destroys Vulkan buffer and frees allocated memory.
    ///
    /// This is just a convenience function equivalent to:
    ///
    /// ```ignore
    /// vkDestroyBuffer(device, buffer, allocationCallbacks);
    /// vmaFreeMemory(allocator, allocation);
    /// ```
    ///
    /// It is safe to pass null as buffer and/or allocation.

    pub unsafe fn destroy_buffer(&self, buffer: vk::Buffer, allocation: ffi::Allocation) {
        ffi::vmaDestroyBuffer(self.clone(), buffer, allocation);
    }
    /// Function similar to vmaCreateBuffer().
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
    /// Function similar to vmaCreateAliasingBuffer() but for images.
    pub unsafe fn create_aliasing_image(
        &self,
        allocation: ffi::Allocation,
        image_create_info: &vk::ImageCreateInfo,
    ) -> VulkanResult<vk::Image> {
        let mut image: vk::Image = mem::zeroed();
        ffi::vmaCreateAliasingImage(self.clone(), allocation, image_create_info, &mut image)
            .into_result(image)
    }
    /// Function similar to vmaCreateAliasingBuffer2() but for images.
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
    /// Destroys Vulkan image and frees allocated memory.
    ///
    /// This is just a convenience function equivalent to:
    ///
    /// ```ignore
    /// vkDestroyImage(device, image, allocationCallbacks);
    /// vmaFreeMemory(allocator, allocation);
    /// ```
    ///
    /// It is safe to pass null as image and/or allocation.
    pub unsafe fn destroy_image(&self, image: vk::Image, allocation: ffi::Allocation) {
        ffi::vmaDestroyImage(self.clone(), image, allocation);
    }
    /// Builds and returns statistics as a null-terminated string in `JSON` format.
    /// - `allocator`
    /// - *out* `ppStatsString` Must be freed using vmaFreeStatsString() function.
    /// - `detailedMap`
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

macro_rules! ffi_copy {
    ($e:expr) => {
        ManuallyDrop::into_inner($e.unsafe_copy())
    };
}

/// An interface to the inner allocation algorithms, doesn't actually allocate any real memory.
///
/// Please note that you should consciously handle virtual allocations that could remain unfreed in the block.
/// You should either free them individually using vmaVirtualFree() or call vmaClearVirtualBlock()
/// if you are sure this is what you want. If you do neither, an assert is called.
///
/// If you keep pointers to some additional metadata associated with your virtual allocations in their `pUserData`,
/// don't forget to free them.
impl ffi::VirtualBlock {
    /// Creates new `VmaVirtualBlock` object.
    pub fn new(info: &ffi::VirtualBlockCreateInfo) -> Self {
        unsafe {
            let mut out: ffi::VirtualBlock = mem::zeroed();
            ffi::vmaCreateVirtualBlock(info, &mut out);
            out
        }
    }
    /// Returns true of the `VmaVirtualBlock` is empty - contains 0 virtual allocations and has all its space available for new allocations.
    pub fn is_empty(&self) -> bool {
        unsafe { ffi::vmaIsVirtualBlockEmpty(ffi_copy!(self.clone())) == vk::TRUE }
    }
    /// Returns information about a specific virtual allocation within a virtual block, like its size and `pUserData` pointer.
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
    /// Allocates new virtual allocation inside given `VmaVirtualBlock.`
    ///
    /// If the allocation fails due to not enough free space available, `VK_ERROR_OUT_OF_DEVICE_MEMORY` is returned
    /// (despite the function doesn't ever allocate actual `GPU` memory).
    /// `pAllocation` is then set to `VK_NULL_HANDLE` and `pOffset`, if not null, it set to `UINT64_MAX`.
    ///
    /// - `virtualBlock` Virtual block
    /// - `pCreateInfo` Parameters for the allocation
    /// - *out* `pAllocation` Returned handle of the new allocation
    /// - *out* `pOffset` Returned offset of the new allocation. Optional, can be null.
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
    /// Frees virtual allocation inside given `VmaVirtualBlock.`
    ///
    /// It is correct to call this function with `allocation == VK_NULL_HANDLE` - it does nothing.
    pub fn free(&self, allocation: ffi::VirtualAllocation) {
        unsafe {
            ffi::vmaVirtualFree(ffi_copy!(self.clone()), allocation);
        }
    }
    /// Frees all virtual allocations inside given `VmaVirtualBlock.`
    ///
    /// You must either call this function or free each virtual allocation individually with vmaVirtualFree()
    /// before destroying a virtual block. Otherwise, an assert is called.
    ///
    /// If you keep pointer to some additional metadata associated with your virtual allocation in its `pUserData`,
    /// don't forget to free it as well.
    pub fn clear(&self) {
        unsafe {
            ffi::vmaClearVirtualBlock(ffi_copy!(self.clone()));
        }
    }
    /// Changes custom pointer associated with given virtual allocation.
    pub fn set_allocation_user_data(
        &self,
        allocation: ffi::VirtualAllocation,
        user_data: *mut std::ffi::c_void,
    ) {
        unsafe {
            ffi::vmaSetVirtualAllocationUserData(ffi_copy!(self.clone()), allocation, user_data);
        }
    }
    /// Calculates and returns statistics about virtual allocations and memory usage in given `VmaVirtualBlock.`
    ///
    /// This function is fast to call. For more detailed statistics, see vmaCalculateVirtualBlockStatistics().
    pub fn get_statistics(&self) -> ffi::Statistics {
        unsafe {
            let mut out: ffi::Statistics = mem::zeroed();
            ffi::vmaGetVirtualBlockStatistics(ffi_copy!(self.clone()), &mut out);
            out
        }
    }
    /// Calculates and returns detailed statistics about virtual allocations and memory usage in given `VmaVirtualBlock.`
    ///
    /// This function is slow to call. Use for debugging purposes.
    /// For less detailed statistics, see vmaGetVirtualBlockStatistics().
    pub fn calculate_statistics(&self) -> ffi::DetailedStatistics {
        unsafe {
            let mut out: ffi::DetailedStatistics = mem::zeroed();
            ffi::vmaCalculateVirtualBlockStatistics(ffi_copy!(self.clone()), &mut out);
            out
        }
    }
    /// Builds and returns a null-terminated string in `JSON` format with information about given `VmaVirtualBlock.`
    /// - `virtualBlock` Virtual block.
    /// - *out* `ppStatsString` Returned string.
    /// - `detailedMap` Pass `VK_FALSE` to only obtain statistics as returned by vmaCalculateVirtualBlockStatistics(). Pass `VK_TRUE` to also obtain full list of allocations and free spaces.
    ///
    /// Returned string must be freed using vmaFreeVirtualBlockStatsString().
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
