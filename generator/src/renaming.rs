use std::fmt::Write;

pub struct CamelCaseSplit<'a> {
    str: &'a str,
}

impl<'a> CamelCaseSplit<'a> {
    fn new(str: &'a str) -> Self {
        Self { str }
    }
}

impl<'a> Iterator for CamelCaseSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.str.is_empty() {
            return None;
        }

        let mut chars = self.str.chars();
        let mut prev = chars.next().unwrap();

        for (i, c) in chars.enumerate() {
            // just match all the different situations where we want to end a "chunk"
            // Aa|A, Aa|42, 42|Aa       * Aa is just an example of identifier starting with a capital letter
            if (prev.is_ascii_lowercase() && c.is_ascii_uppercase())
                || (prev.is_ascii_lowercase() && c.is_ascii_digit())
                || (prev.is_ascii_digit() && c.is_ascii_uppercase())
            {
                let (l, r) = self.str.split_at(i + 1); // +1 because we started iterating after already pulling a character from the iterator
                self.str = r;
                return Some(l);
            }
            prev = c;
        }

        return Some(std::mem::replace(&mut self.str, &""));
    }
}

static mut INTERNAL: String = String::new();

pub fn camel_to_snake(out: &mut String) {
    assert_eq!(std::thread::current().name().unwrap(), "main");

    // pnext is somewhat special, preserve it
    if out == "pNext" {
        out.clear();
        out.push_str("p_next");
        return;
    }

    let copy = unsafe {
        INTERNAL.clear();
        INTERNAL.push_str(&*out);
        out.clear();
        &INTERNAL
    };

    // filter p_vulkan_functions to "vulkan_functions"
    let mut iter = CamelCaseSplit::new(copy).filter(|&str| !str.chars().all(|c| c == 'p'));
    out.push_str(iter.next().unwrap());
    for next in iter {
        out.push('_');
        out.push_str(next);
    }
    out.make_ascii_lowercase();
}

// pub fn snake_to_camel(out: &mut String) {
//     assert_eq!(std::thread::current().name().unwrap(), "main");

//     let copy = unsafe {
//         INTERNAL.clear();
//         INTERNAL.push_str(&*out);
//         out.clear();
//         &INTERNAL
//     };

//     for next in copy.split('_') {
//         let mut c = next.chars();
//         out.push(c.next().unwrap().to_ascii_uppercase());
//         out.extend(c.map(|c| c.to_ascii_lowercase()));
//     }
// }

pub fn item_name<'a>(str: &'a str, item: bool, buf: &'a mut String) {
    buf.clear();

    if let Some(flagbits) = str
        .strip_suffix("FlagBits")
        .and_then(|s| s.strip_prefix("Vma"))
    {
        write!(buf, "{flagbits}Flags").unwrap();
        return;
    }

    if item {
        if let Some(vk) = str.strip_prefix("Vk").or_else(|| str.strip_prefix("vk")) {
            write!(buf, "vk::{vk}").unwrap();
            return;
        }
    }

    if item {
        if str.starts_with("PFN_") {
            write!(buf, "Option<{str}>").unwrap();
            return;
        }
    }

    buf.push_str(
        None.or_else(|| str.strip_prefix("Vma"))
            .or_else(|| str.strip_prefix("Vk"))
            .or_else(|| str.strip_prefix("vk"))
            .unwrap_or(str),
    );
}

pub fn rename(str: &str) -> Option<&'static str> {
    static mut RENAMES: &mut [(&str, &str)] = &mut [
        ("uint32_t", "u32"),
        ("size_t", "usize"),
        ("VkDeviceSize", "u64"),
        (
            "VkExternalMemoryHandleTypeFlags",
            "ExternalMemoryHandleTypeFlags",
        ),
        (
            "VkExternalMemoryHandleTypeFlagsKHR",
            "ExternalMemoryHandleTypeFlags",
        ),
    ];
    static SORT: std::sync::Once = std::sync::Once::new();

    SORT.call_once(|| unsafe {
        RENAMES.sort_unstable_by_key(|&(s, _)| s);
    });
    let renames = unsafe { &*RENAMES };

    renames
        .binary_search_by_key(&str, |(s, _)| s)
        .map(|i| renames[i].1)
        .ok()
}

pub fn strip_enum_variant(enum_name: &str, variant: &str, buf: &mut String) {
    let strip_map = [
        ("VmaAllocatorCreateFlagBits", "VMA_ALLOCATOR_CREATE"),
        ("VmaMemoryUsage", "VMA_MEMORY_USAGE"),
        ("VmaAllocationCreateFlagBits", "VMA_ALLOCATION_CREATE"),
        ("VmaPoolCreateFlagBits", "VMA_POOL_CREATE"),
        ("VmaDefragmentationFlagBits", "VMA_DEFRAGMENTATION_FLAG"),
        ("VmaVirtualBlockCreateFlagBits", "VMA_VIRTUAL_BLOCK_CREATE"),
        (
            "VmaVirtualAllocationCreateFlagBits",
            "VMA_VIRTUAL_ALLOCATION_CREATE",
        ),
    ];

    buf.clear();
    for (name, strip) in strip_map {
        if enum_name == name {
            let preserve = variant
                .strip_prefix(strip)
                .unwrap()
                .trim_start_matches('_')
                .trim_end_matches("_BIT");

            buf.reserve(preserve.len());
            for word in preserve.split('_') {
                let mut chars = word.chars();
                buf.push(chars.next().unwrap().to_ascii_uppercase());
                buf.extend(chars.map(|c| c.to_ascii_lowercase()));
            }
            assert!(!buf.is_empty());
            return;
        }
    }

    buf.push_str(variant);
}
