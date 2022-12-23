/// Functions to configure clang to die when encountering size_t
/// ripped out of bindgen

pub fn get_build_target() -> String {
    // in a buildscript, the target is available as an environment variable
    // https://stackoverflow.com/a/51311222
    rust_to_clang_target(&std::env::var("TARGET").unwrap())
}

// Some architecture triplets are different between rust and libclang, see #1211
// and duplicates.
fn rust_to_clang_target(rust_target: &str) -> String {
    if rust_target.starts_with("aarch64-apple-") {
        let mut clang_target = "arm64-apple-".to_owned();
        clang_target.push_str(rust_target.strip_prefix("aarch64-apple-").unwrap());
        return clang_target;
    } else if rust_target.starts_with("riscv64gc-") {
        let mut clang_target = "riscv64-".to_owned();
        clang_target.push_str(rust_target.strip_prefix("riscv64gc-").unwrap());
        return clang_target;
    }
    rust_target.to_owned()
}

pub fn find_clang_executable(clang_args: &[String]) -> Option<clang_sys::support::Clang> {
    // Remove header include flags so we don't incorrectly promote them to `-isystem` (what is isystem)
    let clang_args_for_clang_sys = {
        let mut vec = Vec::with_capacity(clang_args.len());
        let mut args = clang_args.into_iter();
        while let Some(arg) = args.next() {
            // https://clang.llvm.org/docs/ClangCommandLineReference.html#include-path-management
            // -isystem and -isystem-after are harmless.
            if arg == "-I" || arg == "--include-directory" {
                args.next();
                continue;
            }

            if arg.starts_with("-I") || arg.starts_with("--include-directory=") {
                continue;
            }

            vec.push(arg.to_owned());
        }
        vec
    };

    clang_sys::support::Clang::find(None, &clang_args_for_clang_sys)
}

pub fn get_header_include_paths(cpp: bool, clang: &clang_sys::support::Clang) -> Vec<String> {
    let search_paths = if cpp {
        clang.cpp_search_paths.as_ref()
    } else {
        clang.c_search_paths.as_ref()
    };

    let mut paths = Vec::new();
    for path in search_paths.into_iter().flatten() {
        if let Some(path) = path.as_os_str().to_str() {
            paths.push(path.to_owned());
        }
    }

    paths
}
