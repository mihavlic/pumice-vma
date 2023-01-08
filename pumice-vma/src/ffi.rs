#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deprecated)]
use pumice::{util::ObjectHandle, vk};
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(
    instance: vk::Instance,
    p_name: *const std::os::raw::c_char,
) -> vk::PfnVoidFunction;
pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(
    device: vk::Device,
    p_name: *const std::os::raw::c_char,
) -> vk::PfnVoidFunction;
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
    physical_device: vk::PhysicalDevice,
    p_properties: *mut vk::PhysicalDeviceProperties,
);
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
    physical_device: vk::PhysicalDevice,
    p_memory_properties: *mut vk::PhysicalDeviceMemoryProperties,
);
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
    device: vk::Device,
    p_allocate_info: *const vk::MemoryAllocateInfo,
    p_allocator: *const vk::AllocationCallbacks,
    p_memory: *mut vk::DeviceMemory,
) -> vk::Result;
pub type PFN_vkFreeMemory = unsafe extern "system" fn(
    device: vk::Device,
    memory: vk::DeviceMemory,
    p_allocator: *const vk::AllocationCallbacks,
);
pub type PFN_vkMapMemory = unsafe extern "system" fn(
    device: vk::Device,
    memory: vk::DeviceMemory,
    offset: vk::DeviceSize,
    size: vk::DeviceSize,
    flags: vk::MemoryMapFlags,
    pp_data: *mut *mut std::os::raw::c_void,
) -> vk::Result;
pub type PFN_vkUnmapMemory = unsafe extern "system" fn(
    device: vk::Device,
    memory: vk::DeviceMemory,
);
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
    device: vk::Device,
    memory_range_count: u32,
    p_memory_ranges: *const vk::MappedMemoryRange,
) -> vk::Result;
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
    device: vk::Device,
    memory_range_count: u32,
    p_memory_ranges: *const vk::MappedMemoryRange,
) -> vk::Result;
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(
    device: vk::Device,
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    memory_offset: vk::DeviceSize,
) -> vk::Result;
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(
    device: vk::Device,
    image: vk::Image,
    memory: vk::DeviceMemory,
    memory_offset: vk::DeviceSize,
) -> vk::Result;
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(
    device: vk::Device,
    buffer: vk::Buffer,
    p_memory_requirements: *mut vk::MemoryRequirements,
);
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(
    device: vk::Device,
    image: vk::Image,
    p_memory_requirements: *mut vk::MemoryRequirements,
);
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
    device: vk::Device,
    p_create_info: *const vk::BufferCreateInfo,
    p_allocator: *const vk::AllocationCallbacks,
    p_buffer: *mut vk::Buffer,
) -> vk::Result;
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
    device: vk::Device,
    buffer: vk::Buffer,
    p_allocator: *const vk::AllocationCallbacks,
);
pub type PFN_vkCreateImage = unsafe extern "system" fn(
    device: vk::Device,
    p_create_info: *const vk::ImageCreateInfo,
    p_allocator: *const vk::AllocationCallbacks,
    p_image: *mut vk::Image,
) -> vk::Result;
pub type PFN_vkDestroyImage = unsafe extern "system" fn(
    device: vk::Device,
    image: vk::Image,
    p_allocator: *const vk::AllocationCallbacks,
);
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(
    command_buffer: vk::CommandBuffer,
    src_buffer: vk::Buffer,
    dst_buffer: vk::Buffer,
    region_count: u32,
    p_regions: *const vk::BufferCopy,
);
/// Fallback type for function pointers that are not generated
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PFN_UNAVAILABLE(unsafe extern "system" fn());
/// VK_VERSION_1_1 or VK_KHR_dedicated_allocation
#[cfg(feature = "VK_VERSION_1_1")]
pub type PFN_vkGetBufferMemoryRequirements2KHR = unsafe extern "system" fn(
    device: vk::Device,
    p_info: *const vk::BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut vk::MemoryRequirements2,
);
#[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_dedicated_allocation"))]
pub type PFN_vkGetBufferMemoryRequirements2KHR = unsafe extern "system" fn(
    device: vk::Device,
    p_info: *const vk::BufferMemoryRequirementsInfo2KHR,
    p_memory_requirements: *mut vk::MemoryRequirements2KHR,
);
#[cfg(
    all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_dedicated_allocation"))
)]
pub type PFN_vkGetBufferMemoryRequirements2KHR = PFN_UNAVAILABLE;
/// VK_VERSION_1_1 or VK_KHR_dedicated_allocation
#[cfg(feature = "VK_VERSION_1_1")]
pub type PFN_vkGetImageMemoryRequirements2KHR = unsafe extern "system" fn(
    device: vk::Device,
    p_info: *const vk::ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut vk::MemoryRequirements2,
);
#[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_dedicated_allocation"))]
pub type PFN_vkGetImageMemoryRequirements2KHR = unsafe extern "system" fn(
    device: vk::Device,
    p_info: *const vk::ImageMemoryRequirementsInfo2KHR,
    p_memory_requirements: *mut vk::MemoryRequirements2KHR,
);
#[cfg(
    all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_dedicated_allocation"))
)]
pub type PFN_vkGetImageMemoryRequirements2KHR = PFN_UNAVAILABLE;
/// VK_VERSION_1_1 or VK_KHR_bind_memory2
#[cfg(feature = "VK_VERSION_1_1")]
pub type PFN_vkBindBufferMemory2KHR = unsafe extern "system" fn(
    device: vk::Device,
    bind_info_count: u32,
    p_bind_infos: *const vk::BindBufferMemoryInfo,
) -> vk::Result;
#[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_bind_memory2"))]
pub type PFN_vkBindBufferMemory2KHR = unsafe extern "system" fn(
    device: vk::Device,
    bind_info_count: u32,
    p_bind_infos: *const vk::BindBufferMemoryInfoKHR,
) -> vk::Result;
#[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_bind_memory2")))]
pub type PFN_vkBindBufferMemory2KHR = PFN_UNAVAILABLE;
/// VK_VERSION_1_1 or VK_KHR_bind_memory2
#[cfg(feature = "VK_VERSION_1_1")]
pub type PFN_vkBindImageMemory2KHR = unsafe extern "system" fn(
    device: vk::Device,
    bind_info_count: u32,
    p_bind_infos: *const vk::BindImageMemoryInfo,
) -> vk::Result;
#[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_bind_memory2"))]
pub type PFN_vkBindImageMemory2KHR = unsafe extern "system" fn(
    device: vk::Device,
    bind_info_count: u32,
    p_bind_infos: *const vk::BindImageMemoryInfoKHR,
) -> vk::Result;
#[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_bind_memory2")))]
pub type PFN_vkBindImageMemory2KHR = PFN_UNAVAILABLE;
/// VK_VERSION_1_1 or VK_KHR_get_physical_device_properties2
#[cfg(feature = "VK_VERSION_1_1")]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = unsafe extern "system" fn(
    physical_device: vk::PhysicalDevice,
    p_memory_properties: *mut vk::PhysicalDeviceMemoryProperties2,
);
#[cfg(
    all(
        not(feature = "VK_VERSION_1_1"),
        feature = "VK_KHR_get_physical_device_properties2"
    )
)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = unsafe extern "system" fn(
    physical_device: vk::PhysicalDevice,
    p_memory_properties: *mut vk::PhysicalDeviceMemoryProperties2KHR,
);
#[cfg(
    all(
        not(feature = "VK_VERSION_1_1"),
        not(feature = "VK_KHR_get_physical_device_properties2")
    )
)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = PFN_UNAVAILABLE;
/// VK_VERSION_1_3 or (VK_VERSION_1_1 and VK_KHR_maintenance4)
#[cfg(feature = "VK_VERSION_1_3")]
pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
    device: vk::Device,
    p_info: *const vk::DeviceBufferMemoryRequirements,
    p_memory_requirements: *mut vk::MemoryRequirements2,
);
#[cfg(
    all(
        not(feature = "VK_VERSION_1_3"),
        all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4")
    )
)]
pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
    device: vk::Device,
    p_info: *const vk::DeviceBufferMemoryRequirementsKHR,
    p_memory_requirements: *mut vk::MemoryRequirements2KHR,
);
#[cfg(
    all(
        not(feature = "VK_VERSION_1_3"),
        not(all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4"))
    )
)]
pub type PFN_vkGetDeviceBufferMemoryRequirements = PFN_UNAVAILABLE;
/// VK_VERSION_1_3 or (VK_VERSION_1_1 and VK_KHR_maintenance4)
#[cfg(feature = "VK_VERSION_1_3")]
pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
    device: vk::Device,
    p_info: *const vk::DeviceImageMemoryRequirements,
    p_memory_requirements: *mut vk::MemoryRequirements2,
);
#[cfg(
    all(
        not(feature = "VK_VERSION_1_3"),
        all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4")
    )
)]
pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
    device: vk::Device,
    p_info: *const vk::DeviceImageMemoryRequirementsKHR,
    p_memory_requirements: *mut vk::MemoryRequirements2KHR,
);
#[cfg(
    all(
        not(feature = "VK_VERSION_1_3"),
        not(all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4"))
    )
)]
pub type PFN_vkGetDeviceImageMemoryRequirements = PFN_UNAVAILABLE;
pub type PFN_vmaAllocateDeviceMemoryFunction = unsafe extern "C" fn(
    allocator: Allocator,
    memory_type: u32,
    memory: vk::DeviceMemory,
    size: u64,
);
pub type PFN_vmaFreeDeviceMemoryFunction = unsafe extern "C" fn(
    allocator: Allocator,
    memory_type: u32,
    memory: vk::DeviceMemory,
    size: u64,
);
#[cfg(feature = "VK_VERSION_1_1")]
pub type ExternalMemoryHandleTypeFlags = vk::ExternalMemoryHandleTypeFlags;
#[cfg(
    all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_external_memory_capabilities")
)]
pub type ExternalMemoryHandleTypeFlags = vk::ExternalMemoryHandleTypeFlagsKHR;
#[cfg(
    all(
        not(feature = "VK_VERSION_1_1"),
        not(feature = "VK_KHR_external_memory_capabilities")
    )
)]
pub type ExternalMemoryHandleTypeFlags = u32;
macro_rules! non_dispatchable_handle {
    ($name:ident) => {
        #[repr(transparent)] #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash,
        Default)] pub struct $name (pub u64); impl $name { pub const fn null() -> Self {
        Self(0) } } impl std::fmt::Pointer for $name { fn fmt(& self, f : & mut
        std::fmt::Formatter) -> std::fmt::Result { write!(f, "0x{:x}", self.0) } } impl
        std::fmt::Debug for $name { fn fmt(& self, f : & mut std::fmt::Formatter) ->
        std::fmt::Result { write!(f, "0x{:x}", self.0) } }
    };
}
macro_rules! dispatchable_handle {
    ($name:ident) => {
        #[repr(transparent)] #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $name (pub * mut ()); impl $name { pub const fn null() -> Self {
        Self(std::ptr::null_mut()) } } impl Default for $name { fn default() -> Self {
        Self(std::ptr::null_mut()) } } unsafe impl Send for $name {} unsafe impl Sync for
        $name {} impl std::fmt::Pointer for $name { fn fmt(& self, f : & mut
        std::fmt::Formatter) -> std::fmt::Result { std::fmt::Pointer::fmt(& self.0, f) }
        } impl std::fmt::Debug for $name { fn fmt(& self, f : & mut std::fmt::Formatter)
        -> std::fmt::Result { std::fmt::Debug::fmt(& self.0, f) } }
    };
}
macro_rules! dispatchable_handle_eh {
    ($name:ident) => {
        #[repr(transparent)] #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash)] pub
        struct $name (pub * mut ()); impl $name { pub const fn null() -> Self {
        Self(std::ptr::null_mut()) } } unsafe impl Send for $name {} unsafe impl Sync for
        $name {} impl std::fmt::Pointer for $name { fn fmt(& self, f : & mut
        std::fmt::Formatter) -> std::fmt::Result { std::fmt::Pointer::fmt(& self.0, f) }
        } impl std::fmt::Debug for $name { fn fmt(& self, f : & mut std::fmt::Formatter)
        -> std::fmt::Result { std::fmt::Debug::fmt(& self.0, f) } }
    };
}
macro_rules! dispatchable_handle_drop {
    ($name:ident) => {
        #[repr(transparent)] #[derive(Eq, PartialEq, Ord, PartialOrd, Hash)] pub struct
        $name (pub * mut ()); impl $name { #[doc =
        " Creates a copy of this pointer, this is unsafe because the type implements Drop"]
        #[doc =
        " which runs the ffi destroy function, care must be taken that the type is not duplicated."]
        pub unsafe fn unsafe_copy(& self) -> std::mem::ManuallyDrop < Self > {
        std::mem::ManuallyDrop::new(Self(self.0)) } } unsafe impl Send for $name {}
        unsafe impl Sync for $name {} impl std::fmt::Pointer for $name { fn fmt(& self, f
        : & mut std::fmt::Formatter) -> std::fmt::Result { std::fmt::Pointer::fmt(& self
        .0, f) } } impl std::fmt::Debug for $name { fn fmt(& self, f : & mut
        std::fmt::Formatter) -> std::fmt::Result { std::fmt::Debug::fmt(& self.0, f) } }
    };
}
dispatchable_handle_eh! {
    Allocator
}
dispatchable_handle! {
    Pool
}
dispatchable_handle! {
    Allocation
}
dispatchable_handle! {
    DefragmentationContext
}
dispatchable_handle_drop! {
    VirtualBlock
}
non_dispatchable_handle! {
    VirtualAllocation
}
non_dispatchable_handle! {
    AllocHandle
}
impl Default for AllocatorCreateInfo {
    fn default() -> Self {
        Self {
            flags: AllocatorCreateFlags::empty(),
            physical_device: vk::PhysicalDevice::null(),
            device: vk::Device::null(),
            preferred_large_heap_block_size: 0,
            allocation_callbacks: std::ptr::null(),
            device_memory_callbacks: std::ptr::null(),
            heap_size_limit: std::ptr::null(),
            vulkan_functions: std::ptr::null(),
            instance: vk::Instance::null(),
            vulkan_api_version: 0,
            type_external_memory_handle_types: std::ptr::null(),
        }
    }
}
impl Default for AllocationCreateInfo {
    fn default() -> Self {
        Self {
            flags: AllocationCreateFlags::empty(),
            usage: MemoryUsage::Unknown,
            required_flags: vk::MemoryPropertyFlags::empty(),
            preferred_flags: vk::MemoryPropertyFlags::empty(),
            memory_type_bits: 0,
            pool: Pool::null(),
            user_data: std::ptr::null_mut(),
            priority: 0.5,
        }
    }
}
impl Default for PoolCreateInfo {
    fn default() -> Self {
        Self {
            memory_type_index: 0,
            block_size: 0,
            max_block_count: 0,
            flags: PoolCreateFlags::empty(),
            min_block_count: 0,
            priority: 0.5,
            min_allocation_alignment: 0,
            memory_allocate_next: std::ptr::null_mut(),
        }
    }
}
impl Default for VirtualBlockCreateInfo {
    fn default() -> Self {
        Self {
            size: 0,
            flags: VirtualBlockCreateFlags::empty(),
            allocation_callbacks: std::ptr::null(),
        }
    }
}
impl Default for VirtualAllocationCreateInfo {
    fn default() -> Self {
        Self {
            size: 0,
            alignment: 0,
            flags: VirtualAllocationCreateFlags::empty(),
            user_data: std::ptr::null_mut(),
        }
    }
}
/// Flags for created`VmaAllocator.`
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AllocatorCreateFlags(pub u32);
impl AllocatorCreateFlags {
    /// Allocator and all objects created from it will not be synchronized internally, so you must guarantee they are used from only one thread at a time or synchronized externally by you.
    ///
    /// Using this flag may increase performance because internal mutexes are not used.
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(1);
    /// Enables usage of VK_KHR_dedicated_allocation extension.
    ///
    /// The flag works only if VmaAllocatorCreateInfo::vulkanApiVersion `== VK_API_VERSION_1_0`.
    /// When it is `VK_API_VERSION_1_1`, the flag is ignored because the extension has been promoted to Vulkan 1.1.
    ///
    /// Using this extension will automatically allocate dedicated blocks of memory for
    /// some buffers and images instead of suballocating place for them out of bigger
    /// memory blocks (as if you explicitly used`VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT`
    /// flag) when it is recommended by the driver. It may improve performance on some
    /// GPUs.
    ///
    /// You may set this flag only if you found out that following device extensions are
    /// supported, you enabled them while creating Vulkan device passed as
    /// VmaAllocatorCreateInfo::device, and you want them to be used internally by this
    /// library:
    ///
    /// - VK_KHR_get_memory_requirements2 (device extension)
    /// - VK_KHR_dedicated_allocation (device extension)
    ///
    /// When this flag is set, you can experience following warnings reported by Vulkan
    /// validation layer. You can ignore them.
    ///
    /// > vkBindBufferMemory(): Binding memory to buffer 0x2d but vkGetBufferMemoryRequirements() has not been called on that buffer.
    pub const KHR_DEDICATED_ALLOCATION: Self = Self(2);
    /// Enables usage of VK_KHR_bind_memory2 extension.
    ///
    /// The flag works only if VmaAllocatorCreateInfo::vulkanApiVersion `== VK_API_VERSION_1_0`.
    /// When it is `VK_API_VERSION_1_1`, the flag is ignored because the extension has been promoted to Vulkan 1.1.
    ///
    /// You may set this flag only if you found out that this device extension is supported,
    /// you enabled it while creating Vulkan device passed as VmaAllocatorCreateInfo::device,
    /// and you want it to be used internally by this library.
    ///
    /// The extension provides functions `vkBindBufferMemory2KHR` and `vkBindImageMemory2KHR`,
    /// which allow to pass a chain of `pNext` structures while binding.
    /// This flag is required if you use `pNext` parameter in vmaBindBufferMemory2() or vmaBindImageMemory2().
    pub const KHR_BIND_MEMORY_2: Self = Self(4);
    /// Enables usage of VK_EXT_memory_budget extension.
    ///
    /// You may set this flag only if you found out that this device extension is supported,
    /// you enabled it while creating Vulkan device passed as VmaAllocatorCreateInfo::device,
    /// and you want it to be used internally by this library, along with another instance extension
    /// VK_KHR_get_physical_device_properties2, which is required by it (or Vulkan 1.1, where this extension is promoted).
    ///
    /// The extension provides query for current memory usage and budget, which will probably
    /// be more accurate than an estimation used by the library otherwise.
    pub const EXT_MEMORY_BUDGET: Self = Self(8);
    /// Enables usage of VK_AMD_device_coherent_memory extension.
    ///
    /// You may set this flag only if you:
    ///
    /// - found out that this device extension is supported and enabled it while creating Vulkan device passed as VmaAllocatorCreateInfo::device,
    /// - checked that `VkPhysicalDeviceCoherentMemoryFeaturesAMD::deviceCoherentMemory` is true and set it while creating the Vulkan device,
    /// - want it to be used internally by this library.
    ///
    /// The extension and accompanying device feature provide access to memory types with
    /// `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD` and `VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD` flags.
    /// They are useful mostly for writing breadcrumb markers - a common method for debugging`GPU` crash/hang/TDR.
    ///
    /// When the extension is not enabled, such memory types are still enumerated, but their usage is illegal.
    /// To protect from this error, if you don't create the allocator with this flag, it will refuse to allocate any memory or create a custom pool in such memory type,
    /// returning `VK_ERROR_FEATURE_NOT_PRESENT`.
    pub const AMD_DEVICE_COHERENT_MEMORY: Self = Self(16);
    /// Enables usage of "buffer device address" feature, which allows you to use function
    /// `vkGetBufferDeviceAddress*` to get raw`GPU` pointer to a buffer and pass it for usage inside a shader.
    ///
    /// You may set this flag only if you:
    ///
    /// 1. (For Vulkan version < 1.2) Found as available and enabled device extension
    /// VK_KHR_buffer_device_address.
    /// This extension is promoted to core Vulkan 1.2.
    /// 2. Found as available and enabled device feature `VkPhysicalDeviceBufferDeviceAddressFeatures::bufferDeviceAddress`.
    ///
    /// When this flag is set, you can create buffers with `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` using VMA.
    /// The library automatically adds `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT` to
    /// allocated memory blocks wherever it might be needed.
    ///
    /// For more information, see documentation chapter enabling_buffer_device_address.
    pub const BUFFER_DEVICE_ADDRESS: Self = Self(32);
    /// Enables usage of VK_EXT_memory_priority extension in the library.
    ///
    /// You may set this flag only if you found available and enabled this device extension,
    /// along with `VkPhysicalDeviceMemoryPriorityFeaturesEXT::memoryPriority == VK_TRUE`,
    /// while creating Vulkan device passed as VmaAllocatorCreateInfo::device.
    ///
    /// When this flag is used, VmaAllocationCreateInfo::priority and VmaPoolCreateInfo::priority
    /// are used to set priorities of allocated Vulkan memory. Without it, these variables are ignored.
    ///
    /// A priority must be a floating-point value between 0 and 1, indicating the priority of the allocation relative to other memory allocations.
    /// Larger values are higher priority. The granularity of the priorities is implementation-dependent.
    /// It is automatically passed to every call to `vkAllocateMemory` done by the library using structure `VkMemoryPriorityAllocateInfoEXT`.
    /// The value to be used for default priority is 0.5.
    /// For more details, see the documentation of the VK_EXT_memory_priority extension.
    pub const EXT_MEMORY_PRIORITY: Self = Self(64);
}
pumice::bitflags_impl! {
    AllocatorCreateFlags : u32, 0x7f, EXTERNALLY_SYNCHRONIZED, KHR_DEDICATED_ALLOCATION,
    KHR_BIND_MEMORY_2, EXT_MEMORY_BUDGET, AMD_DEVICE_COHERENT_MEMORY,
    BUFFER_DEVICE_ADDRESS, EXT_MEMORY_PRIORITY
}
/// Intended usage of the allocated memory.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u32)]
pub enum MemoryUsage {
    /// No intended memory usage specified.
    /// Use other members of VmaAllocationCreateInfo to specify your requirements.
    Unknown = 0,
    #[deprecated = "Obsolete, preserved for backward compatibility."]
    /// Prefers `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`.
    GpuOnly = 1,
    #[deprecated = "Obsolete, preserved for backward compatibility."]
    /// Guarantees `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT` and `VK_MEMORY_PROPERTY_HOST_COHERENT_BIT`.
    CpuOnly = 2,
    #[deprecated = "Obsolete, preserved for backward compatibility."]
    /// Guarantees `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT`, prefers `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`.
    CpuToGpu = 3,
    #[deprecated = "Obsolete, preserved for backward compatibility."]
    /// Guarantees `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT`, prefers `VK_MEMORY_PROPERTY_HOST_CACHED_BIT`.
    GpuToCpu = 4,
    #[deprecated = "Obsolete, preserved for backward compatibility."]
    /// Prefers not `VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT`.
    CpuCopy = 5,
    /// Lazily allocated`GPU` memory having `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`.
    /// Exists mostly on mobile platforms. Using it on desktop`PC` or other GPUs with no such memory type present will fail the allocation.
    ///
    /// Usage: Memory for transient attachment images (color attachments, depth attachments etc.), created with `VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT`.
    ///
    /// Allocations with this usage are always created as dedicated - it implies`VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT.`
    GpuLazilyAllocated = 6,
    /// Selects best memory type automatically.
    /// This flag is recommended for most common use cases.
    ///
    /// When using this flag, if you want to map the allocation (using vmaMapMemory() or`VMA_ALLOCATION_CREATE_MAPPED_BIT),`
    /// you must pass one of the flags:`VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT` or`VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT`
    /// in VmaAllocationCreateInfo::flags.
    ///
    /// It can be used only with functions that let the library know `VkBufferCreateInfo` or `VkImageCreateInfo`, e.g.
    /// vmaCreateBuffer(), vmaCreateImage(), vmaFindMemoryTypeIndexForBufferInfo(), vmaFindMemoryTypeIndexForImageInfo()
    /// and not with generic memory allocation functions.
    Auto = 7,
    /// Selects best memory type automatically with preference for`GPU` (device) memory.
    ///
    /// When using this flag, if you want to map the allocation (using vmaMapMemory() or`VMA_ALLOCATION_CREATE_MAPPED_BIT),`
    /// you must pass one of the flags:`VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT` or`VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT`
    /// in VmaAllocationCreateInfo::flags.
    ///
    /// It can be used only with functions that let the library know `VkBufferCreateInfo` or `VkImageCreateInfo`, e.g.
    /// vmaCreateBuffer(), vmaCreateImage(), vmaFindMemoryTypeIndexForBufferInfo(), vmaFindMemoryTypeIndexForImageInfo()
    /// and not with generic memory allocation functions.
    AutoPreferDevice = 8,
    /// Selects best memory type automatically with preference for`CPU` (host) memory.
    ///
    /// When using this flag, if you want to map the allocation (using vmaMapMemory() or`VMA_ALLOCATION_CREATE_MAPPED_BIT),`
    /// you must pass one of the flags:`VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT` or`VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT`
    /// in VmaAllocationCreateInfo::flags.
    ///
    /// It can be used only with functions that let the library know `VkBufferCreateInfo` or `VkImageCreateInfo`, e.g.
    /// vmaCreateBuffer(), vmaCreateImage(), vmaFindMemoryTypeIndexForBufferInfo(), vmaFindMemoryTypeIndexForImageInfo()
    /// and not with generic memory allocation functions.
    AutoPreferHost = 9,
}
/// Flags to be passed as VmaAllocationCreateInfo::flags.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct AllocationCreateFlags(pub u32);
impl AllocationCreateFlags {
    /// Set this flag if the allocation should have its own memory block.
    ///
    /// Use it for special, big resources, like fullscreen images used as attachments.
    pub const DEDICATED_MEMORY: Self = Self(1);
    /// Set this flag to only try to allocate from existing `VkDeviceMemory` blocks and never create new such block.
    ///
    /// If new allocation cannot be placed in any of the existing blocks, allocation
    /// fails with `VK_ERROR_OUT_OF_DEVICE_MEMORY` error.
    ///
    /// You should not use`VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT` and
    /// `VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT` at the same time. It makes no sense.
    pub const NEVER_ALLOCATE: Self = Self(2);
    /// Set this flag to use a memory that will be persistently mapped and retrieve pointer to it.
    ///
    /// Pointer to mapped memory will be returned through VmaAllocationInfo::pMappedData.
    ///
    /// It is valid to use this flag for allocation made from memory type that is not
    /// `HOST_VISIBLE`. This flag is then ignored and memory is not mapped. This is
    /// useful if you need an allocation that is efficient to use on`GPU`
    /// (`DEVICE_LOCAL`) and still want to map it directly if possible on platforms that
    /// support it (e.g. Intel GPU).
    pub const MAPPED: Self = Self(4);
    #[deprecated = "Preserved for backward compatibility. Consider using vmaSetAllocationName() instead."]
    /// Set this flag to treat VmaAllocationCreateInfo::pUserData as pointer to a
    /// null-terminated string. Instead of copying pointer value, a local copy of the
    /// string is made and stored in allocation's `pName`. The string is automatically
    /// freed together with the allocation. It is also used in vmaBuildStatsString().
    pub const USER_DATA_COPY_STRING: Self = Self(32);
    /// Allocation will be created from upper stack in a double stack pool.
    ///
    /// This flag is only allowed for custom pools created with`VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT` flag.
    pub const UPPER_ADDRESS: Self = Self(64);
    /// Create both buffer/image and allocation, but don't bind them together.
    /// It is useful when you want to bind yourself to do some more advanced binding, e.g. using some extensions.
    /// The flag is meaningful only with functions that bind by default: vmaCreateBuffer(), vmaCreateImage().
    /// Otherwise it is ignored.
    ///
    /// If you want to make sure the new buffer/image is not tied to the new memory allocation
    /// through `VkMemoryDedicatedAllocateInfoKHR` structure in case the allocation ends up in its own memory block,
    /// use also flag`VMA_ALLOCATION_CREATE_CAN_ALIAS_BIT.`
    pub const DONT_BIND: Self = Self(128);
    /// Create allocation only if additional device memory required for it, if any, won't exceed
    /// memory budget. Otherwise return `VK_ERROR_OUT_OF_DEVICE_MEMORY`.
    pub const WITHIN_BUDGET: Self = Self(256);
    /// Set this flag if the allocated memory will have aliasing resources.
    ///
    /// Usage of this flag prevents supplying `VkMemoryDedicatedAllocateInfoKHR` when`VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT` is specified.
    /// Otherwise created dedicated memory will not be suitable for aliasing resources, resulting in Vulkan Validation Layer errors.
    pub const CAN_ALIAS: Self = Self(512);
    /// Requests possibility to map the allocation (using vmaMapMemory() or`VMA_ALLOCATION_CREATE_MAPPED_BIT).`
    ///
    /// - If you use`VMA_MEMORY_USAGE_AUTO` or other `VMA_MEMORY_USAGE_AUTO*` value,
    /// you must use this flag to be able to map the allocation. Otherwise, mapping is incorrect.
    /// - If you use other value of`VmaMemoryUsage,` this flag is ignored and mapping is always possible in memory types that are `HOST_VISIBLE`.
    /// This includes allocations created in custom_memory_pools.
    ///
    /// Declares that mapped memory will only be written sequentially, e.g. using `memcpy()` or a loop writing number-by-number,
    /// never read or accessed randomly, so a memory type can be selected that is uncached and write-combined.
    ///
    /// **Violating this declaration may work correctly, but will likely be very slow.
    /// Watch out for implicit reads introduced by doing e.g. `pMappedData[i] += x;`
    /// Better prepare your data in a local variable and `memcpy()` it to the mapped pointer all at once.**
    pub const HOST_ACCESS_SEQUENTIAL_WRITE: Self = Self(1024);
    /// Requests possibility to map the allocation (using vmaMapMemory() or`VMA_ALLOCATION_CREATE_MAPPED_BIT).`
    ///
    /// - If you use`VMA_MEMORY_USAGE_AUTO` or other `VMA_MEMORY_USAGE_AUTO*` value,
    /// you must use this flag to be able to map the allocation. Otherwise, mapping is incorrect.
    /// - If you use other value of`VmaMemoryUsage,` this flag is ignored and mapping is always possible in memory types that are `HOST_VISIBLE`.
    /// This includes allocations created in custom_memory_pools.
    ///
    /// Declares that mapped memory can be read, written, and accessed in random order,
    /// so a `HOST_CACHED` memory type is required.
    pub const HOST_ACCESS_RANDOM: Self = Self(2048);
    /// Together with`VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT` or`VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT,`
    /// it says that despite request for host access, a not-`HOST_VISIBLE` memory type can be selected
    /// if it may improve performance.
    ///
    /// By using this flag, you declare that you will check if the allocation ended up in a `HOST_VISIBLE` memory type
    /// (e.g. using vmaGetAllocationMemoryProperties()) and if not, you will create some "staging" buffer and
    /// issue an explicit transfer to write/read your data.
    /// To prepare for this possibility, don't forget to add appropriate flags like
    /// `VK_BUFFER_USAGE_TRANSFER_DST_BIT`, `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` to the parameters of created buffer or image.
    pub const HOST_ACCESS_ALLOW_TRANSFER_INSTEAD: Self = Self(4096);
    /// Allocation strategy that chooses smallest possible free range for the allocation
    /// to minimize memory usage and fragmentation, possibly at the expense of allocation time.
    pub const STRATEGY_MIN_MEMORY: Self = Self(65536);
    /// Allocation strategy that chooses first suitable free range for the allocation -
    /// not necessarily in terms of the smallest offset but the one that is easiest and fastest to find
    /// to minimize allocation time, possibly at the expense of allocation quality.
    pub const STRATEGY_MIN_TIME: Self = Self(131072);
    /// Allocation strategy that chooses always the lowest offset in available space.
    /// This is not the most efficient strategy but achieves highly packed data.
    /// Used internally by defragmentation, not recommended in typical usage.
    pub const STRATEGY_MIN_OFFSET: Self = Self(262144);
    /// Alias to`VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT.`
    pub const STRATEGY_BEST_FIT: Self = Self(65536);
    /// Alias to`VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT.`
    pub const STRATEGY_FIRST_FIT: Self = Self(131072);
    /// A bit mask to extract only `STRATEGY` bits from entire set of flags.
    pub const STRATEGY_MASK: Self = Self(458752);
}
pumice::bitflags_impl! {
    AllocationCreateFlags : u32, 0x71fe7, DEDICATED_MEMORY, NEVER_ALLOCATE, MAPPED,
    USER_DATA_COPY_STRING, UPPER_ADDRESS, DONT_BIND, WITHIN_BUDGET, CAN_ALIAS,
    HOST_ACCESS_SEQUENTIAL_WRITE, HOST_ACCESS_RANDOM, HOST_ACCESS_ALLOW_TRANSFER_INSTEAD,
    STRATEGY_MIN_MEMORY, STRATEGY_MIN_TIME, STRATEGY_MIN_OFFSET, STRATEGY_BEST_FIT,
    STRATEGY_FIRST_FIT, STRATEGY_MASK
}
/// Flags to be passed as VmaPoolCreateInfo::flags.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PoolCreateFlags(pub u32);
impl PoolCreateFlags {
    /// Use this flag if you always allocate only buffers and linear images or only optimal images out of this pool and so Buffer-Image Granularity can be ignored.
    ///
    /// This is an optional optimization flag.
    ///
    /// If you always allocate using vmaCreateBuffer(), vmaCreateImage(),
    /// vmaAllocateMemoryForBuffer(), then you don't need to use it because allocator
    /// knows exact type of your allocations so it can handle Buffer-Image Granularity
    /// in the optimal way.
    ///
    /// If you also allocate using vmaAllocateMemoryForImage() or vmaAllocateMemory(),
    /// exact type of such allocations is not known, so allocator must be conservative
    /// in handling Buffer-Image Granularity, which can lead to suboptimal allocation
    /// (wasted memory). In that case, if you can make sure you always allocate only
    /// buffers and linear images or only optimal images out of this pool, use this flag
    /// to make allocator disregard Buffer-Image Granularity and so make allocations
    /// faster and more optimal.
    pub const IGNORE_BUFFER_IMAGE_GRANULARITY: Self = Self(2);
    /// Enables alternative, linear allocation algorithm in this pool.
    ///
    /// Specify this flag to enable linear allocation algorithm, which always creates
    /// new allocations after last one and doesn't reuse space from allocations freed in
    /// between. It trades memory consumption for simplified algorithm and data
    /// structure, which has better performance and uses less memory for metadata.
    ///
    /// By using this flag, you can achieve behavior of free-at-once, stack,
    /// ring buffer, and double stack.
    /// For details, see documentation chapter linear_algorithm.
    pub const LINEAR_ALGORITHM: Self = Self(4);
    /// Bit mask to extract only `ALGORITHM` bits from entire set of flags.
    pub const ALGORITHM_MASK: Self = Self(4);
}
pumice::bitflags_impl! {
    PoolCreateFlags : u32, 0x6, IGNORE_BUFFER_IMAGE_GRANULARITY, LINEAR_ALGORITHM,
    ALGORITHM_MASK
}
/// Flags to be passed as VmaDefragmentationInfo::flags.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DefragmentationFlags(pub u32);
impl DefragmentationFlags {
    pub const ALGORITHM_FAST: Self = Self(1);
    pub const ALGORITHM_BALANCED: Self = Self(2);
    pub const ALGORITHM_FULL: Self = Self(4);
    /// Use the most roboust algorithm at the cost of time to compute and number of copies to make.
    /// Only available when bufferImageGranularity is greater than 1, since it aims to reduce
    /// alignment issues between different types of resources.
    /// Otherwise falls back to same behavior as`VMA_DEFRAGMENTATION_FLAG_ALGORITHM_FULL_BIT.`
    pub const ALGORITHM_EXTENSIVE: Self = Self(8);
    /// A bit mask to extract only `ALGORITHM` bits from entire set of flags.
    pub const ALGORITHM_MASK: Self = Self(15);
}
pumice::bitflags_impl! {
    DefragmentationFlags : u32, 0xf, ALGORITHM_FAST, ALGORITHM_BALANCED, ALGORITHM_FULL,
    ALGORITHM_EXTENSIVE, ALGORITHM_MASK
}
/// Operation performed on single defragmentation move. See structure`VmaDefragmentationMove.`
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u32)]
pub enum DefragmentationMoveOperation {
    /// Buffer/image has been recreated at `dstTmpAllocation`, data has been copied, old buffer/image has been destroyed. `srcAllocation` should be changed to point to the new place. This is the default value set by vmaBeginDefragmentationPass().
    VMA_DEFRAGMENTATION_MOVE_OPERATION_COPY = 0,
    /// Set this value if you cannot move the allocation. New place reserved at `dstTmpAllocation` will be freed. `srcAllocation` will remain unchanged.
    VMA_DEFRAGMENTATION_MOVE_OPERATION_IGNORE = 1,
    /// Set this value if you decide to abandon the allocation and you destroyed the buffer/image. New place reserved at `dstTmpAllocation` will be freed, along with `srcAllocation`, which will be destroyed.
    VMA_DEFRAGMENTATION_MOVE_OPERATION_DESTROY = 2,
}
/// Flags to be passed as VmaVirtualBlockCreateInfo::flags.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VirtualBlockCreateFlags(pub u32);
impl VirtualBlockCreateFlags {
    /// Enables alternative, linear allocation algorithm in this virtual block.
    ///
    /// Specify this flag to enable linear allocation algorithm, which always creates
    /// new allocations after last one and doesn't reuse space from allocations freed in
    /// between. It trades memory consumption for simplified algorithm and data
    /// structure, which has better performance and uses less memory for metadata.
    ///
    /// By using this flag, you can achieve behavior of free-at-once, stack,
    /// ring buffer, and double stack.
    /// For details, see documentation chapter linear_algorithm.
    pub const LINEAR_ALGORITHM: Self = Self(1);
    /// Bit mask to extract only `ALGORITHM` bits from entire set of flags.
    pub const ALGORITHM_MASK: Self = Self(1);
}
pumice::bitflags_impl! {
    VirtualBlockCreateFlags : u32, 0x1, LINEAR_ALGORITHM, ALGORITHM_MASK
}
/// Flags to be passed as VmaVirtualAllocationCreateInfo::flags.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct VirtualAllocationCreateFlags(pub u32);
impl VirtualAllocationCreateFlags {
    /// Allocation will be created from upper stack in a double stack pool.
    ///
    /// This flag is only allowed for virtual blocks created with`VMA_VIRTUAL_BLOCK_CREATE_LINEAR_ALGORITHM_BIT` flag.
    pub const UPPER_ADDRESS: Self = Self(64);
    /// Allocation strategy that tries to minimize memory usage.
    pub const STRATEGY_MIN_MEMORY: Self = Self(65536);
    /// Allocation strategy that tries to minimize allocation time.
    pub const STRATEGY_MIN_TIME: Self = Self(131072);
    /// Allocation strategy that chooses always the lowest offset in available space.
    /// This is not the most efficient strategy but achieves highly packed data.
    pub const STRATEGY_MIN_OFFSET: Self = Self(262144);
    /// A bit mask to extract only `STRATEGY` bits from entire set of flags.
    ///
    /// These strategy flags are binary compatible with equivalent flags in`VmaAllocationCreateFlagBits.`
    pub const STRATEGY_MASK: Self = Self(458752);
}
pumice::bitflags_impl! {
    VirtualAllocationCreateFlags : u32, 0x70040, UPPER_ADDRESS, STRATEGY_MIN_MEMORY,
    STRATEGY_MIN_TIME, STRATEGY_MIN_OFFSET, STRATEGY_MASK
}
/// Set of callbacks that the library will call for `vkAllocateMemory` and `vkFreeMemory`.
///
/// Provided for informative purpose, e.g. to gather statistics about number of
/// allocations or total amount of memory allocated in Vulkan.
///
/// Used in VmaAllocatorCreateInfo::pDeviceMemoryCallbacks.
#[derive(Clone)]
#[repr(C)]
pub struct DeviceMemoryCallbacks {
    /// Optional, can be null.
    pub pfnAllocate: Option<PFN_vmaAllocateDeviceMemoryFunction>,
    /// Optional, can be null.
    pub pfnFree: Option<PFN_vmaFreeDeviceMemoryFunction>,
    /// Optional, can be null.
    pub pUserData: *mut std::ffi::c_void,
}
/// Pointers to some Vulkan functions - a subset used by the library.
///
/// Used in VmaAllocatorCreateInfo::pVulkanFunctions.
#[derive(Clone)]
#[repr(C)]
pub struct VulkanFunctions {
    /// Required when using VMA_DYNAMIC_VULKAN_FUNCTIONS.
    pub vkGetInstanceProcAddr: Option<PFN_vkGetInstanceProcAddr>,
    /// Required when using VMA_DYNAMIC_VULKAN_FUNCTIONS.
    pub vkGetDeviceProcAddr: Option<PFN_vkGetDeviceProcAddr>,
    pub vkGetPhysicalDeviceProperties: Option<PFN_vkGetPhysicalDeviceProperties>,
    pub vkGetPhysicalDeviceMemoryProperties: Option<
        PFN_vkGetPhysicalDeviceMemoryProperties,
    >,
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
    /// Fetch "vkGetBufferMemoryRequirements2" on Vulkan >= 1.1, fetch "vkGetBufferMemoryRequirements2KHR" when using VK_KHR_dedicated_allocation extension.
    pub vkGetBufferMemoryRequirements2KHR: Option<PFN_vkGetBufferMemoryRequirements2KHR>,
    /// Fetch "vkGetImageMemoryRequirements2" on Vulkan >= 1.1, fetch "vkGetImageMemoryRequirements2KHR" when using VK_KHR_dedicated_allocation extension.
    pub vkGetImageMemoryRequirements2KHR: Option<PFN_vkGetImageMemoryRequirements2KHR>,
    /// Fetch "vkBindBufferMemory2" on Vulkan >= 1.1, fetch "vkBindBufferMemory2KHR" when using VK_KHR_bind_memory2 extension.
    pub vkBindBufferMemory2KHR: Option<PFN_vkBindBufferMemory2KHR>,
    /// Fetch "vkBindImageMemory2" on Vulkan >= 1.1, fetch "vkBindImageMemory2KHR" when using VK_KHR_bind_memory2 extension.
    pub vkBindImageMemory2KHR: Option<PFN_vkBindImageMemory2KHR>,
    pub vkGetPhysicalDeviceMemoryProperties2KHR: Option<
        PFN_vkGetPhysicalDeviceMemoryProperties2KHR,
    >,
    /// Fetch from "vkGetDeviceBufferMemoryRequirements" on Vulkan >= 1.3, but you can also fetch it from "vkGetDeviceBufferMemoryRequirementsKHR" if you enabled extension VK_KHR_maintenance4.
    pub vkGetDeviceBufferMemoryRequirements: Option<
        PFN_vkGetDeviceBufferMemoryRequirements,
    >,
    /// Fetch from "vkGetDeviceImageMemoryRequirements" on Vulkan >= 1.3, but you can also fetch it from "vkGetDeviceImageMemoryRequirementsKHR" if you enabled extension VK_KHR_maintenance4.
    pub vkGetDeviceImageMemoryRequirements: Option<
        PFN_vkGetDeviceImageMemoryRequirements,
    >,
}
/// Description of a Allocator to be created.
#[derive(Clone)]
#[repr(C)]
pub struct AllocatorCreateInfo {
    /// Flags for created allocator. Use`VmaAllocatorCreateFlagBits` enum.
    pub flags: AllocatorCreateFlags,
    /// Vulkan physical device.
    /// It must be valid throughout whole lifetime of created allocator.
    pub physical_device: vk::PhysicalDevice,
    /// Vulkan device.
    /// It must be valid throughout whole lifetime of created allocator.
    pub device: vk::Device,
    /// Preferred size of a single `VkDeviceMemory` block to be allocated from large heaps > 1 GiB. Optional.
    /// Set to 0 to use default, which is currently 256 MiB.
    pub preferred_large_heap_block_size: u64,
    /// Custom`CPU` memory allocation callbacks. Optional.
    /// Optional, can be null. When specified, will also be used for all CPU-side memory allocations.
    pub allocation_callbacks: *const vk::AllocationCallbacks,
    /// Informative callbacks for `vkAllocateMemory`, `vkFreeMemory`. Optional.
    /// Optional, can be null.
    pub device_memory_callbacks: *const DeviceMemoryCallbacks,
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
    /// Warning! Using this feature may not be equivalent to installing a`GPU` with
    /// smaller amount of memory, because graphics driver doesn't necessary fail new
    /// allocations with `VK_ERROR_OUT_OF_DEVICE_MEMORY` result when memory capacity is
    /// exceeded. It may return success and just silently migrate some device memory
    /// blocks to system RAM. This driver behavior can also be controlled using
    /// VK_AMD_memory_overallocation_behavior extension.
    pub heap_size_limit: *const u64,
    /// Pointers to Vulkan functions. Can be null.
    ///
    /// For details see [Pointers to Vulkan functions](@ref config_Vulkan_functions).
    pub vulkan_functions: *const VulkanFunctions,
    /// Handle to Vulkan instance object.
    ///
    /// Starting from version 3.0.0 this member is no longer optional, it must be set!
    pub instance: vk::Instance,
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
    pub type_external_memory_handle_types: *const ExternalMemoryHandleTypeFlags,
}
/// Information about existing`VmaAllocator` object.
#[derive(Clone)]
#[repr(C)]
pub struct AllocatorInfo {
    /// Handle to Vulkan instance object.
    ///
    /// This is the same value as has been passed through VmaAllocatorCreateInfo::instance.
    pub instance: vk::Instance,
    /// Handle to Vulkan physical device object.
    ///
    /// This is the same value as has been passed through VmaAllocatorCreateInfo::physicalDevice.
    pub physical_device: vk::PhysicalDevice,
    /// Handle to Vulkan device object.
    ///
    /// This is the same value as has been passed through VmaAllocatorCreateInfo::device.
    pub device: vk::Device,
}
/// Calculated statistics of memory usage e.g. in a specific memory type, heap, custom pool, or total.
///
/// These are fast to calculate.
/// See functions: vmaGetHeapBudgets(), vmaGetPoolStatistics().
#[derive(Clone)]
#[repr(C)]
pub struct Statistics {
    /// Number of `VkDeviceMemory` objects - Vulkan memory blocks allocated.
    pub block_count: u32,
    /// Number of`VmaAllocation` objects allocated.
    ///
    /// Dedicated allocations have their own blocks, so each one adds 1 to `allocationCount` as well as `blockCount`.
    pub allocation_count: u32,
    /// Number of bytes allocated in `VkDeviceMemory` blocks.
    ///
    /// To avoid confusion, please be aware that what Vulkan calls an "allocation" - a whole `VkDeviceMemory` object
    /// (e.g. as in `VkPhysicalDeviceLimits::maxMemoryAllocationCount`) is called a "block" in VMA, while`VMA` calls
    /// "allocation" a`VmaAllocation` object that represents a memory region sub-allocated from such block, usually for a single buffer or image.
    pub block_bytes: u64,
    /// Total number of bytes occupied by all`VmaAllocation` objects.
    ///
    /// Always less or equal than `blockBytes`.
    /// Difference `(blockBytes - allocationBytes)` is the amount of memory allocated from Vulkan
    /// but unused by any`VmaAllocation.`
    pub allocation_bytes: u64,
}
/// More detailed statistics than`VmaStatistics.`
///
/// These are slower to calculate. Use for debugging purposes.
/// See functions: vmaCalculateStatistics(), vmaCalculatePoolStatistics().
///
/// Previous version of the statistics`API` provided averages, but they have been removed
/// because they can be easily calculated as:
///
/// ```ignore
/// VkDeviceSize allocationSizeAvg = detailedStats.statistics.allocationBytes / detailedStats.statistics.allocationCount;
/// VkDeviceSize unusedBytes = detailedStats.statistics.blockBytes - detailedStats.statistics.allocationBytes;
/// VkDeviceSize unusedRangeSizeAvg = unusedBytes / detailedStats.unusedRangeCount;
/// ```
#[derive(Clone)]
#[repr(C)]
pub struct DetailedStatistics {
    /// Basic statistics.
    pub statistics: Statistics,
    /// Number of free ranges of memory between allocations.
    pub unused_range_count: u32,
    /// Smallest allocation size. `VK_WHOLE_SIZE` if there are 0 allocations.
    pub allocation_size_min: u64,
    /// Largest allocation size. 0 if there are 0 allocations.
    pub allocation_size_max: u64,
    /// Smallest empty range size. `VK_WHOLE_SIZE` if there are 0 empty ranges.
    pub unused_range_size_min: u64,
    /// Largest empty range size. 0 if there are 0 empty ranges.
    pub unused_range_size_max: u64,
}
/// General statistics from current state of the Allocator -
/// total memory usage across all memory heaps and types.
///
/// These are slower to calculate. Use for debugging purposes.
/// See function vmaCalculateStatistics().
#[derive(Clone)]
#[repr(C)]
pub struct TotalStatistics {
    pub memory_type: [DetailedStatistics; 32],
    pub memory_heap: [DetailedStatistics; 16],
    pub total: DetailedStatistics,
}
/// Statistics of current memory usage and available budget for a specific memory heap.
///
/// These are fast to calculate.
/// See function vmaGetHeapBudgets().
#[derive(Clone)]
#[repr(C)]
pub struct Budget {
    /// Statistics fetched from the library.
    pub statistics: Statistics,
    /// Estimated current memory usage of the program, in bytes.
    ///
    /// Fetched from system using VK_EXT_memory_budget extension if enabled.
    ///
    /// It might be different than `statistics.blockBytes` (usually higher) due to additional implicit objects
    /// also occupying the memory, like swapchain, pipelines, descriptor heaps, command buffers, or
    /// `VkDeviceMemory` blocks allocated outside of this library, if any.
    pub usage: u64,
    /// Estimated amount of memory available to the program, in bytes.
    ///
    /// Fetched from system using VK_EXT_memory_budget extension if enabled.
    ///
    /// It might be different (most probably smaller) than `VkMemoryHeap::size[heapIndex]` due to factors
    /// external to the program, decided by the operating system.
    /// Difference `budget - usage` is the amount of additional memory that can probably
    /// be allocated without problems. Exceeding the budget may result in various problems.
    pub budget: u64,
}
/// Parameters of new`VmaAllocation.`
///
/// To be used with functions like vmaCreateBuffer(), vmaCreateImage(), and many others.
#[derive(Clone)]
#[repr(C)]
pub struct AllocationCreateInfo {
    /// Use`VmaAllocationCreateFlagBits` enum.
    pub flags: AllocationCreateFlags,
    /// Intended usage of memory.
    ///
    /// You can leave`VMA_MEMORY_USAGE_UNKNOWN` if you specify memory requirements in other way.
    ///
    /// If `pool` is not null, this member is ignored.
    pub usage: MemoryUsage,
    /// Flags that must be set in a Memory Type chosen for an allocation.
    ///
    /// Leave 0 if you specify memory requirements in other way.
    ///
    /// If `pool` is not null, this member is ignored.
    pub required_flags: vk::MemoryPropertyFlags,
    /// Flags that preferably should be set in a memory type chosen for an allocation.
    ///
    /// Set to 0 if no additional flags are preferred.
    ///
    /// If `pool` is not null, this member is ignored.
    pub preferred_flags: vk::MemoryPropertyFlags,
    /// Bitmask containing one bit set for every memory type acceptable for this allocation.
    ///
    /// Value 0 is equivalent to `UINT32_MAX` - it means any memory type is accepted if
    /// it meets other requirements specified by this structure, with no further
    /// restrictions on memory type index.
    ///
    /// If `pool` is not null, this member is ignored.
    pub memory_type_bits: u32,
    /// Pool that this allocation should be created in.
    ///
    /// Leave `VK_NULL_HANDLE` to allocate from default pool. If not null, members:
    /// `usage`, `requiredFlags`, `preferredFlags`, `memoryTypeBits` are ignored.
    pub pool: Pool,
    /// Custom general-purpose pointer that will be stored in`VmaAllocation,` can be read as VmaAllocationInfo::pUserData and changed using vmaSetAllocationUserData().
    ///
    /// If`VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT` is used, it must be either
    /// null or pointer to a null-terminated string. The string will be then copied to
    /// internal buffer, so it doesn't need to be valid after allocation call.
    pub user_data: *mut std::ffi::c_void,
    /// A floating-point value between 0 and 1, indicating the priority of the allocation relative to other memory allocations.
    ///
    /// It is used only when`VMA_ALLOCATOR_CREATE_EXT_MEMORY_PRIORITY_BIT` flag was used during creation of the`VmaAllocator` object
    /// and this allocation ends up as dedicated or is explicitly forced as dedicated using`VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT.`
    /// Otherwise, it has the priority of a memory block where it is placed and this variable is ignored.
    pub priority: f32,
}
/// Describes parameter of created`VmaPool.`
#[derive(Clone)]
#[repr(C)]
pub struct PoolCreateInfo {
    /// Vulkan memory type index to allocate this pool from.
    pub memory_type_index: u32,
    /// Use combination of`VmaPoolCreateFlagBits.`
    pub flags: PoolCreateFlags,
    /// Size of a single `VkDeviceMemory` block to be allocated as part of this pool, in bytes. Optional.
    ///
    /// Specify nonzero to set explicit, constant size of memory blocks used by this
    /// pool.
    ///
    /// Leave 0 to use default and let the library manage block sizes automatically.
    /// Sizes of particular blocks may vary.
    /// In this case, the pool will also support dedicated allocations.
    pub block_size: u64,
    /// Minimum number of blocks to be always allocated in this pool, even if they stay empty.
    ///
    /// Set to 0 to have no preallocated blocks and allow the pool be completely empty.
    pub min_block_count: usize,
    /// Maximum number of blocks that can be allocated in this pool. Optional.
    ///
    /// Set to 0 to use default, which is `SIZE_MAX`, which means no limit.
    ///
    /// Set to same value as VmaPoolCreateInfo::minBlockCount to have fixed amount of memory allocated
    /// throughout whole lifetime of this pool.
    pub max_block_count: usize,
    /// A floating-point value between 0 and 1, indicating the priority of the allocations in this pool relative to other memory allocations.
    ///
    /// It is used only when`VMA_ALLOCATOR_CREATE_EXT_MEMORY_PRIORITY_BIT` flag was used during creation of the`VmaAllocator` object.
    /// Otherwise, this variable is ignored.
    pub priority: f32,
    /// Additional minimum alignment to be used for all allocations created from this pool. Can be 0.
    ///
    /// Leave 0 (default) not to impose any additional alignment. If not 0, it must be a power of two.
    /// It can be useful in cases where alignment returned by Vulkan by functions like `vkGetBufferMemoryRequirements` is not enough,
    /// e.g. when doing interop with OpenGL.
    pub min_allocation_alignment: u64,
    /// Additional `pNext` chain to be attached to `VkMemoryAllocateInfo` used for every allocation made by this pool. Optional.
    ///
    /// Optional, can be null. If not null, it must point to a `pNext` chain of structures that can be attached to `VkMemoryAllocateInfo`.
    /// It can be useful for special needs such as adding `VkExportMemoryAllocateInfoKHR`.
    /// Structures pointed by this member must remain alive and unchanged for the whole lifetime of the custom pool.
    ///
    /// Please note that some structures, e.g. `VkMemoryPriorityAllocateInfoEXT`, `VkMemoryDedicatedAllocateInfoKHR`,
    /// can be attached automatically by this library when using other, more convenient of its features.
    pub memory_allocate_next: *mut std::ffi::c_void,
}
/// Parameters of`VmaAllocation` objects, that can be retrieved using function vmaGetAllocationInfo().
#[derive(Clone)]
#[repr(C)]
pub struct AllocationInfo {
    /// Memory type index that this allocation was allocated from.
    ///
    /// It never changes.
    pub memory_type: u32,
    /// Handle to Vulkan memory object.
    ///
    /// Same memory object can be shared by multiple allocations.
    ///
    /// It can change after the allocation is moved during defragmentation.
    pub device_memory: vk::DeviceMemory,
    /// Offset in `VkDeviceMemory` object to the beginning of this allocation, in bytes. `(deviceMemory, offset)` pair is unique to this allocation.
    ///
    /// You usually don't need to use this offset. If you create a buffer or an image together with the allocation using e.g. function
    /// vmaCreateBuffer(), vmaCreateImage(), functions that operate on these resources refer to the beginning of the buffer or image,
    /// not entire device memory block. Functions like vmaMapMemory(), vmaBindBufferMemory() also refer to the beginning of the allocation
    /// and apply this offset automatically.
    ///
    /// It can change after the allocation is moved during defragmentation.
    pub offset: u64,
    /// Size of this allocation, in bytes.
    ///
    /// It never changes.
    ///
    /// Allocation size returned in this variable may be greater than the size
    /// requested for the resource e.g. as `VkBufferCreateInfo::size`. Whole size of the
    /// allocation is accessible for operations on memory e.g. using a pointer after
    /// mapping with vmaMapMemory(), but operations on the resource e.g. using
    /// `vkCmdCopyBuffer` must be limited to the size of the resource.
    pub size: u64,
    /// Pointer to the beginning of this allocation as mapped data.
    ///
    /// If the allocation hasn't been mapped using vmaMapMemory() and hasn't been
    /// created with`VMA_ALLOCATION_CREATE_MAPPED_BIT` flag, this value is null.
    ///
    /// It can change after call to vmaMapMemory(), vmaUnmapMemory().
    /// It can also change after the allocation is moved during defragmentation.
    pub mapped_data: *mut std::ffi::c_void,
    /// Custom general-purpose pointer that was passed as VmaAllocationCreateInfo::pUserData or set using vmaSetAllocationUserData().
    ///
    /// It can change after call to vmaSetAllocationUserData() for this allocation.
    pub user_data: *mut std::ffi::c_void,
    /// Custom allocation name that was set with vmaSetAllocationName().
    ///
    /// It can change after call to vmaSetAllocationName() for this allocation.
    ///
    /// Another way to set custom name is to pass it in VmaAllocationCreateInfo::pUserData with
    /// additional flag`VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT` set [DEPRECATED].
    pub name: *const std::ffi::c_char,
}
/// Parameters for defragmentation.
///
/// To be used with function vmaBeginDefragmentation().
#[derive(Clone)]
#[repr(C)]
pub struct DefragmentationInfo {
    /// Use combination of`VmaDefragmentationFlagBits.`
    pub flags: DefragmentationFlags,
    /// Custom pool to be defragmented.
    ///
    /// If null then default pools will undergo defragmentation process.
    pub pool: Pool,
    /// Maximum numbers of bytes that can be copied during single pass, while moving allocations to different places.
    ///
    /// `0` means no limit.
    pub max_bytes_per_pass: u64,
    /// Maximum number of allocations that can be moved during single pass to a different place.
    ///
    /// `0` means no limit.
    pub max_allocations_per_pass: u32,
}
/// Single move of an allocation to be done for defragmentation.
#[derive(Clone)]
#[repr(C)]
pub struct DefragmentationMove {
    /// Operation to be performed on the allocation by vmaEndDefragmentationPass(). Default value is`VMA_DEFRAGMENTATION_MOVE_OPERATION_COPY.` You can modify it.
    pub operation: DefragmentationMoveOperation,
    /// Allocation that should be moved.
    pub src_allocation: Allocation,
    /// Temporary allocation pointing to destination memory that will replace `srcAllocation`.
    ///
    /// **Do not store this allocation in your data structures! It exists only temporarily, for the duration of the defragmentation pass,
    /// to be used for binding new buffer/image to the destination memory using e.g. vmaBindBufferMemory().
    /// vmaEndDefragmentationPass() will destroy it and make `srcAllocation` point to this memory.**
    pub dst_tmp_allocation: Allocation,
}
/// Parameters for incremental defragmentation steps.
///
/// To be used with function vmaBeginDefragmentationPass().
#[derive(Clone)]
#[repr(C)]
pub struct DefragmentationPassMoveInfo {
    /// Number of elements in the `pMoves` array.
    pub move_count: u32,
    /// Array of moves to be performed by the user in the current defragmentation pass.
    ///
    /// Pointer to an array of `moveCount` elements, owned by VMA, created in vmaBeginDefragmentationPass(), destroyed in vmaEndDefragmentationPass().
    ///
    /// For each element, you should:
    ///
    /// 1. Create a new buffer/image in the place pointed by VmaDefragmentationMove::dstMemory + VmaDefragmentationMove::dstOffset.
    /// 2. Copy data from the VmaDefragmentationMove::srcAllocation e.g. using `vkCmdCopyBuffer`, `vkCmdCopyImage`.
    /// 3. Make sure these commands finished executing on the GPU.
    /// 4. Destroy the old buffer/image.
    ///
    /// Only then you can finish defragmentation pass by calling vmaEndDefragmentationPass().
    /// After this call, the allocation will point to the new place in memory.
    ///
    /// Alternatively, if you cannot move specific allocation, you can set VmaDefragmentationMove::operation to`VMA_DEFRAGMENTATION_MOVE_OPERATION_IGNORE.`
    ///
    /// Alternatively, if you decide you want to completely remove the allocation:
    ///
    /// 1. Destroy its buffer/image.
    /// 2. Set VmaDefragmentationMove::operation to`VMA_DEFRAGMENTATION_MOVE_OPERATION_DESTROY.`
    ///
    /// Then, after vmaEndDefragmentationPass() the allocation will be freed.
    pub moves: *mut DefragmentationMove,
}
/// Statistics returned for defragmentation process in function vmaEndDefragmentation().
#[derive(Clone)]
#[repr(C)]
pub struct DefragmentationStats {
    /// Total number of bytes that have been copied while moving allocations to different places.
    pub bytes_moved: u64,
    /// Total number of bytes that have been released to the system by freeing empty `VkDeviceMemory` objects.
    pub bytes_freed: u64,
    /// Number of allocations that have been moved to different places.
    pub allocations_moved: u32,
    /// Number of empty `VkDeviceMemory` objects that have been released to the system.
    pub device_memory_blocks_freed: u32,
}
/// Parameters of created`VmaVirtualBlock` object to be passed to vmaCreateVirtualBlock().
#[derive(Clone)]
#[repr(C)]
pub struct VirtualBlockCreateInfo {
    /// Total size of the virtual block.
    ///
    /// Sizes can be expressed in bytes or any units you want as long as you are consistent in using them.
    /// For example, if you allocate from some array of structures, 1 can mean single instance of entire structure.
    pub size: u64,
    /// Use combination of`VmaVirtualBlockCreateFlagBits.`
    pub flags: VirtualBlockCreateFlags,
    /// Custom`CPU` memory allocation callbacks. Optional.
    ///
    /// Optional, can be null. When specified, they will be used for all CPU-side memory allocations.
    pub allocation_callbacks: *const vk::AllocationCallbacks,
}
/// Parameters of created virtual allocation to be passed to vmaVirtualAllocate().
#[derive(Clone)]
#[repr(C)]
pub struct VirtualAllocationCreateInfo {
    /// Size of the allocation.
    ///
    /// Cannot be zero.
    pub size: u64,
    /// Required alignment of the allocation. Optional.
    ///
    /// Must be power of two. Special value 0 has the same meaning as 1 - means no special alignment is required, so allocation can start at any offset.
    pub alignment: u64,
    /// Use combination of`VmaVirtualAllocationCreateFlagBits.`
    pub flags: VirtualAllocationCreateFlags,
    /// Custom pointer to be associated with the allocation. Optional.
    ///
    /// It can be any value and can be used for user-defined purposes. It can be fetched or changed later.
    pub user_data: *mut std::ffi::c_void,
}
/// Parameters of an existing virtual allocation, returned by vmaGetVirtualAllocationInfo().
#[derive(Clone)]
#[repr(C)]
pub struct VirtualAllocationInfo {
    /// Offset of the allocation.
    ///
    /// Offset at which the allocation was made.
    pub offset: u64,
    /// Size of the allocation.
    ///
    /// Same value as passed in VmaVirtualAllocationCreateInfo::size.
    pub size: u64,
    /// Custom pointer associated with the allocation.
    ///
    /// Same value as passed in VmaVirtualAllocationCreateInfo::pUserData or to vmaSetVirtualAllocationUserData().
    pub user_data: *mut std::ffi::c_void,
}
extern "C" {
    /// Creates`VmaAllocator` object.
    pub fn vmaCreateAllocator(
        create_info: *const AllocatorCreateInfo,
        allocator: *mut Allocator,
    ) -> vk::Result;
}
extern "C" {
    /// Destroys allocator object.
    pub fn vmaDestroyAllocator(allocator: Allocator);
}
extern "C" {
    /// Returns information about existing`VmaAllocator` object - handle to Vulkan device etc.
    ///
    /// It might be useful if you want to keep just the`VmaAllocator` handle and fetch other required handles to
    /// `VkPhysicalDevice`, `VkDevice` etc. every time using this function.
    pub fn vmaGetAllocatorInfo(allocator: Allocator, allocator_info: *mut AllocatorInfo);
}
extern "C" {
    /// PhysicalDeviceProperties are fetched from physicalDevice by the allocator.
    /// You can access it here, without fetching it again on your own.
    pub fn vmaGetPhysicalDeviceProperties(
        allocator: Allocator,
        physical_device_properties: *mut *const vk::PhysicalDeviceProperties,
    );
}
extern "C" {
    /// PhysicalDeviceMemoryProperties are fetched from physicalDevice by the allocator.
    /// You can access it here, without fetching it again on your own.
    pub fn vmaGetMemoryProperties(
        allocator: Allocator,
        physical_device_memory_properties: *mut *const vk::PhysicalDeviceMemoryProperties,
    );
}
extern "C" {
    /// Given Memory Type Index, returns Property Flags of this memory type.
    ///
    /// This is just a convenience function. Same information can be obtained using
    /// vmaGetMemoryProperties().
    pub fn vmaGetMemoryTypeProperties(
        allocator: Allocator,
        memory_type_index: u32,
        flags: *mut vk::MemoryPropertyFlags,
    );
}
extern "C" {
    /// Sets index of the current frame.
    pub fn vmaSetCurrentFrameIndex(allocator: Allocator, frame_index: u32);
}
extern "C" {
    /// Retrieves statistics from current state of the Allocator.
    ///
    /// This function is called "calculate" not "get" because it has to traverse all
    /// internal data structures, so it may be quite slow. Use it for debugging purposes.
    /// For faster but more brief statistics suitable to be called every frame or every allocation,
    /// use vmaGetHeapBudgets().
    ///
    /// Note that when using allocator from multiple threads, returned information may immediately
    /// become outdated.
    pub fn vmaCalculateStatistics(allocator: Allocator, stats: *mut TotalStatistics);
}
extern "C" {
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
    pub fn vmaGetHeapBudgets(allocator: Allocator, budgets: *mut Budget);
}
extern "C" {
    /// Helps to find memoryTypeIndex, given memoryTypeBits and VmaAllocationCreateInfo.
    ///
    /// This algorithm tries to find a memory type that:
    ///
    /// - Is allowed by memoryTypeBits.
    /// - Contains all the flags from pAllocationCreateInfo->requiredFlags.
    /// - Matches intended usage.
    /// - Has as many flags from pAllocationCreateInfo->preferredFlags as possible.
    ///
    /// \return Returns`VK_ERROR_FEATURE_NOT_PRESENT` if not found. Receiving such result
    /// from this function or any other allocating function probably means that your
    /// device doesn't support any memory type with requested features for the specific
    /// type of resource you want to use it for. Please check parameters of your
    /// resource, like image layout (OPTIMAL versus LINEAR) or mip level count.
    pub fn vmaFindMemoryTypeIndex(
        allocator: Allocator,
        memory_type_bits: u32,
        allocation_create_info: *const AllocationCreateInfo,
        memory_type_index: *mut u32,
    ) -> vk::Result;
}
extern "C" {
    /// Helps to find memoryTypeIndex, given VkBufferCreateInfo and VmaAllocationCreateInfo.
    ///
    /// It can be useful e.g. to determine value to be used as VmaPoolCreateInfo::memoryTypeIndex.
    /// It internally creates a temporary, dummy buffer that never has memory bound.
    pub fn vmaFindMemoryTypeIndexForBufferInfo(
        allocator: Allocator,
        buffer_create_info: *const vk::BufferCreateInfo,
        allocation_create_info: *const AllocationCreateInfo,
        memory_type_index: *mut u32,
    ) -> vk::Result;
}
extern "C" {
    /// Helps to find memoryTypeIndex, given VkImageCreateInfo and VmaAllocationCreateInfo.
    ///
    /// It can be useful e.g. to determine value to be used as VmaPoolCreateInfo::memoryTypeIndex.
    /// It internally creates a temporary, dummy image that never has memory bound.
    pub fn vmaFindMemoryTypeIndexForImageInfo(
        allocator: Allocator,
        image_create_info: *const vk::ImageCreateInfo,
        allocation_create_info: *const AllocationCreateInfo,
        memory_type_index: *mut u32,
    ) -> vk::Result;
}
extern "C" {
    /// Allocates Vulkan device memory and creates`VmaPool` object.
    ///
    /// - `allocator` Allocator object.
    /// - `pCreateInfo` Parameters of pool to create.
    /// - *out* `pPool` Handle to created pool.
    pub fn vmaCreatePool(
        allocator: Allocator,
        create_info: *const PoolCreateInfo,
        pool: *mut Pool,
    ) -> vk::Result;
}
extern "C" {
    /// Destroys`VmaPool` object and frees Vulkan device memory.
    pub fn vmaDestroyPool(allocator: Allocator, pool: Pool);
}
extern "C" {
    /// Retrieves statistics of existing`VmaPool` object.
    ///
    /// - `allocator` Allocator object.
    /// - `pool` Pool object.
    /// - *out* `pPoolStats` Statistics of specified pool.
    pub fn vmaGetPoolStatistics(
        allocator: Allocator,
        pool: Pool,
        pool_stats: *mut Statistics,
    );
}
extern "C" {
    /// Retrieves detailed statistics of existing`VmaPool` object.
    ///
    /// - `allocator` Allocator object.
    /// - `pool` Pool object.
    /// - *out* `pPoolStats` Statistics of specified pool.
    pub fn vmaCalculatePoolStatistics(
        allocator: Allocator,
        pool: Pool,
        pool_stats: *mut DetailedStatistics,
    );
}
extern "C" {
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
    pub fn vmaCheckPoolCorruption(allocator: Allocator, pool: Pool) -> vk::Result;
}
extern "C" {
    /// Retrieves name of a custom pool.
    ///
    /// After the call `ppName` is either null or points to an internally-owned null-terminated string
    /// containing name of the pool that was previously set. The pointer becomes invalid when the pool is
    /// destroyed or its name is changed using vmaSetPoolName().
    pub fn vmaGetPoolName(
        allocator: Allocator,
        pool: Pool,
        name: *mut *const std::ffi::c_char,
    );
}
extern "C" {
    /// Sets name of a custom pool.
    ///
    /// `pName` can be either null or pointer to a null-terminated string with new name for the pool.
    /// Function makes internal copy of the string, so it can be changed or freed immediately after this call.
    pub fn vmaSetPoolName(
        allocator: Allocator,
        pool: Pool,
        name: *const std::ffi::c_char,
    );
}
extern "C" {
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
    pub fn vmaAllocateMemory(
        allocator: Allocator,
        vk_memory_requirements: *const vk::MemoryRequirements,
        create_info: *const AllocationCreateInfo,
        allocation: *mut Allocation,
        allocation_info: *mut AllocationInfo,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaAllocateMemoryPages(
        allocator: Allocator,
        vk_memory_requirements: *const vk::MemoryRequirements,
        create_info: *const AllocationCreateInfo,
        allocation_count: usize,
        allocations: *mut Allocation,
        allocation_info: *mut AllocationInfo,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaAllocateMemoryForBuffer(
        allocator: Allocator,
        buffer: vk::Buffer,
        create_info: *const AllocationCreateInfo,
        allocation: *mut Allocation,
        allocation_info: *mut AllocationInfo,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaAllocateMemoryForImage(
        allocator: Allocator,
        image: vk::Image,
        create_info: *const AllocationCreateInfo,
        allocation: *mut Allocation,
        allocation_info: *mut AllocationInfo,
    ) -> vk::Result;
}
extern "C" {
    /// Frees memory previously allocated using vmaAllocateMemory(), vmaAllocateMemoryForBuffer(), or vmaAllocateMemoryForImage().
    ///
    /// Passing `VK_NULL_HANDLE` as `allocation` is valid. Such function call is just skipped.
    pub fn vmaFreeMemory(allocator: Allocator, allocation: Allocation);
}
extern "C" {
    /// Frees memory and destroys multiple allocations.
    ///
    /// Word "pages" is just a suggestion to use this function to free pieces of memory used for sparse binding.
    /// It is just a general purpose function to free memory and destroy allocations made using e.g. vmaAllocateMemory(),
    /// vmaAllocateMemoryPages() and other functions.
    /// It may be internally optimized to be more efficient than calling vmaFreeMemory() `allocationCount` times.
    ///
    /// Allocations in `pAllocations` array can come from any memory pools and types.
    /// Passing `VK_NULL_HANDLE` as elements of `pAllocations` array is valid. Such entries are just skipped.
    pub fn vmaFreeMemoryPages(
        allocator: Allocator,
        allocation_count: usize,
        allocations: *const Allocation,
    );
}
extern "C" {
    /// Returns current information about specified allocation.
    ///
    /// Current parameters of given allocation are returned in `pAllocationInfo`.
    ///
    /// Although this function doesn't lock any mutex, so it should be quite efficient,
    /// you should avoid calling it too often.
    /// You can retrieve same VmaAllocationInfo structure while creating your resource, from function
    /// vmaCreateBuffer(), vmaCreateImage(). You can remember it if you are sure parameters don't change
    /// (e.g. due to defragmentation).
    pub fn vmaGetAllocationInfo(
        allocator: Allocator,
        allocation: Allocation,
        allocation_info: *mut AllocationInfo,
    );
}
extern "C" {
    /// Sets pUserData in given allocation to new value.
    ///
    /// The value of pointer `pUserData` is copied to allocation's `pUserData`.
    /// It is opaque, so you can use it however you want - e.g.
    /// as a pointer, ordinal number or some handle to you own data.
    pub fn vmaSetAllocationUserData(
        allocator: Allocator,
        allocation: Allocation,
        user_data: *mut std::ffi::c_void,
    );
}
extern "C" {
    /// Sets pName in given allocation to new value.
    ///
    /// `pName` must be either null, or pointer to a null-terminated string. The function
    /// makes local copy of the string and sets it as allocation's `pName`. String
    /// passed as pName doesn't need to be valid for whole lifetime of the allocation -
    /// you can free it after this call. String previously pointed by allocation's
    /// `pName` is freed from memory.
    pub fn vmaSetAllocationName(
        allocator: Allocator,
        allocation: Allocation,
        name: *const std::ffi::c_char,
    );
}
extern "C" {
    /// Given an allocation, returns Property Flags of its memory type.
    ///
    /// This is just a convenience function. Same information can be obtained using
    /// vmaGetAllocationInfo() + vmaGetMemoryProperties().
    pub fn vmaGetAllocationMemoryProperties(
        allocator: Allocator,
        allocation: Allocation,
        flags: *mut vk::MemoryPropertyFlags,
    );
}
extern "C" {
    /// Maps memory represented by given allocation and returns pointer to it.
    ///
    /// Maps memory represented by given allocation to make it accessible to`CPU` code.
    /// When succeeded, `*ppData` contains pointer to first byte of this memory.
    ///
    /// **
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
    /// "0-th" mapping made automatically due to`VMA_ALLOCATION_CREATE_MAPPED_BIT` flag.
    ///
    /// This function fails when used on allocation made in memory type that is not
    /// `HOST_VISIBLE`.
    ///
    /// This function doesn't automatically flush or invalidate caches.
    /// If the allocation is made from a memory types that is not `HOST_COHERENT`,
    /// you also need to use vmaInvalidateAllocation() / vmaFlushAllocation(), as required by Vulkan specification.
    pub fn vmaMapMemory(
        allocator: Allocator,
        allocation: Allocation,
        data: *mut *mut std::ffi::c_void,
    ) -> vk::Result;
}
extern "C" {
    /// Unmaps memory represented by given allocation, mapped previously using vmaMapMemory().
    ///
    /// For details, see description of vmaMapMemory().
    ///
    /// This function doesn't automatically flush or invalidate caches.
    /// If the allocation is made from a memory types that is not `HOST_COHERENT`,
    /// you also need to use vmaInvalidateAllocation() / vmaFlushAllocation(), as required by Vulkan specification.
    pub fn vmaUnmapMemory(allocator: Allocator, allocation: Allocation);
}
extern "C" {
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
    pub fn vmaFlushAllocation(
        allocator: Allocator,
        allocation: Allocation,
        offset: u64,
        size: u64,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaInvalidateAllocation(
        allocator: Allocator,
        allocation: Allocation,
        offset: u64,
        size: u64,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaFlushAllocations(
        allocator: Allocator,
        allocation_count: u32,
        allocations: *const Allocation,
        offsets: *const u64,
        sizes: *const u64,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaInvalidateAllocations(
        allocator: Allocator,
        allocation_count: u32,
        allocations: *const Allocation,
        offsets: *const u64,
        sizes: *const u64,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaCheckCorruption(allocator: Allocator, memory_type_bits: u32) -> vk::Result;
}
extern "C" {
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
    pub fn vmaBeginDefragmentation(
        allocator: Allocator,
        info: *const DefragmentationInfo,
        context: *mut DefragmentationContext,
    ) -> vk::Result;
}
extern "C" {
    /// Ends defragmentation process.
    ///
    /// - `allocator` Allocator object.
    /// - `context` Context object that has been created by vmaBeginDefragmentation().
    /// - *out* `pStats` Optional stats for the defragmentation. Can be null.
    ///
    /// Use this function to finish defragmentation started by vmaBeginDefragmentation().
    pub fn vmaEndDefragmentation(
        allocator: Allocator,
        context: DefragmentationContext,
        stats: *mut DefragmentationStats,
    );
}
extern "C" {
    /// Starts single defragmentation pass.
    ///
    /// - `allocator` Allocator object.
    /// - `context` Context object that has been created by vmaBeginDefragmentation().
    /// - *out* `pPassInfo` Computed information for current pass.
    /// \returns
    /// - `VK_SUCCESS` if no more moves are possible. Then you can omit call to vmaEndDefragmentationPass() and simply end whole defragmentation.
    /// - `VK_INCOMPLETE` if there are pending moves returned in `pPassInfo`. You need to perform them, call vmaEndDefragmentationPass(),
    /// and then preferably try another pass with vmaBeginDefragmentationPass().
    pub fn vmaBeginDefragmentationPass(
        allocator: Allocator,
        context: DefragmentationContext,
        pass_info: *mut DefragmentationPassMoveInfo,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaEndDefragmentationPass(
        allocator: Allocator,
        context: DefragmentationContext,
        pass_info: *mut DefragmentationPassMoveInfo,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaBindBufferMemory(
        allocator: Allocator,
        allocation: Allocation,
        buffer: vk::Buffer,
    ) -> vk::Result;
}
extern "C" {
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
    /// If `pNext` is not null,`VmaAllocator` object must have been created with`VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT` flag
    /// or with VmaAllocatorCreateInfo::vulkanApiVersion `>= VK_API_VERSION_1_1`. Otherwise the call fails.
    pub fn vmaBindBufferMemory2(
        allocator: Allocator,
        allocation: Allocation,
        allocation_local_offset: u64,
        buffer: vk::Buffer,
        p_next: *const std::ffi::c_void,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaBindImageMemory(
        allocator: Allocator,
        allocation: Allocation,
        image: vk::Image,
    ) -> vk::Result;
}
extern "C" {
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
    /// If `pNext` is not null,`VmaAllocator` object must have been created with`VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT` flag
    /// or with VmaAllocatorCreateInfo::vulkanApiVersion `>= VK_API_VERSION_1_1`. Otherwise the call fails.
    pub fn vmaBindImageMemory2(
        allocator: Allocator,
        allocation: Allocation,
        allocation_local_offset: u64,
        image: vk::Image,
        p_next: *const std::ffi::c_void,
    ) -> vk::Result;
}
extern "C" {
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
    /// If`VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT` flag was used,
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
    pub fn vmaCreateBuffer(
        allocator: Allocator,
        buffer_create_info: *const vk::BufferCreateInfo,
        allocation_create_info: *const AllocationCreateInfo,
        buffer: *mut vk::Buffer,
        allocation: *mut Allocation,
        allocation_info: *mut AllocationInfo,
    ) -> vk::Result;
}
extern "C" {
    /// Creates a buffer with additional minimum alignment.
    ///
    /// Similar to vmaCreateBuffer() but provides additional parameter `minAlignment` which allows to specify custom,
    /// minimum alignment to be used when placing the buffer inside a larger memory block, which may be needed e.g.
    /// for interop with OpenGL.
    pub fn vmaCreateBufferWithAlignment(
        allocator: Allocator,
        buffer_create_info: *const vk::BufferCreateInfo,
        allocation_create_info: *const AllocationCreateInfo,
        min_alignment: u64,
        buffer: *mut vk::Buffer,
        allocation: *mut Allocation,
        allocation_info: *mut AllocationInfo,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaCreateAliasingBuffer(
        allocator: Allocator,
        allocation: Allocation,
        buffer_create_info: *const vk::BufferCreateInfo,
        buffer: *mut vk::Buffer,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaCreateAliasingBuffer2(
        allocator: Allocator,
        allocation: Allocation,
        allocation_local_offset: u64,
        buffer_create_info: *const vk::BufferCreateInfo,
        buffer: *mut vk::Buffer,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaDestroyBuffer(
        allocator: Allocator,
        buffer: vk::Buffer,
        allocation: Allocation,
    );
}
extern "C" {
    /// Function similar to vmaCreateBuffer().
    pub fn vmaCreateImage(
        allocator: Allocator,
        image_create_info: *const vk::ImageCreateInfo,
        allocation_create_info: *const AllocationCreateInfo,
        image: *mut vk::Image,
        allocation: *mut Allocation,
        allocation_info: *mut AllocationInfo,
    ) -> vk::Result;
}
extern "C" {
    /// Function similar to vmaCreateAliasingBuffer() but for images.
    pub fn vmaCreateAliasingImage(
        allocator: Allocator,
        allocation: Allocation,
        image_create_info: *const vk::ImageCreateInfo,
        image: *mut vk::Image,
    ) -> vk::Result;
}
extern "C" {
    /// Function similar to vmaCreateAliasingBuffer2() but for images.
    pub fn vmaCreateAliasingImage2(
        allocator: Allocator,
        allocation: Allocation,
        allocation_local_offset: u64,
        image_create_info: *const vk::ImageCreateInfo,
        image: *mut vk::Image,
    ) -> vk::Result;
}
extern "C" {
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
    pub fn vmaDestroyImage(
        allocator: Allocator,
        image: vk::Image,
        allocation: Allocation,
    );
}
extern "C" {
    /// Creates new`VmaVirtualBlock` object.
    ///
    /// - `pCreateInfo` Parameters for creation.
    /// - *out* `pVirtualBlock` Returned virtual block object or `VMA_NULL` if creation failed.
    pub fn vmaCreateVirtualBlock(
        create_info: *const VirtualBlockCreateInfo,
        virtual_block: *mut VirtualBlock,
    ) -> vk::Result;
}
extern "C" {
    /// Destroys`VmaVirtualBlock` object.
    ///
    /// Please note that you should consciously handle virtual allocations that could remain unfreed in the block.
    /// You should either free them individually using vmaVirtualFree() or call vmaClearVirtualBlock()
    /// if you are sure this is what you want. If you do neither, an assert is called.
    ///
    /// If you keep pointers to some additional metadata associated with your virtual allocations in their `pUserData`,
    /// don't forget to free them.
    pub fn vmaDestroyVirtualBlock(virtual_block: VirtualBlock);
}
extern "C" {
    /// Returns true of the`VmaVirtualBlock` is empty - contains 0 virtual allocations and has all its space available for new allocations.
    pub fn vmaIsVirtualBlockEmpty(virtual_block: VirtualBlock) -> vk::Bool32;
}
extern "C" {
    /// Returns information about a specific virtual allocation within a virtual block, like its size and `pUserData` pointer.
    pub fn vmaGetVirtualAllocationInfo(
        virtual_block: VirtualBlock,
        allocation: VirtualAllocation,
        virtual_alloc_info: *mut VirtualAllocationInfo,
    );
}
extern "C" {
    /// Allocates new virtual allocation inside given`VmaVirtualBlock.`
    ///
    /// If the allocation fails due to not enough free space available, `VK_ERROR_OUT_OF_DEVICE_MEMORY` is returned
    /// (despite the function doesn't ever allocate actual`GPU` memory).
    /// `pAllocation` is then set to `VK_NULL_HANDLE` and `pOffset`, if not null, it set to `UINT64_MAX`.
    ///
    /// - `virtualBlock` Virtual block
    /// - `pCreateInfo` Parameters for the allocation
    /// - *out* `pAllocation` Returned handle of the new allocation
    /// - *out* `pOffset` Returned offset of the new allocation. Optional, can be null.
    pub fn vmaVirtualAllocate(
        virtual_block: VirtualBlock,
        create_info: *const VirtualAllocationCreateInfo,
        allocation: *mut VirtualAllocation,
        offset: *mut u64,
    ) -> vk::Result;
}
extern "C" {
    /// Frees virtual allocation inside given`VmaVirtualBlock.`
    ///
    /// It is correct to call this function with `allocation == VK_NULL_HANDLE` - it does nothing.
    pub fn vmaVirtualFree(virtual_block: VirtualBlock, allocation: VirtualAllocation);
}
extern "C" {
    /// Frees all virtual allocations inside given`VmaVirtualBlock.`
    ///
    /// You must either call this function or free each virtual allocation individually with vmaVirtualFree()
    /// before destroying a virtual block. Otherwise, an assert is called.
    ///
    /// If you keep pointer to some additional metadata associated with your virtual allocation in its `pUserData`,
    /// don't forget to free it as well.
    pub fn vmaClearVirtualBlock(virtual_block: VirtualBlock);
}
extern "C" {
    /// Changes custom pointer associated with given virtual allocation.
    pub fn vmaSetVirtualAllocationUserData(
        virtual_block: VirtualBlock,
        allocation: VirtualAllocation,
        user_data: *mut std::ffi::c_void,
    );
}
extern "C" {
    /// Calculates and returns statistics about virtual allocations and memory usage in given`VmaVirtualBlock.`
    ///
    /// This function is fast to call. For more detailed statistics, see vmaCalculateVirtualBlockStatistics().
    pub fn vmaGetVirtualBlockStatistics(
        virtual_block: VirtualBlock,
        stats: *mut Statistics,
    );
}
extern "C" {
    /// Calculates and returns detailed statistics about virtual allocations and memory usage in given`VmaVirtualBlock.`
    ///
    /// This function is slow to call. Use for debugging purposes.
    /// For less detailed statistics, see vmaGetVirtualBlockStatistics().
    pub fn vmaCalculateVirtualBlockStatistics(
        virtual_block: VirtualBlock,
        stats: *mut DetailedStatistics,
    );
}
extern "C" {
    /// Builds and returns a null-terminated string in`JSON` format with information about given`VmaVirtualBlock.`
    /// - `virtualBlock` Virtual block.
    /// - *out* `ppStatsString` Returned string.
    /// - `detailedMap` Pass `VK_FALSE` to only obtain statistics as returned by vmaCalculateVirtualBlockStatistics(). Pass `VK_TRUE` to also obtain full list of allocations and free spaces.
    ///
    /// Returned string must be freed using vmaFreeVirtualBlockStatsString().
    pub fn vmaBuildVirtualBlockStatsString(
        virtual_block: VirtualBlock,
        stats_string: *mut *mut std::ffi::c_char,
        detailed_map: vk::Bool32,
    );
}
extern "C" {
    /// Frees a string returned by vmaBuildVirtualBlockStatsString().
    pub fn vmaFreeVirtualBlockStatsString(
        virtual_block: VirtualBlock,
        stats_string: *mut std::ffi::c_char,
    );
}
extern "C" {
    /// Builds and returns statistics as a null-terminated string in`JSON` format.
    /// - `allocator`
    /// - *out* `ppStatsString` Must be freed using vmaFreeStatsString() function.
    /// - `detailedMap`
    pub fn vmaBuildStatsString(
        allocator: Allocator,
        stats_string: *mut *mut std::ffi::c_char,
        detailed_map: vk::Bool32,
    );
}
extern "C" {
    pub fn vmaFreeStatsString(allocator: Allocator, stats_string: *mut std::ffi::c_char);
}
