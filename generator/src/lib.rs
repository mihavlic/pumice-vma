mod clang_setup;
mod renaming;

use std::{
    fmt::{Display, Write},
    fs::File,
    path::PathBuf,
};

use clang::*;
use codewrite::{
    formatters::{prettyplease::PrettyFormatter, Formatter},
    CFmt, Cond, Format2Format, Fun, FunOnce,
};
use codewrite_macro::code;

use crate::clang_setup::{find_clang_executable, get_build_target, get_header_include_paths};

type Writer = PrettyFormatter<Format2Format<File>>;

macro_rules! unwrap_path {
    ($e:expr) => {
        {
            let e = $e;
            e.to_str().unwrap_or_else(|| panic!("Path isn't UTF8, this is a limitation of clang bindings, go pester the developer ({})", e.to_string_lossy()))
        }
    };
}

pub struct BuildOptions<'a> {
    pub includes: &'a [PathBuf],
    pub source_header: PathBuf,
    pub output_path: PathBuf,
}

pub fn write_bindings(
    options: &BuildOptions,
) -> Result<(), <Writer as codewrite::formatters::Formatter>::Err> {
    assert!(options.source_header.is_file() && options.source_header.exists());

    let includes = options
        .includes
        .iter()
        .map(|s| format!("-I{}", unwrap_path!(s)));

    let mut args = ["-target".to_owned(), get_build_target()]
        .into_iter()
        .chain(includes)
        .collect::<Vec<_>>();

    let executable = find_clang_executable(&args).unwrap();
    args.extend(
        get_header_include_paths(true, &executable)
            .into_iter()
            .flat_map(|header| ["-isystem".to_owned(), header]),
    );

    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, true);

    let tu = index
        .parser(unwrap_path!(&options.source_header))
        .arguments(&args)
        .parse()
        .unwrap();

    let file = File::create(&options.output_path).unwrap();
    let mut writer = PrettyFormatter::new(Format2Format(file));

    insert_preamble().cfmt(&mut writer);

    tu.get_entity()
        .get_children()
        .into_iter()
        .for_each(|e| write_toplevel(&mut writer, e));

    writer.finish()
}

macro_rules! name {
    ($e:expr, $b:expr) => {{
        use std::borrow::Borrow;
        use std::borrow::BorrowMut;
        let s = $e;
        let s = renaming::rename(&s).unwrap_or(&s);
        renaming::item_name(&s, true, $b.borrow_mut());
        let v: &str = $b.borrow();
        v
    }};
}

macro_rules! ident {
    ($e:expr, $b:expr) => {{
        use std::borrow::Borrow;
        use std::borrow::BorrowMut;
        let s = $e;
        let s = renaming::rename(&s).unwrap_or(&s);
        renaming::item_name(&s, false, $b.borrow_mut());
        renaming::camel_to_snake($b.borrow_mut());
        let v: &str = $b.borrow();
        v
    }};
}

fn write_toplevel(w: &mut Writer, e: clang::Entity) {
    let raw_name = e.get_name().unwrap_or("Vk".to_owned());

    #[allow(unused)]
    #[derive(Clone, Copy)]
    enum MatchStrategy {
        Prefix,
        Suffix,
        Exact,
    }
    use MatchStrategy::*;

    const BLOCKED: &[(MatchStrategy, &str)] = &[
        (Prefix, "Vk"),
        (Prefix, "vk"),
        (Prefix, "StdVideo"),
        (Prefix, "__darwin_"),
        (Prefix, "PFN_vk"),
        // this blocks both VmaAllocator and VmaAllocator_T, the latter is not a part of the public api
        (Exact, "VmaAllocator"),
        (Exact, "VmaAllocator_T"),
        (Exact, "VmaPool"),
        (Exact, "VmaPool_T"),
        (Exact, "VmaAllocation"),
        (Exact, "VmaAllocation_T"),
        (Exact, "VmaDefragmentationContext"),
        (Exact, "VmaDefragmentationContext_T"),
        (Exact, "VmaVirtualBlock"),
        (Exact, "VmaVirtualBlock_T"),
        // nondispatchable handles
        (Exact, "VmaVirtualAllocation"),
        (Exact, "VmaVirtualAllocation_T"),
        (Exact, "VmaAllocHandle"),
        (Exact, "VmaAllocHandle_T"),
    ];

    for &(strategy, pattern) in BLOCKED {
        if match strategy {
            Prefix => raw_name.starts_with(pattern),
            Suffix => raw_name.ends_with(pattern),
            Exact => raw_name == pattern,
        } {
            return;
        }
    }

    let mut b1 = String::new();
    let mut b2 = String::new();
    let mut b3 = String::new();

    match e.get_kind() {
        EntityKind::StructDecl => {
            let comment = FmtDocComment(e.get_comment().unwrap_or(String::new()));
            let name = name!(&raw_name, b1);

            let fields = FunOnce::new(|w: &mut Writer| {
                for field in e.get_children() {
                    assert!(!field.is_bit_field(), "Unsupported");
                    assert_eq!(field.get_kind(), EntityKind::FieldDecl);

                    let comment = FmtDocComment(field.get_comment().unwrap_or(String::new()));
                    let raw_f_name = field.get_name().unwrap();

                    let f_name = if matches!(
                        raw_name.as_str(),
                        "VmaVulkanFunctions" | "VmaDeviceMemoryCallbacks"
                    ) {
                        &raw_f_name
                    } else {
                        ident!(raw_f_name, b2)
                    };

                    let f_type = TypeFmt(field.get_type().unwrap());

                    code!(
                        w,
                        $comment
                        pub $f_name: $f_type,
                    )
                }
            });

            code!(
                w,
                $comment
                #[derive(Clone)]
                #[repr(C)]
                pub struct $name {
                    $fields
                }
            )
        }
        EntityKind::EnumDecl => {
            let comment = FmtDocComment(e.get_comment().unwrap_or(String::new()));
            let name = name!(&raw_name, b1);
            let underlying_type = TypeFmt(e.get_enum_underlying_type().unwrap());

            if raw_name.contains("FlagBits") {
                let variants = FunOnce::new(|w: &mut Writer| {
                    for field in e.get_children() {
                        assert_eq!(field.get_kind(), EntityKind::EnumConstantDecl);

                        let comment = FmtDocComment(field.get_comment().unwrap_or(String::new()));
                        let f_name_raw = field.get_name().unwrap();
                        if f_name_raw.ends_with("_MAX_ENUM") {
                            continue;
                        }
                        renaming::strip_enum_variant(&raw_name, &f_name_raw, &mut b2);
                        renaming::camel_to_snake(&mut b2);
                        b2.make_ascii_uppercase();
                        let f_name = &b2;
                        let (_, f_value) = field.get_enum_constant_value().unwrap();

                        code!(
                            w,
                            $comment
                            pub const $f_name: Self = Self($f_value);
                        )
                    }
                });

                let mut all = 0;
                for field in e.get_children() {
                    assert_eq!(field.get_kind(), EntityKind::EnumConstantDecl);
                    let f_name_raw = field.get_name().unwrap();
                    if f_name_raw.ends_with("_MAX_ENUM") {
                        continue;
                    }
                    let (_, f_value) = field.get_enum_constant_value().unwrap();
                    all |= f_value;
                }
                let all = Fun::new(|w: &mut Writer| write!(w, "{:#x}", all).unwrap());

                let variants_post = FunOnce::new(|w: &mut Writer| {
                    let mut first = true;
                    for field in e.get_children() {
                        assert_eq!(field.get_kind(), EntityKind::EnumConstantDecl);

                        let f_name_raw = field.get_name().unwrap();
                        if f_name_raw.ends_with("_MAX_ENUM") {
                            continue;
                        }
                        renaming::strip_enum_variant(&raw_name, &f_name_raw, &mut b3);
                        renaming::camel_to_snake(&mut b3);
                        b3.make_ascii_uppercase();
                        let f_name = &b3;

                        let comma = Cond::new(!first, ',');
                        code!(
                            w,
                            $comma $f_name
                        );
                        first = false;
                    }
                });

                code!(
                    w,
                    $comment
                    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
                    #[repr(transparent)]
                    pub struct $name(pub $underlying_type);
                    impl $name {
                        $variants
                    }
                    pumice::bitflags_impl! {
                        $name: $underlying_type, $all, $variants_post
                    }
                )
            } else {
                let comment = FmtDocComment(e.get_comment().unwrap_or(String::new()));
                let variants = FunOnce::new(|w: &mut Writer| {
                    for field in e.get_children() {
                        assert_eq!(field.get_kind(), EntityKind::EnumConstantDecl);

                        let comment = FmtDocComment(field.get_comment().unwrap_or(String::new()));
                        let f_name_raw = field.get_name().unwrap();
                        if f_name_raw.ends_with("_MAX_ENUM") {
                            continue;
                        }
                        renaming::strip_enum_variant(&raw_name, &f_name_raw, &mut b3);
                        let f_name = &b3;
                        let (_, f_value) = field.get_enum_constant_value().unwrap();

                        code!(
                            w,
                            $comment
                            $f_name = $f_value,
                        )
                    }
                });

                code!(
                    w,
                    $comment
                    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
                    #[repr($underlying_type)]
                    pub enum $name {
                        $variants
                    }
                )
            }
        }
        EntityKind::FunctionDecl => {
            let comment = FmtDocComment(e.get_comment().unwrap_or(String::new()));
            let name = raw_name;

            let args = e.get_arguments().unwrap();
            // eprintln!("Fun {}", e.get_name().unwrap());
            // for a in &args {
            //     let t = a.get_type().unwrap();

            //     fn on_type(t: Type) {
            //         eprint!("({:?}, const {})", t.get_kind(), t.is_const_qualified(),);

            //         if let Some(t) = t.get_pointee_type() {
            //             eprint!(" *");
            //             on_type(t)
            //         } else if t.get_kind() == TypeKind::Typedef {
            //             let t = t
            //                 .get_declaration()
            //                 .unwrap()
            //                 .get_typedef_underlying_type()
            //                 .unwrap();
            //             eprint!(" Typedef");
            //             on_type(t);
            //         }
            //     }

            //     let can = t.get_canonical_type();
            //     eprint!(
            //         "  {}, {}, ({}): ",
            //         a.get_name().unwrap(),
            //         t.get_display_name(),
            //         can.get_display_name()
            //     );
            //     on_type(can);

            //     eprintln!()
            // }
            let args = FunOnce::new(|w: &mut Writer| {
                for arg in args {
                    assert_eq!(arg.get_kind(), EntityKind::ParmDecl);

                    let f_name = ident!(arg.get_name().unwrap(), b2);
                    let f_type = arg.get_type().unwrap();

                    code!(
                        w,
                        $f_name: $(TypeFmt(f_type)),
                    )
                }
            });

            let ret = e.get_result_type();
            let ret = Cond::new(
                ret.is_some() && ret.unwrap().get_kind() != TypeKind::Void,
                FunOnce::new(|w: &mut Writer| {
                    let ret = TypeFmt(ret.unwrap());
                    if name != "void" {
                        code!(w, -> $ret);
                    }
                }),
            );

            code!(
                w,
                extern "C" {
                    $comment
                    pub fn $name($args) $ret;
                }
            )
        }
        _ => {}
    }
}

struct TypeFmt<'a>(Type<'a>);

impl<'a> Display for TypeFmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type_fmt(self.0, f)
    }
}

fn type_fmt(ty: Type, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    macro_rules! either {
        ($rust:ty, $ffi:ty) => {
            if std::any::TypeId::of::<$rust>() == std::any::TypeId::of::<$ffi>() {
                stringify!($rust)
            } else {
                stringify!($ffi)
            }
        };
    }

    let str = match ty.get_kind() {
        TypeKind::Void => "std::ffi::c_void",
        TypeKind::Bool => "bool",
        // for the various chars we probably want to preserve the semantics of it being text
        TypeKind::CharS => "std::ffi::c_char", // either!(i8, std::ffi::c_char),
        TypeKind::CharU => "std::ffi::c_char", // either!(u8, std::ffi::c_char),
        TypeKind::SChar => "std::ffi::c_schar", // either!(i8, std::ffi::c_schar),
        TypeKind::UChar => "std::ffi::c_uchar", // either!(u8, std::ffi::c_uchar),
        TypeKind::WChar => "u16",
        TypeKind::Char16 => "u16",
        TypeKind::Char32 => "u32",
        TypeKind::Short => either!(i16, std::ffi::c_short),
        TypeKind::UShort => either!(u16, std::ffi::c_ushort),
        TypeKind::Int => either!(i32, std::ffi::c_int),
        TypeKind::UInt => either!(u32, std::ffi::c_uint),
        TypeKind::Long => either!(i64, std::ffi::c_long),
        TypeKind::ULong => either!(u64, std::ffi::c_ulong),
        TypeKind::LongLong => either!(i64, std::ffi::c_longlong),
        TypeKind::ULongLong => either!(u64, std::ffi::c_ulonglong),
        TypeKind::Int128 => "i128",
        TypeKind::UInt128 => "u128",
        TypeKind::Half => "u16", // rust doesn't have f16, we make do
        TypeKind::Float16 => "u16",
        TypeKind::Float => either!(f32, std::ffi::c_float),
        TypeKind::Double => either!(f64, std::ffi::c_double),
        // uhh, https://en.cppreference.com/w/cpp/types/nullptr_t
        TypeKind::Nullptr => {
            return write!(f, "*mut ()");
        }
        TypeKind::Float128 => "u128",
        TypeKind::ConstantArray => {
            let size = ty.get_size().unwrap();
            let inner = ty.get_element_type().unwrap();
            return write!(f, "[{}; {size}]", TypeFmt(inner));
        }
        TypeKind::Pointer => {
            let inner = ty.get_pointee_type().unwrap();
            // const may be hidden in typedefs
            if ty
                .get_canonical_type()
                .get_pointee_type()
                .unwrap()
                .is_const_qualified()
            {
                return write!(f, "*const {}", TypeFmt(inner));
            } else {
                return write!(f, "*mut  {}", TypeFmt(inner));
            }
        }
        // does this even have any abi?
        TypeKind::LValueReference => unimplemented!(),
        TypeKind::RValueReference => unimplemented!(),
        // not applicable? just display it
        _ => {
            let mut buf = String::new();

            // the display may already include "const", we strip it
            let display = ty.get_display_name();
            let name = name!(display.trim_start_matches("const").trim(), buf);

            return write!(f, "{name}");
        }
    };
    write!(f, "{str}")
}

#[derive(Clone)]
struct WordSplit<'a>(&'a str);

impl<'a> Iterator for WordSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let mut offset = 0;
        for c in self.0.chars() {
            let coffset = offset;
            offset += c.len_utf8();
            match c {
                '\n' => {
                    let found = self.0[offset..]
                        .find(|c: char| c != '\n')
                        .map(|i| i + offset)
                        .unwrap_or(self.0.len());
                    let ret = &self.0[coffset..found];
                    self.0 = &self.0[found..];
                    return Some(ret);
                }
                c if c.is_whitespace() => {}
                _ => {
                    let found = self.0[offset..]
                        .find(|c: char| c.is_whitespace())
                        .map(|i| i + offset)
                        .unwrap_or(self.0.len());
                    let ret = &self.0[coffset..found];
                    self.0 = &self.0[found..];
                    return Some(ret);
                }
            }
        }
        self.0 = "";
        None
    }
}

struct FmtDocComment(String);

#[derive(PartialEq, Eq, Clone, Copy)]
enum DocState {
    Start,
    LineStart,
    LineFresh,
    Normal,
}

impl Display for FmtDocComment {
    // why do I keep doing this? achjo
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.0.trim();

        if str.is_empty() {
            return Ok(());
        }

        fn subslice_expand<'a>(str: &'a str, subslice: &'a str) -> &'a str {
            let offset: usize = unsafe { subslice.as_ptr().offset_from(str.as_ptr()) }
                .try_into()
                .unwrap();
            &str[offset..]
        }

        fn make_iter<'a>(str: &'a str) -> impl Iterator<Item = &'a str> + Clone {
            WordSplit(str)
                .filter(|&s| !matches!(s, "///" | "/**" | "*/" | "\\ref" | "\\brief" | "\\note"))
        }

        let mut state = DocState::Start;
        let mut words = make_iter(str);
        let mut bold = false;
        let mut param_arg = false;

        let push =
            |f: &mut std::fmt::Formatter<'_>, c: char, state: &mut DocState| -> std::fmt::Result {
                match c {
                    '\r' => return Ok(()),
                    '\n' => {
                        if *state == DocState::LineStart {
                            write!(f, "///\n")?;
                            return Ok(());
                        }
                        *state = DocState::LineStart;
                    }
                    ' ' => {
                        if *state == DocState::LineStart || *state == DocState::LineFresh {
                            return Ok(());
                        }
                    }
                    _ => {
                        if *state == DocState::Start || *state == DocState::LineStart {
                            write!(f, "/// ")?;
                        }
                        *state = DocState::Normal;
                    }
                }
                f.write_char(c)
            };

        macro_rules! pwrite {
            ($e:expr) => {
                for c in $e.chars() {
                    push(f, c, &mut state)?;
                }
            };
        }

        while let Some(mut word) = words.next() {
            match word {
                "\\code" => {
                    pwrite!("```ignore");
                    continue;
                }
                "\\endcode" => {
                    pwrite!("```");
                    continue;
                }
                "\\param" => {
                    param_arg = true;
                    pwrite!("- ");
                    state = DocState::Normal;
                    continue;
                }
                "\\param[out]" => {
                    param_arg = true;
                    pwrite!("- *out* ");
                    state = DocState::Normal;
                    continue;
                }
                "\\n" => {
                    word = "\n";
                }
                deprecated @ "\\deprecated" => {
                    let expand = subslice_expand(str, deprecated);
                    let end = expand.find('\n').map(|i| i).unwrap_or(expand.len() - 1);
                    writeln!(
                        f,
                        "#[deprecated = {:?}]",
                        &expand[deprecated.len()..end].trim()
                    )?;

                    // "skip" the string we've just written out
                    words = make_iter(&expand[end + 1..]);
                    if state != DocState::Start {
                        state = DocState::LineStart;
                    }
                    continue;
                }
                "\\warning" => {
                    state = DocState::LineStart;
                    bold = true;
                    pwrite!(" **");
                    state = DocState::LineFresh;
                    continue;
                }
                _ if param_arg && !word.chars().all(char::is_whitespace) => {
                    pwrite!("`");
                    pwrite!(word.trim());
                    pwrite!("`");
                    param_arg = false;
                    state = DocState::Normal;
                    continue;
                }
                _ if word.starts_with('#')
                    || (word.len() > 1
                        && word.chars().all(|c| c.is_ascii_uppercase() || c == '_')) =>
                {
                    let raw = word.trim_start_matches('#');
                    pwrite!("`");
                    pwrite!(raw);
                    pwrite!("`");
                    state = DocState::Normal;
                    continue;
                }
                _ => {}
            }

            word = word.trim_end_matches("*/");

            let last = words.clone().next().is_none();
            let all_whitespace = word.chars().all(char::is_whitespace);
            let doublelf = word.ends_with("\n\n");
            let bold_end = bold && (last || doublelf);

            let mut delayed = "";
            if bold_end {
                let end = word
                    .rfind(|c: char| !c.is_whitespace())
                    .map(|i| i + 1)
                    .unwrap_or(0);
                word = &word[..end].trim_end();
                delayed = &word[end..];
            }
            if state == DocState::Normal && !all_whitespace {
                f.write_char(' ')?;
            }
            pwrite!(word);
            if bold_end {
                pwrite!("**");
                pwrite!(delayed);
                bold = false;
            }
        }

        if state != DocState::Start && state != DocState::LineStart {
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn insert_preamble() -> impl CFmt<Writer> {
    Fun::new(|w: &mut Writer| {
        code!(
            w,
            #![allow(non_upper_case_globals)]
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(deprecated)]

            use pumice::{util::ObjectHandle, vk};

            // at this time pumice does not generate type aliases for all function's function pointers so this must be done manually
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

            pub type PFN_vkUnmapMemory =
                unsafe extern "system" fn(device: vk::Device, memory: vk::DeviceMemory);

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

            $("/// Fallback type for function pointers that are not generated\n")
            #[derive(Clone, Copy)]
            #[repr(transparent)]
            pub struct PFN_UNAVAILABLE(unsafe extern "system" fn());

            $("/// VK_VERSION_1_1 or VK_KHR_dedicated_allocation\n")
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
            #[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_dedicated_allocation")))]
            pub type PFN_vkGetBufferMemoryRequirements2KHR = PFN_UNAVAILABLE;

            $("/// VK_VERSION_1_1 or VK_KHR_dedicated_allocation\n")
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
            #[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_dedicated_allocation")))]
            pub type PFN_vkGetImageMemoryRequirements2KHR = PFN_UNAVAILABLE;

            $("/// VK_VERSION_1_1 or VK_KHR_bind_memory2\n")
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

            $("/// VK_VERSION_1_1 or VK_KHR_bind_memory2\n")
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

            $("/// VK_VERSION_1_1 or VK_KHR_get_physical_device_properties2\n")
            #[cfg(feature = "VK_VERSION_1_1")]
            pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = unsafe extern "system" fn(
                physical_device: vk::PhysicalDevice,
                p_memory_properties: *mut vk::PhysicalDeviceMemoryProperties2,
            );
            #[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_get_physical_device_properties2"))]
            pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = unsafe extern "system" fn(
                physical_device: vk::PhysicalDevice,
                p_memory_properties: *mut vk::PhysicalDeviceMemoryProperties2KHR,
            );
            #[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_get_physical_device_properties2")))]
            pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = PFN_UNAVAILABLE;

            $("/// VK_VERSION_1_3 or (VK_VERSION_1_1 and VK_KHR_maintenance4)\n")
            #[cfg(feature = "VK_VERSION_1_3")]
            pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
                device: vk::Device,
                p_info: *const vk::DeviceBufferMemoryRequirements,
                p_memory_requirements: *mut vk::MemoryRequirements2,
            );
            #[cfg(all(not(feature = "VK_VERSION_1_3"), all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4")))]
            pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
                device: vk::Device,
                p_info: *const vk::DeviceBufferMemoryRequirementsKHR,
                p_memory_requirements: *mut vk::MemoryRequirements2KHR,
            );
            #[cfg(all(not(feature = "VK_VERSION_1_3"), not(all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4"))))]
            pub type PFN_vkGetDeviceBufferMemoryRequirements = PFN_UNAVAILABLE;

            $("/// VK_VERSION_1_3 or (VK_VERSION_1_1 and VK_KHR_maintenance4)\n")
            #[cfg(feature = "VK_VERSION_1_3")]
            pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
                device: vk::Device,
                p_info: *const vk::DeviceImageMemoryRequirements,
                p_memory_requirements: *mut vk::MemoryRequirements2,
            );
            #[cfg(all(not(feature = "VK_VERSION_1_3"), all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4")))]
            pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
                device: vk::Device,
                p_info: *const vk::DeviceImageMemoryRequirementsKHR,
                p_memory_requirements: *mut vk::MemoryRequirements2KHR,
            );
            #[cfg(all(not(feature = "VK_VERSION_1_3"), not(all(feature = "VK_VERSION_1_1", feature = "VK_KHR_maintenance4"))))]
            pub type PFN_vkGetDeviceImageMemoryRequirements = PFN_UNAVAILABLE;

            pub type PFN_vmaAllocateDeviceMemoryFunction = unsafe extern "C" fn(
                allocator: Allocator,
                memory_type: u32,
                memory: vk::DeviceMemory,
                size: u64
            );

            pub type PFN_vmaFreeDeviceMemoryFunction = unsafe extern "C" fn(
                allocator: Allocator,
                memory_type: u32,
                memory: vk::DeviceMemory,
                size: u64
            );

            #[cfg(feature = "VK_VERSION_1_1")]
            pub type ExternalMemoryHandleTypeFlags = vk::ExternalMemoryHandleTypeFlags;
            #[cfg(all(not(feature = "VK_VERSION_1_1"), feature = "VK_KHR_external_memory_capabilities"))]
            pub type ExternalMemoryHandleTypeFlags = vk::ExternalMemoryHandleTypeFlagsKHR;
            #[cfg(all(not(feature = "VK_VERSION_1_1"), not(feature = "VK_KHR_external_memory_capabilities")))]
            pub type ExternalMemoryHandleTypeFlags = u32;

            macro_rules! non_dispatchable_handle {
                (~$name:ident) => {
                    #[repr(transparent)]
                    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
                    pub struct ~$name(pub u64);

                    impl ~$name {
                        pub const fn null() -> Self {
                            Self(0)
                        }
                    }

                    impl std::fmt::Pointer for ~$name {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            write!(f, "0x{:x}", self.0)
                        }
                    }

                    impl std::fmt::Debug for ~$name {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            write!(f, "0x{:x}", self.0)
                        }
                    }
                };
            }

            macro_rules! dispatchable_handle {
                (~$name:ident) => {
                    #[repr(transparent)]
                    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
                    pub struct ~$name(pub *mut ());

                    impl ~$name {
                        pub const fn null() -> Self {
                            Self(std::ptr::null_mut())
                        }
                    }

                    impl Default for ~$name {
                        fn default() -> Self {
                            Self(std::ptr::null_mut())
                        }
                    }

                    unsafe impl Send for ~$name {}
                    unsafe impl Sync for ~$name {}

                    impl std::fmt::Pointer for ~$name {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            std::fmt::Pointer::fmt(&self.0, f)
                        }
                    }

                    impl std::fmt::Debug for ~$name {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            std::fmt::Debug::fmt(&self.0, f)
                        }
                    }
                };
            }

            macro_rules! dispatchable_handle_eh {
                (~$name:ident) => {
                    #[repr(transparent)]
                    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
                    pub struct ~$name(pub *mut ());

                    impl ~$name {
                        pub const fn null() -> Self {
                            Self(std::ptr::null_mut())
                        }
                    }

                    unsafe impl Send for ~$name {}
                    unsafe impl Sync for ~$name {}

                    impl std::fmt::Pointer for ~$name {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            std::fmt::Pointer::fmt(&self.0, f)
                        }
                    }

                    impl std::fmt::Debug for ~$name {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            std::fmt::Debug::fmt(&self.0, f)
                        }
                    }
                };
            }

            macro_rules! dispatchable_handle_drop {
                (~$name:ident) => {
                    #[repr(transparent)]
                    #[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
                    pub struct ~$name(pub *mut ());

                    impl ~$name {
                        /// Creates a copy of this pointer, this is unsafe because the type implements Drop
                        /// which runs the ffi destroy function, care must be taken that the type is not duplicated.
                        pub unsafe fn unsafe_copy(&self) -> std::mem::ManuallyDrop<Self> {
                            std::mem::ManuallyDrop::new(Self(self.0))
                        }
                    }

                    unsafe impl Send for ~$name {}
                    unsafe impl Sync for ~$name {}

                    impl std::fmt::Pointer for ~$name {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            std::fmt::Pointer::fmt(&self.0, f)
                        }
                    }

                    impl std::fmt::Debug for ~$name {
                        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            std::fmt::Debug::fmt(&self.0, f)
                        }
                    }
                };
            }

            dispatchable_handle_eh!{ Allocator }

            dispatchable_handle!{ Pool }
            dispatchable_handle!{ Allocation }
            dispatchable_handle!{ DefragmentationContext }
            dispatchable_handle_drop!{ VirtualBlock }

            non_dispatchable_handle!{ VirtualAllocation }
            non_dispatchable_handle!{ AllocHandle }

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

        )
    })
}
