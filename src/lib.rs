//! # llvm-full
//!
//! Provides prebuilt LLVM installations from
//! [llvm-full](https://github.com/yasuo-ozu/llvm-full) releases.
//!
//! This crate's build script downloads and extracts the correct LLVM archive
//! for your target platform. Downstream `-sys` crates can use the
//! `DEP_LLVM_FULL_ROOT` environment variable to locate the LLVM installation.
//!
//! ## Environment Variables
//!
//! - `LLVM_FULL_VERSION` (required): The LLVM version to download (e.g. `18.1.8`)
//! - `LLVM_FULL_PREFIX` (optional): Skip download and use an existing LLVM installation

use std::path::{Path, PathBuf};

/// Returns the LLVM installation prefix set by the build script.
///
/// This reads from the `DEP_LLVM_FULL_ROOT` environment variable, which is set
/// by this crate's build script when used as a dependency.
///
/// When used directly (not as a dependency), falls back to `LLVM_FULL_PREFIX`.
pub fn llvm_prefix() -> PathBuf {
    if let Ok(root) = std::env::var("DEP_LLVM_FULL_ROOT") {
        return PathBuf::from(root);
    }
    if let Ok(prefix) = std::env::var("LLVM_FULL_PREFIX") {
        return PathBuf::from(prefix);
    }
    panic!("LLVM prefix not found. Use this crate as a build-dependency, or set LLVM_FULL_PREFIX.");
}

/// Returns the path to LLVM's include directory.
pub fn llvm_include_dir() -> PathBuf {
    llvm_prefix().join("include")
}

/// Returns the path to LLVM's library directory.
pub fn llvm_library_dir() -> PathBuf {
    llvm_prefix().join("lib")
}

/// Returns the path to LLVM's bin directory.
pub fn llvm_bin_dir() -> PathBuf {
    llvm_prefix().join("bin")
}

/// Returns the path to `llvm-config` (or `llvm-config.exe` on Windows).
pub fn llvm_config_path() -> PathBuf {
    let bin = llvm_bin_dir();
    if cfg!(windows) {
        bin.join("llvm-config.exe")
    } else {
        bin.join("llvm-config")
    }
}

/// Checks whether the LLVM installation contains the expected C API headers.
pub fn verify_installation() -> bool {
    let include = llvm_include_dir();
    Path::new(&include).join("llvm-c").join("Types.h").exists()
        && Path::new(&include)
            .join("llvm-c")
            .join("TargetMachine.h")
            .exists()
}
