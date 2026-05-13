use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

const REPO: &str = "yasuo-ozu/llvm-full";

const VERSION_TABLE: &[(&str, &str)] = &[
    ("CARGO_FEATURE_LLVM10_0", "10.0.1"),
    ("CARGO_FEATURE_LLVM11_0", "11.0.1"),
    ("CARGO_FEATURE_LLVM12_0", "12.0.1"),
    ("CARGO_FEATURE_LLVM13_0", "13.0.1"),
    ("CARGO_FEATURE_LLVM14_0", "14.0.6"),
    ("CARGO_FEATURE_LLVM15_0", "15.0.7"),
    ("CARGO_FEATURE_LLVM16_0", "16.0.6"),
    ("CARGO_FEATURE_LLVM17_0", "17.0.6"),
    ("CARGO_FEATURE_LLVM18_1", "18.1.8"),
    ("CARGO_FEATURE_LLVM19_1", "19.1.7"),
    ("CARGO_FEATURE_LLVM20_1", "20.1.8"),
    ("CARGO_FEATURE_LLVM21_1", "21.1.4"),
    ("CARGO_FEATURE_LLVM22_1", "22.1.0"),
];

fn version_from_features() -> Option<&'static str> {
    let mut selected = None;
    for &(env_key, version) in VERSION_TABLE {
        if env::var(env_key).is_ok() {
            if selected.is_some() {
                panic!(
                    "Multiple LLVM version features enabled. Enable exactly one llvmXX-Y feature."
                );
            }
            selected = Some(version);
        }
    }
    selected
}

fn target_name() -> Option<(&'static str, &'static str)> {
    let target_os = env::var("CARGO_CFG_TARGET_OS").ok()?;
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").ok()?;
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    match (target_os.as_str(), target_arch.as_str(), target_env.as_str()) {
        // Linux glibc
        ("linux", "x86_64", "gnu") | ("linux", "x86_64", "") => Some(("linux-x86_64", "tar.xz")),
        ("linux", "aarch64", "gnu") | ("linux", "aarch64", "") => {
            Some(("linux-aarch64", "tar.xz"))
        }
        ("linux", "x86", "gnu") | ("linux", "x86", "") => Some(("linux-i686", "tar.xz")),
        // Linux musl
        ("linux", "x86_64", "musl") => Some(("linux-x86_64-musl", "tar.xz")),
        ("linux", "aarch64", "musl") => Some(("linux-aarch64-musl", "tar.xz")),
        // macOS
        ("macos", "aarch64", _) => Some(("macos-aarch64", "tar.xz")),
        ("macos", "x86_64", _) => Some(("macos-x86_64", "tar.xz")),
        // Windows MSVC
        ("windows", "x86_64", "msvc") => Some(("windows-msvc", "zip")),
        ("windows", "aarch64", "msvc") => Some(("windows-aarch64-msvc", "zip")),
        ("windows", "x86", "msvc") => Some(("windows-i686-msvc", "zip")),
        // Windows GNU
        ("windows", "x86_64", "gnu") => Some(("windows-gnu", "zip")),
        ("windows", "x86", "gnu") => Some(("windows-i686-gnu", "zip")),
        _ => None,
    }
}

fn download(url: &str) -> Result<Vec<u8>, String> {
    eprintln!("Downloading {url}");
    let resp = ureq::get(url)
        .call()
        .map_err(|e| format!("HTTP request failed: {e}"))?;
    let mut body = Vec::new();
    resp.into_reader()
        .read_to_end(&mut body)
        .map_err(|e| format!("Failed to read response: {e}"))?;
    Ok(body)
}

fn extract_tar_xz(data: &[u8], dest: &Path) -> Result<(), String> {
    let xz_reader = xz2::read::XzDecoder::new(data);
    let mut archive = tar::Archive::new(xz_reader);
    archive
        .unpack(dest)
        .map_err(|e| format!("Failed to extract tar.xz: {e}"))?;
    Ok(())
}

fn extract_zip(data: &[u8], dest: &Path) -> Result<(), String> {
    let cursor = io::Cursor::new(data);
    let mut archive =
        zip::ZipArchive::new(cursor).map_err(|e| format!("Failed to open zip: {e}"))?;
    archive
        .extract(dest)
        .map_err(|e| format!("Failed to extract zip: {e}"))?;
    Ok(())
}

fn main() {
    println!("cargo:rerun-if-env-changed=LLVM_FULL_PREFIX");

    // If LLVM_FULL_PREFIX is set, use that directly
    if let Ok(prefix) = env::var("LLVM_FULL_PREFIX") {
        let prefix_path = PathBuf::from(&prefix);
        emit_metadata(&prefix_path);
        return;
    }

    let version = version_from_features().unwrap_or_else(|| {
        panic!(
            "No LLVM version feature enabled. Enable exactly one of: \
             llvm10-0, llvm11-0, llvm12-0, llvm13-0, llvm14-0, llvm15-0, llvm16-0, \
             llvm17-0, llvm18-1, llvm19-1, llvm20-1, llvm21-1, llvm22-1"
        );
    });

    let (target, ext) = target_name().unwrap_or_else(|| {
        panic!(
            "Unsupported target: {}-{}-{}",
            env::var("CARGO_CFG_TARGET_OS").unwrap_or_default(),
            env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default(),
            env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default(),
        );
    });

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let llvm_dir = out_dir.join(format!("llvm-{version}"));

    // Check if already extracted
    if llvm_dir
        .join("include")
        .join("llvm-c")
        .join("Types.h")
        .exists()
    {
        eprintln!(
            "LLVM {version} already extracted at {}",
            llvm_dir.display()
        );
        emit_metadata(&llvm_dir);
        return;
    }

    let archive_name = format!("llvm-{version}-{target}.{ext}");
    let url = format!("https://github.com/{REPO}/releases/download/v{version}/{archive_name}");

    let data = download(&url).unwrap_or_else(|e| {
        panic!("Failed to download LLVM {version} for {target}: {e}");
    });

    // Clean and recreate
    let _ = fs::remove_dir_all(&llvm_dir);
    fs::create_dir_all(&llvm_dir).expect("Failed to create LLVM directory");

    match ext {
        "tar.xz" => extract_tar_xz(&data, &llvm_dir).expect("Failed to extract"),
        "zip" => extract_zip(&data, &llvm_dir).expect("Failed to extract"),
        _ => unreachable!(),
    }

    emit_metadata(&llvm_dir);
}

fn emit_metadata(llvm_dir: &Path) {
    let llvm_dir_str = llvm_dir.display();
    println!("cargo:root={llvm_dir_str}");
    println!("cargo:rustc-link-search=native={llvm_dir_str}/lib");
    println!("cargo:include={llvm_dir_str}/include");
    println!("cargo:rustc-env=LLVM_FULL_PREFIX={llvm_dir_str}");

    // On macOS, add RPATH so the runtime linker can find LLVM shared libraries
    // (e.g. libunwind.1.dylib) that get pulled in via the link search path.
    // Without this, @rpath references in those dylibs can't be resolved.
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{llvm_dir_str}/lib");
    }

    // Parse version from directory name or llvm-config if available
    if let Some(name) = llvm_dir.file_name().and_then(|n| n.to_str()) {
        if let Some(version) = name.strip_prefix("llvm-") {
            println!("cargo:version={version}");
        }
    }
}
