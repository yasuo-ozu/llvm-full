//! # llvm-full
//!
//! Provides prebuilt LLVM installations from
//! [llvm-full](https://github.com/yasuo-ozu/llvm-full) releases.
//!
//! This crate's build script downloads and extracts the correct LLVM archive
//! for your target platform. Downstream `-sys` crates can use the
//! `DEP_LLVM_FULL_ROOT` environment variable to locate the LLVM installation.
//!
//! ## Version Selection
//!
//! Enable exactly one feature flag to select the LLVM version:
//!
//! ```toml
//! [build-dependencies]
//! llvm-full = { version = "0.1", features = ["llvm18-1"] }
//! ```
//!
//! Available features: `llvm10-0`, `llvm11-0`, `llvm12-0`, `llvm13-0`,
//! `llvm14-0`, `llvm15-0`, `llvm16-0`, `llvm17-0`, `llvm18-1`, `llvm19-1`,
//! `llvm20-1`, `llvm21-1`, `llvm22-1`.
//!
//! Alternatively, set `LLVM_FULL_PREFIX` to skip download and use an existing
//! LLVM installation.

use std::path::{Path, PathBuf};

/// Returns the LLVM installation prefix set by the build script.
///
/// Resolution order:
/// 1. `DEP_LLVM_FULL_ROOT` (set for downstream crates using this as a build-dependency)
/// 2. `LLVM_FULL_PREFIX` runtime environment variable
/// 3. `LLVM_FULL_PREFIX` compile-time value (set by this crate's build script)
pub fn llvm_prefix() -> PathBuf {
    if let Ok(root) = std::env::var("DEP_LLVM_FULL_ROOT") {
        return PathBuf::from(root);
    }
    if let Ok(prefix) = std::env::var("LLVM_FULL_PREFIX") {
        return PathBuf::from(prefix);
    }
    if let Some(prefix) = option_env!("LLVM_FULL_PREFIX") {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llvm_prefix_exists() {
        let prefix = llvm_prefix();
        assert!(
            prefix.exists(),
            "LLVM prefix does not exist: {}",
            prefix.display()
        );
    }

    #[test]
    fn test_llvm_include_dir_has_headers() {
        let include = llvm_include_dir();
        assert!(
            include.join("llvm-c").join("Types.h").exists(),
            "Missing llvm-c/Types.h in {}",
            include.display()
        );
    }

    #[test]
    fn test_verify_installation() {
        assert!(verify_installation(), "LLVM installation verification failed");
    }
}
