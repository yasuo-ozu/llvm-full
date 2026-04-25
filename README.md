# llvm-full

Prebuilt LLVM archives with full C/C++ API, clang, and LLD for multiple platforms.

## Why llvm-full?

| Feature | llvm-full | apt.llvm.org | Homebrew | Official releases |
|---------|-----------|--------------|----------|-------------------|
| Full LLVM C/C++ API (headers + libs) | Yes | Yes | Yes | Partial |
| `llvm-config` / `llvm-config.exe` | Yes | Yes | Yes | Not on Windows |
| Clang + LLD included | Yes | Separate pkgs | Yes | Yes |
| Windows MSVC prebuilt | Yes | No | No | No |
| Older LLVM versions (10+) | Yes | Limited | Limited | Archives removed |
| Single archive download | Yes | No | No | Yes |
| Works in Docker / cibuildwheel | Yes | Yes | N/A | Manual |

## Supported Versions

| Version | Linux x86_64 | macOS ARM64 | macOS x86_64 | Windows MSVC |
|---------|:---:|:---:|:---:|:---:|
| 22.1.0 | [![][b-22.1.0-linux]][r-22.1.0] | [![][b-22.1.0-macos-arm]][r-22.1.0] | [![][b-22.1.0-macos-x86]][r-22.1.0] | [![][b-22.1.0-win]][r-22.1.0] |
| 21.1.4 | [![][b-21.1.4-linux]][r-21.1.4] | [![][b-21.1.4-macos-arm]][r-21.1.4] | [![][b-21.1.4-macos-x86]][r-21.1.4] | [![][b-21.1.4-win]][r-21.1.4] |
| 20.1.8 | [![][b-20.1.8-linux]][r-20.1.8] | [![][b-20.1.8-macos-arm]][r-20.1.8] | [![][b-20.1.8-macos-x86]][r-20.1.8] | [![][b-20.1.8-win]][r-20.1.8] |
| 19.1.7 | [![][b-19.1.7-linux]][r-19.1.7] | [![][b-19.1.7-macos-arm]][r-19.1.7] | [![][b-19.1.7-macos-x86]][r-19.1.7] | [![][b-19.1.7-win]][r-19.1.7] |
| 18.1.8 | [![][b-18.1.8-linux]][r-18.1.8] | [![][b-18.1.8-macos-arm]][r-18.1.8] | [![][b-18.1.8-macos-x86]][r-18.1.8] | [![][b-18.1.8-win]][r-18.1.8] |
| 17.0.6 | [![][b-17.0.6-linux]][r-17.0.6] | [![][b-17.0.6-macos-arm]][r-17.0.6] | [![][b-17.0.6-macos-x86]][r-17.0.6] | [![][b-17.0.6-win]][r-17.0.6] |
| 16.0.6 | [![][b-16.0.6-linux]][r-16.0.6] | [![][b-16.0.6-macos-arm]][r-16.0.6] | [![][b-16.0.6-macos-x86]][r-16.0.6] | [![][b-16.0.6-win]][r-16.0.6] |
| 15.0.7 | [![][b-15.0.7-linux]][r-15.0.7] | [![][b-15.0.7-macos-arm]][r-15.0.7] | [![][b-15.0.7-macos-x86]][r-15.0.7] | [![][b-15.0.7-win]][r-15.0.7] |
| 14.0.6 | [![][b-14.0.6-linux]][r-14.0.6] | [![][b-14.0.6-macos-arm]][r-14.0.6] | [![][b-14.0.6-macos-x86]][r-14.0.6] | [![][b-14.0.6-win]][r-14.0.6] |
| 13.0.1 | [![][b-13.0.1-linux]][r-13.0.1] | [![][b-13.0.1-macos-arm]][r-13.0.1] | [![][b-13.0.1-macos-x86]][r-13.0.1] | [![][b-13.0.1-win]][r-13.0.1] |
| 12.0.1 | [![][b-12.0.1-linux]][r-12.0.1] | [![][b-12.0.1-macos-arm]][r-12.0.1] | [![][b-12.0.1-macos-x86]][r-12.0.1] | [![][b-12.0.1-win]][r-12.0.1] |
| 11.0.1 | [![][b-11.0.1-linux]][r-11.0.1] | [![][b-11.0.1-macos-arm]][r-11.0.1] | [![][b-11.0.1-macos-x86]][r-11.0.1] | [![][b-11.0.1-win]][r-11.0.1] |
| 10.0.1 | [![][b-10.0.1-linux]][r-10.0.1] | [![][b-10.0.1-macos-arm]][r-10.0.1] | [![][b-10.0.1-macos-x86]][r-10.0.1] | [![][b-10.0.1-win]][r-10.0.1] |

## Usage

### GitHub Actions

```yaml
steps:
  - uses: yasuo-ozu/llvm-full@main
    with:
      version: "18.1.8"
```

This first tries to download a prebuilt archive from this project's releases. If unavailable, it falls back to building from source (with caching) or installing via apt/brew.

**Environment variables set:**
- `LLVM_PREFIX` - LLVM installation root
- `LLVM_CONFIG` - Path to `llvm-config`
- `LIBCLANG_PATH` - Path to libclang
- `LLVM_INCLUDE_DIR` / `LLVM_LIBRARY_DIR` - Include and library directories
- `LLVM_SYS_<MAJMIN>_PREFIX` - For Rust `llvm-sys` crate

### Shell Script (Docker / cibuildwheel)

```bash
curl -sSL https://raw.githubusercontent.com/yasuo-ozu/llvm-full/main/install-llvm.sh | bash -s -- 18.1.8
```

Or with a custom install prefix:

```bash
curl -sSL https://raw.githubusercontent.com/yasuo-ozu/llvm-full/main/install-llvm.sh | bash -s -- 18.1.8 /usr/local
```

### Rust Crate

Add to your `Cargo.toml`:

```toml
[build-dependencies]
llvm-full = "0.1"
```

Set the desired LLVM version via environment variable:

```bash
LLVM_FULL_VERSION=18.1.8 cargo build
```

The build script automatically downloads and extracts the correct LLVM archive for your target platform, and sets up `cargo:rustc-link-search` and `DEP_LLVM_FULL_ROOT` for downstream crates.

You can also use it as a git dependency:

```toml
[build-dependencies]
llvm-full = { git = "https://github.com/yasuo-ozu/llvm-full" }
```

### Direct Download

Archives are available from [GitHub Releases](https://github.com/yasuo-ozu/llvm-full/releases):

```
https://github.com/yasuo-ozu/llvm-full/releases/download/v{VERSION}/llvm-{VERSION}-{TARGET}.{EXT}
```

Targets: `linux-x86_64`, `macos-aarch64`, `macos-x86_64`, `windows-msvc`

## Archive Contents

Each archive contains a complete LLVM installation:

```
bin/           # clang, lld, llvm-config, llvm-ar, ...
include/       # Full LLVM/Clang C and C++ headers
  clang/
  clang-c/
  llvm/
  llvm-c/
lib/           # Static/shared libraries, cmake modules
  cmake/
  clang/
share/         # Documentation, scan-build, etc.
```

## Building a Release

To trigger a build for LLVM version XX.Y.Z, push a branch named `bXX.Y.Z`:

```bash
git checkout -b b18.1.8
git push origin b18.1.8
```

The CI workflow will build LLVM from source for all targets, create archives, and upload them to GitHub Releases tagged `v18.1.8`.

## License

LLVM is distributed under the [Apache License 2.0 with LLVM Exceptions](https://llvm.org/LICENSE.txt).

<!-- Badge images -->
[b-22.1.0-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-22.1.0-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-22.1.0-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-22.1.0-win]: https://img.shields.io/badge/download-zip-blue
[r-22.1.0]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v22.1.0

[b-21.1.4-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-21.1.4-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-21.1.4-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-21.1.4-win]: https://img.shields.io/badge/download-zip-blue
[r-21.1.4]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v21.1.4

[b-20.1.8-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-20.1.8-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-20.1.8-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-20.1.8-win]: https://img.shields.io/badge/download-zip-blue
[r-20.1.8]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v20.1.8

[b-19.1.7-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-19.1.7-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-19.1.7-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-19.1.7-win]: https://img.shields.io/badge/download-zip-blue
[r-19.1.7]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v19.1.7

[b-18.1.8-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-18.1.8-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-18.1.8-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-18.1.8-win]: https://img.shields.io/badge/download-zip-blue
[r-18.1.8]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v18.1.8

[b-17.0.6-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-17.0.6-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-17.0.6-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-17.0.6-win]: https://img.shields.io/badge/download-zip-blue
[r-17.0.6]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v17.0.6

[b-16.0.6-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-16.0.6-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-16.0.6-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-16.0.6-win]: https://img.shields.io/badge/download-zip-blue
[r-16.0.6]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v16.0.6

[b-15.0.7-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-15.0.7-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-15.0.7-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-15.0.7-win]: https://img.shields.io/badge/download-zip-blue
[r-15.0.7]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v15.0.7

[b-14.0.6-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-14.0.6-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-14.0.6-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-14.0.6-win]: https://img.shields.io/badge/download-zip-blue
[r-14.0.6]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v14.0.6

[b-13.0.1-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-13.0.1-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-13.0.1-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-13.0.1-win]: https://img.shields.io/badge/download-zip-blue
[r-13.0.1]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v13.0.1

[b-12.0.1-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-12.0.1-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-12.0.1-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-12.0.1-win]: https://img.shields.io/badge/download-zip-blue
[r-12.0.1]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v12.0.1

[b-11.0.1-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-11.0.1-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-11.0.1-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-11.0.1-win]: https://img.shields.io/badge/download-zip-blue
[r-11.0.1]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v11.0.1

[b-10.0.1-linux]: https://img.shields.io/badge/download-tar.xz-blue
[b-10.0.1-macos-arm]: https://img.shields.io/badge/download-tar.xz-blue
[b-10.0.1-macos-x86]: https://img.shields.io/badge/download-tar.xz-blue
[b-10.0.1-win]: https://img.shields.io/badge/download-zip-blue
[r-10.0.1]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v10.0.1
