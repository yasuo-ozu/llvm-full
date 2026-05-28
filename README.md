# llvm-full

Prebuilt LLVM archives with full C/C++ API, clang, and LLD for multiple platforms.

## Build Status

| Version | Status |
|--------:|--------|
| 22.1.0 | [![Build][b-22.1.0]][w-22.1.0] |
| 21.1.4 | [![Build][b-21.1.4]][w-21.1.4] |
| 20.1.8 | [![Build][b-20.1.8]][w-20.1.8] |
| 19.1.7 | [![Build][b-19.1.7]][w-19.1.7] |
| 18.1.8 | [![Build][b-18.1.8]][w-18.1.8] |
| 17.0.6 | [![Build][b-17.0.6]][w-17.0.6] |
| 16.0.6 | [![Build][b-16.0.6]][w-16.0.6] |
| 15.0.7 | [![Build][b-15.0.7]][w-15.0.7] |
| 14.0.6 | [![Build][b-14.0.6]][w-14.0.6] |
| 13.0.1 | [![Build][b-13.0.1]][w-13.0.1] |
| 12.0.1 | [![Build][b-12.0.1]][w-12.0.1] |
| 11.0.1 | [![Build][b-11.0.1]][w-11.0.1] |
| 10.0.1 | [![Build][b-10.0.1]][w-10.0.1] |

## Why llvm-full?

| Feature | llvm-full | apt.llvm.org | Homebrew | Official releases |
|---------|-----------|--------------|----------|-------------------|
| Full LLVM C/C++ API (headers + libs) | Yes | Yes | Yes | Partial |
| `llvm-config` / `llvm-config.exe` | Yes | Yes | Yes | Not on Windows |
| Clang + LLD included | Yes | Separate pkgs | Yes | Yes |
| Windows MSVC prebuilt | Yes | No | No | No |
| Windows MinGW/GNU prebuilt | Yes | No | No | No |
| Older LLVM versions (10+) | Yes | Limited | Limited | Archives removed |
| Single archive download | Yes | No | No | Yes |
| Works in Docker / cibuildwheel | Yes | Yes | N/A | Manual |

## Supported Versions & Targets

### Linux

| Version | x86_64 | AArch64 | x86_64 (musl) | AArch64 (musl) | i686 |
|---------:|------|------|------|------|------|
| 22.1.0 | [![][dl]][r-22.1.0] [files][f-22.1.0-linux-x86_64] | [![][dl]][r-22.1.0] [files][f-22.1.0-linux-aarch64] | [![][dl]][r-22.1.0] [files][f-22.1.0-linux-x86_64-musl] | [![][dl]][r-22.1.0] [files][f-22.1.0-linux-aarch64-musl] | [![][dl]][r-22.1.0] [files][f-22.1.0-linux-i686] |
| 21.1.4 | [![][dl]][r-21.1.4] [files][f-21.1.4-linux-x86_64] | [![][dl]][r-21.1.4] [files][f-21.1.4-linux-aarch64] | [![][dl]][r-21.1.4] [files][f-21.1.4-linux-x86_64-musl] | [![][dl]][r-21.1.4] [files][f-21.1.4-linux-aarch64-musl] | [![][dl]][r-21.1.4] [files][f-21.1.4-linux-i686] |
| 20.1.8 | [![][dl]][r-20.1.8] [files][f-20.1.8-linux-x86_64] | [![][dl]][r-20.1.8] [files][f-20.1.8-linux-aarch64] | [![][dl]][r-20.1.8] [files][f-20.1.8-linux-x86_64-musl] | [![][dl]][r-20.1.8] [files][f-20.1.8-linux-aarch64-musl] | [![][dl]][r-20.1.8] [files][f-20.1.8-linux-i686] |
| 19.1.7 | [![][dl]][r-19.1.7] [files][f-19.1.7-linux-x86_64] | [![][dl]][r-19.1.7] [files][f-19.1.7-linux-aarch64] | [![][dl]][r-19.1.7] [files][f-19.1.7-linux-x86_64-musl] | [![][dl]][r-19.1.7] [files][f-19.1.7-linux-aarch64-musl] | [![][dl]][r-19.1.7] [files][f-19.1.7-linux-i686] |
| 18.1.8 | [![][dl]][r-18.1.8] [files][f-18.1.8-linux-x86_64] | [![][dl]][r-18.1.8] [files][f-18.1.8-linux-aarch64] | [![][dl]][r-18.1.8] [files][f-18.1.8-linux-x86_64-musl] | [![][dl]][r-18.1.8] [files][f-18.1.8-linux-aarch64-musl] | [![][dl]][r-18.1.8] [files][f-18.1.8-linux-i686] |
| 17.0.6 | [![][dl]][r-17.0.6] [files][f-17.0.6-linux-x86_64] | [![][dl]][r-17.0.6] [files][f-17.0.6-linux-aarch64] | [![][dl]][r-17.0.6] [files][f-17.0.6-linux-x86_64-musl] | [![][dl]][r-17.0.6] [files][f-17.0.6-linux-aarch64-musl] | [![][dl]][r-17.0.6] [files][f-17.0.6-linux-i686] |
| 16.0.6 | [![][dl]][r-16.0.6] [files][f-16.0.6-linux-x86_64] | [![][dl]][r-16.0.6] [files][f-16.0.6-linux-aarch64] | [![][dl]][r-16.0.6] [files][f-16.0.6-linux-x86_64-musl] | [![][dl]][r-16.0.6] [files][f-16.0.6-linux-aarch64-musl] | [![][dl]][r-16.0.6] [files][f-16.0.6-linux-i686] |
| 15.0.7 | [![][dl]][r-15.0.7] [files][f-15.0.7-linux-x86_64] | [![][dl]][r-15.0.7] [files][f-15.0.7-linux-aarch64] | [![][dl]][r-15.0.7] [files][f-15.0.7-linux-x86_64-musl] | [![][dl]][r-15.0.7] [files][f-15.0.7-linux-aarch64-musl] | [![][dl]][r-15.0.7] [files][f-15.0.7-linux-i686] |
| 14.0.6 | [![][dl]][r-14.0.6] [files][f-14.0.6-linux-x86_64] | [![][dl]][r-14.0.6] [files][f-14.0.6-linux-aarch64] | [![][dl]][r-14.0.6] [files][f-14.0.6-linux-x86_64-musl] | [![][dl]][r-14.0.6] [files][f-14.0.6-linux-aarch64-musl] | [![][dl]][r-14.0.6] [files][f-14.0.6-linux-i686] |
| 13.0.1 | [![][dl]][r-13.0.1] [files][f-13.0.1-linux-x86_64] | [![][dl]][r-13.0.1] [files][f-13.0.1-linux-aarch64] | [![][dl]][r-13.0.1] [files][f-13.0.1-linux-x86_64-musl] | [![][dl]][r-13.0.1] [files][f-13.0.1-linux-aarch64-musl] | [![][dl]][r-13.0.1] [files][f-13.0.1-linux-i686] |
| 12.0.1 | [![][dl]][r-12.0.1] [files][f-12.0.1-linux-x86_64] | [![][dl]][r-12.0.1] [files][f-12.0.1-linux-aarch64] | [![][dl]][r-12.0.1] [files][f-12.0.1-linux-x86_64-musl] | [![][dl]][r-12.0.1] [files][f-12.0.1-linux-aarch64-musl] | [![][dl]][r-12.0.1] [files][f-12.0.1-linux-i686] |
| 11.0.1 | [![][dl]][r-11.0.1] [files][f-11.0.1-linux-x86_64] | [![][dl]][r-11.0.1] [files][f-11.0.1-linux-aarch64] | [![][dl]][r-11.0.1] [files][f-11.0.1-linux-x86_64-musl] | [![][dl]][r-11.0.1] [files][f-11.0.1-linux-aarch64-musl] | [![][dl]][r-11.0.1] [files][f-11.0.1-linux-i686] |
| 10.0.1 | [![][dl]][r-10.0.1] [files][f-10.0.1-linux-x86_64] | [![][dl]][r-10.0.1] [files][f-10.0.1-linux-aarch64] | [![][dl]][r-10.0.1] [files][f-10.0.1-linux-x86_64-musl] | [![][dl]][r-10.0.1] [files][f-10.0.1-linux-aarch64-musl] | [![][dl]][r-10.0.1] [files][f-10.0.1-linux-i686] |

### macOS

| Version | ARM64 | x86_64 |
|---------:|------|------|
| 22.1.0 | [![][dl]][r-22.1.0] [files][f-22.1.0-macos-aarch64] | [![][dl]][r-22.1.0] [files][f-22.1.0-macos-x86_64] |
| 21.1.4 | [![][dl]][r-21.1.4] [files][f-21.1.4-macos-aarch64] | [![][dl]][r-21.1.4] [files][f-21.1.4-macos-x86_64] |
| 20.1.8 | [![][dl]][r-20.1.8] [files][f-20.1.8-macos-aarch64] | [![][dl]][r-20.1.8] [files][f-20.1.8-macos-x86_64] |
| 19.1.7 | [![][dl]][r-19.1.7] [files][f-19.1.7-macos-aarch64] | [![][dl]][r-19.1.7] [files][f-19.1.7-macos-x86_64] |
| 18.1.8 | [![][dl]][r-18.1.8] [files][f-18.1.8-macos-aarch64] | [![][dl]][r-18.1.8] [files][f-18.1.8-macos-x86_64] |
| 17.0.6 | [![][dl]][r-17.0.6] [files][f-17.0.6-macos-aarch64] | [![][dl]][r-17.0.6] [files][f-17.0.6-macos-x86_64] |
| 16.0.6 | [![][dl]][r-16.0.6] [files][f-16.0.6-macos-aarch64] | [![][dl]][r-16.0.6] [files][f-16.0.6-macos-x86_64] |
| 15.0.7 | [![][dl]][r-15.0.7] [files][f-15.0.7-macos-aarch64] | [![][dl]][r-15.0.7] [files][f-15.0.7-macos-x86_64] |
| 14.0.6 | [![][dl]][r-14.0.6] [files][f-14.0.6-macos-aarch64] | [![][dl]][r-14.0.6] [files][f-14.0.6-macos-x86_64] |
| 13.0.1 | [![][dl]][r-13.0.1] [files][f-13.0.1-macos-aarch64] | [![][dl]][r-13.0.1] [files][f-13.0.1-macos-x86_64] |
| 12.0.1 | [![][dl]][r-12.0.1] [files][f-12.0.1-macos-aarch64] | [![][dl]][r-12.0.1] [files][f-12.0.1-macos-x86_64] |
| 11.0.1 | [![][dl]][r-11.0.1] [files][f-11.0.1-macos-aarch64] | [![][dl]][r-11.0.1] [files][f-11.0.1-macos-x86_64] |
| 10.0.1 | [![][dl]][r-10.0.1] [files][f-10.0.1-macos-aarch64] | [![][dl]][r-10.0.1] [files][f-10.0.1-macos-x86_64] |

### Windows

| Version | x64 MSVC | x64 GNU | ARM64 MSVC | i686 MSVC | i686 GNU |
|---------:|------|------|------|------|------|
| 22.1.0 | [![][dl]][r-22.1.0] [files][f-22.1.0-windows-msvc] | [![][dl]][r-22.1.0] [files][f-22.1.0-windows-gnu] | [![][dl]][r-22.1.0] [files][f-22.1.0-windows-aarch64-msvc] | [![][dl]][r-22.1.0] [files][f-22.1.0-windows-i686-msvc] | [![][dl]][r-22.1.0] [files][f-22.1.0-windows-i686-gnu] |
| 21.1.4 | [![][dl]][r-21.1.4] [files][f-21.1.4-windows-msvc] | [![][dl]][r-21.1.4] [files][f-21.1.4-windows-gnu] | [![][dl]][r-21.1.4] [files][f-21.1.4-windows-aarch64-msvc] | [![][dl]][r-21.1.4] [files][f-21.1.4-windows-i686-msvc] | [![][dl]][r-21.1.4] [files][f-21.1.4-windows-i686-gnu] |
| 20.1.8 | [![][dl]][r-20.1.8] [files][f-20.1.8-windows-msvc] | [![][dl]][r-20.1.8] [files][f-20.1.8-windows-gnu] | [![][dl]][r-20.1.8] [files][f-20.1.8-windows-aarch64-msvc] | [![][dl]][r-20.1.8] [files][f-20.1.8-windows-i686-msvc] | [![][dl]][r-20.1.8] [files][f-20.1.8-windows-i686-gnu] |
| 19.1.7 | [![][dl]][r-19.1.7] [files][f-19.1.7-windows-msvc] | [![][dl]][r-19.1.7] [files][f-19.1.7-windows-gnu] | [![][dl]][r-19.1.7] [files][f-19.1.7-windows-aarch64-msvc] | [![][dl]][r-19.1.7] [files][f-19.1.7-windows-i686-msvc] | [![][dl]][r-19.1.7] [files][f-19.1.7-windows-i686-gnu] |
| 18.1.8 | [![][dl]][r-18.1.8] [files][f-18.1.8-windows-msvc] | [![][dl]][r-18.1.8] [files][f-18.1.8-windows-gnu] | [![][dl]][r-18.1.8] [files][f-18.1.8-windows-aarch64-msvc] | [![][dl]][r-18.1.8] [files][f-18.1.8-windows-i686-msvc] | [![][dl]][r-18.1.8] [files][f-18.1.8-windows-i686-gnu] |
| 17.0.6 | [![][dl]][r-17.0.6] [files][f-17.0.6-windows-msvc] | [![][dl]][r-17.0.6] [files][f-17.0.6-windows-gnu] | [![][dl]][r-17.0.6] [files][f-17.0.6-windows-aarch64-msvc] | [![][dl]][r-17.0.6] [files][f-17.0.6-windows-i686-msvc] | [![][dl]][r-17.0.6] [files][f-17.0.6-windows-i686-gnu] |
| 16.0.6 | [![][dl]][r-16.0.6] [files][f-16.0.6-windows-msvc] | [![][dl]][r-16.0.6] [files][f-16.0.6-windows-gnu] | [![][dl]][r-16.0.6] [files][f-16.0.6-windows-aarch64-msvc] | [![][dl]][r-16.0.6] [files][f-16.0.6-windows-i686-msvc] | [![][dl]][r-16.0.6] [files][f-16.0.6-windows-i686-gnu] |
| 15.0.7 | [![][dl]][r-15.0.7] [files][f-15.0.7-windows-msvc] | [![][dl]][r-15.0.7] [files][f-15.0.7-windows-gnu] | [![][dl]][r-15.0.7] [files][f-15.0.7-windows-aarch64-msvc] | [![][dl]][r-15.0.7] [files][f-15.0.7-windows-i686-msvc] | [![][dl]][r-15.0.7] [files][f-15.0.7-windows-i686-gnu] |
| 14.0.6 | [![][dl]][r-14.0.6] [files][f-14.0.6-windows-msvc] | [![][dl]][r-14.0.6] [files][f-14.0.6-windows-gnu] | [![][dl]][r-14.0.6] [files][f-14.0.6-windows-aarch64-msvc] | [![][dl]][r-14.0.6] [files][f-14.0.6-windows-i686-msvc] | [![][dl]][r-14.0.6] [files][f-14.0.6-windows-i686-gnu] |
| 13.0.1 | [![][dl]][r-13.0.1] [files][f-13.0.1-windows-msvc] | [![][dl]][r-13.0.1] [files][f-13.0.1-windows-gnu] | [![][dl]][r-13.0.1] [files][f-13.0.1-windows-aarch64-msvc] | [![][dl]][r-13.0.1] [files][f-13.0.1-windows-i686-msvc] | [![][dl]][r-13.0.1] [files][f-13.0.1-windows-i686-gnu] |
| 12.0.1 | [![][dl]][r-12.0.1] [files][f-12.0.1-windows-msvc] | [![][dl]][r-12.0.1] [files][f-12.0.1-windows-gnu] | [![][dl]][r-12.0.1] [files][f-12.0.1-windows-aarch64-msvc] | [![][dl]][r-12.0.1] [files][f-12.0.1-windows-i686-msvc] | [![][dl]][r-12.0.1] [files][f-12.0.1-windows-i686-gnu] |
| 11.0.1 | [![][dl]][r-11.0.1] [files][f-11.0.1-windows-msvc] | [![][dl]][r-11.0.1] [files][f-11.0.1-windows-gnu] | [![][dl]][r-11.0.1] [files][f-11.0.1-windows-aarch64-msvc] | [![][dl]][r-11.0.1] [files][f-11.0.1-windows-i686-msvc] | [![][dl]][r-11.0.1] [files][f-11.0.1-windows-i686-gnu] |
| 10.0.1 | [![][dl]][r-10.0.1] [files][f-10.0.1-windows-msvc] | [![][dl]][r-10.0.1] [files][f-10.0.1-windows-gnu] | [![][dl]][r-10.0.1] [files][f-10.0.1-windows-aarch64-msvc] | [![][dl]][r-10.0.1] [files][f-10.0.1-windows-i686-msvc] | [![][dl]][r-10.0.1] [files][f-10.0.1-windows-i686-gnu] |

### BSD

| Version | FreeBSD x86_64 | FreeBSD i686 | NetBSD x86_64 | NetBSD AArch64 | NetBSD i686 | OpenBSD x86_64 | OpenBSD AArch64 | OpenBSD i686 |
|---------:|------|------|------|------|------|------|------|------|
| [22.1.0][r-22.1.0] | [files][f-22.1.0-freebsd-x86_64] | [files][f-22.1.0-freebsd-i686] | [files][f-22.1.0-netbsd-x86_64] | [files][f-22.1.0-netbsd-aarch64] | [files][f-22.1.0-netbsd-i686] | [files][f-22.1.0-openbsd-x86_64] | [files][f-22.1.0-openbsd-aarch64] | [files][f-22.1.0-openbsd-i686] |
| [21.1.4][r-21.1.4] | [files][f-21.1.4-freebsd-x86_64] | [files][f-21.1.4-freebsd-i686] | [files][f-21.1.4-netbsd-x86_64] | [files][f-21.1.4-netbsd-aarch64] | [files][f-21.1.4-netbsd-i686] | [files][f-21.1.4-openbsd-x86_64] | [files][f-21.1.4-openbsd-aarch64] | [files][f-21.1.4-openbsd-i686] |
| [20.1.8][r-20.1.8] | [files][f-20.1.8-freebsd-x86_64] | [files][f-20.1.8-freebsd-i686] | [files][f-20.1.8-netbsd-x86_64] | [files][f-20.1.8-netbsd-aarch64] | [files][f-20.1.8-netbsd-i686] | [files][f-20.1.8-openbsd-x86_64] | [files][f-20.1.8-openbsd-aarch64] | [files][f-20.1.8-openbsd-i686] |
| [19.1.7][r-19.1.7] | [files][f-19.1.7-freebsd-x86_64] | [files][f-19.1.7-freebsd-i686] | [files][f-19.1.7-netbsd-x86_64] | [files][f-19.1.7-netbsd-aarch64] | [files][f-19.1.7-netbsd-i686] | [files][f-19.1.7-openbsd-x86_64] | [files][f-19.1.7-openbsd-aarch64] | [files][f-19.1.7-openbsd-i686] |
| [18.1.8][r-18.1.8] | [files][f-18.1.8-freebsd-x86_64] | [files][f-18.1.8-freebsd-i686] | [files][f-18.1.8-netbsd-x86_64] | [files][f-18.1.8-netbsd-aarch64] | [files][f-18.1.8-netbsd-i686] | [files][f-18.1.8-openbsd-x86_64] | [files][f-18.1.8-openbsd-aarch64] | [files][f-18.1.8-openbsd-i686] |
| [17.0.6][r-17.0.6] | [files][f-17.0.6-freebsd-x86_64] | [files][f-17.0.6-freebsd-i686] | [files][f-17.0.6-netbsd-x86_64] | [files][f-17.0.6-netbsd-aarch64] | [files][f-17.0.6-netbsd-i686] | [files][f-17.0.6-openbsd-x86_64] | [files][f-17.0.6-openbsd-aarch64] | [files][f-17.0.6-openbsd-i686] |
| [16.0.6][r-16.0.6] | [files][f-16.0.6-freebsd-x86_64] | [files][f-16.0.6-freebsd-i686] | [files][f-16.0.6-netbsd-x86_64] | [files][f-16.0.6-netbsd-aarch64] | [files][f-16.0.6-netbsd-i686] | [files][f-16.0.6-openbsd-x86_64] | [files][f-16.0.6-openbsd-aarch64] | [files][f-16.0.6-openbsd-i686] |
| [15.0.7][r-15.0.7] | [files][f-15.0.7-freebsd-x86_64] | [files][f-15.0.7-freebsd-i686] | [files][f-15.0.7-netbsd-x86_64] | [files][f-15.0.7-netbsd-aarch64] | [files][f-15.0.7-netbsd-i686] | [files][f-15.0.7-openbsd-x86_64] | [files][f-15.0.7-openbsd-aarch64] | [files][f-15.0.7-openbsd-i686] |
| [14.0.6][r-14.0.6] | [files][f-14.0.6-freebsd-x86_64] | [files][f-14.0.6-freebsd-i686] | [files][f-14.0.6-netbsd-x86_64] | [files][f-14.0.6-netbsd-aarch64] | [files][f-14.0.6-netbsd-i686] | [files][f-14.0.6-openbsd-x86_64] | [files][f-14.0.6-openbsd-aarch64] | [files][f-14.0.6-openbsd-i686] |
| [13.0.1][r-13.0.1] | [files][f-13.0.1-freebsd-x86_64] | [files][f-13.0.1-freebsd-i686] | [files][f-13.0.1-netbsd-x86_64] | [files][f-13.0.1-netbsd-aarch64] | [files][f-13.0.1-netbsd-i686] | [files][f-13.0.1-openbsd-x86_64] | [files][f-13.0.1-openbsd-aarch64] | [files][f-13.0.1-openbsd-i686] |
| [12.0.1][r-12.0.1] | [files][f-12.0.1-freebsd-x86_64] | [files][f-12.0.1-freebsd-i686] | [files][f-12.0.1-netbsd-x86_64] | [files][f-12.0.1-netbsd-aarch64] | [files][f-12.0.1-netbsd-i686] | [files][f-12.0.1-openbsd-x86_64] | [files][f-12.0.1-openbsd-aarch64] | [files][f-12.0.1-openbsd-i686] |
| [11.0.1][r-11.0.1] | [files][f-11.0.1-freebsd-x86_64] | [files][f-11.0.1-freebsd-i686] | [files][f-11.0.1-netbsd-x86_64] | [files][f-11.0.1-netbsd-aarch64] | [files][f-11.0.1-netbsd-i686] | [files][f-11.0.1-openbsd-x86_64] | [files][f-11.0.1-openbsd-aarch64] | [files][f-11.0.1-openbsd-i686] |
| [10.0.1][r-10.0.1] | [files][f-10.0.1-freebsd-x86_64] | [files][f-10.0.1-freebsd-i686] | [files][f-10.0.1-netbsd-x86_64] | [files][f-10.0.1-netbsd-aarch64] | [files][f-10.0.1-netbsd-i686] | [files][f-10.0.1-openbsd-x86_64] | [files][f-10.0.1-openbsd-aarch64] | [files][f-10.0.1-openbsd-i686] |

## Usage

### GitHub Actions

```yaml
steps:
  - name: Install LLVM and Clang
    uses: yasuo-ozu/llvm-full@main
    id: llvm
    with:
      version: "18.1.8"
```

This first tries to download a prebuilt archive from this project's releases. If unavailable, it falls back to building from source (with caching) or installing via apt/brew.

#### Inputs

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `version` | Yes | | LLVM version, e.g. `18.1.8` |
| `directory` | No | platform default | Installation directory |
| `target` | No | auto-detected | Target override (see [targets](#supported-targets) below) |
| `env` | No | `false` | Set `CC` and `CXX` to the installed Clang |
| `fallback` | No | `true` | Fall back to apt/brew/source if prebuilt unavailable |

#### Outputs

| Output | Description |
|--------|-------------|
| `version` | The full LLVM version installed |
| `prefix` | The LLVM installation prefix directory |

#### Example with all options

```yaml
steps:
  - name: Install LLVM
    uses: yasuo-ozu/llvm-full@main
    id: llvm
    with:
      version: "18.1.8"
      directory: ${{ runner.temp }}/llvm
      env: true

  - name: Use LLVM
    run: |
      echo "LLVM installed at ${{ steps.llvm.outputs.prefix }}"
      clang --version
```

#### Environment variables set

- `LLVM_PREFIX` / `LLVM_PATH` - LLVM installation root
- `LLVM_CONFIG` / `LLVM_CONFIG_PATH` - Path to `llvm-config`
- `LIBCLANG_PATH` - Path to libclang shared library
- `LLVM_INCLUDE_DIR` / `LLVM_LIBRARY_DIR` - Include and library directories
- `LLVM_SYS_<MAJMIN>_PREFIX` - For Rust `llvm-sys` crate
- `LD_LIBRARY_PATH` (Linux) / `DYLD_LIBRARY_PATH` (macOS) - Library search path
- `CC` / `CXX` - Clang paths (when `env: true`)

### Shell Script (Docker / cibuildwheel)

```bash
curl -sSL https://raw.githubusercontent.com/yasuo-ozu/llvm-full/main/install-llvm.sh | bash -s -- 18.1.8
```

Or with a custom install prefix:

```bash
curl -sSL https://raw.githubusercontent.com/yasuo-ozu/llvm-full/main/install-llvm.sh | bash -s -- 18.1.8 /usr/local
```

### cibuildwheel (GitHub Actions)

Complete example for building Python wheels that depend on LLVM using [cibuildwheel](https://cibuildwheel.pypa.io/):

```yaml
name: Build wheels
on: [push, pull_request]

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-14, windows-latest]
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"

      # Install LLVM on the host for Windows (not containerized)
      - name: Install LLVM (Windows)
        if: runner.os == 'Windows'
        uses: yasuo-ozu/llvm-full@main
        with:
          version: "18.1.8"
          env: true

      - name: Build wheels
        uses: pypa/cibuildwheel@v2
        env:
          # Linux: install inside the manylinux/musllinux container
          CIBW_BEFORE_ALL_LINUX: >
            curl -sSL https://raw.githubusercontent.com/yasuo-ozu/llvm-full/main/install-llvm.sh
            | bash -s -- 18.1.8
          CIBW_ENVIRONMENT_LINUX: >
            LLVM_PREFIX=/opt/llvm
            LLVM_CONFIG=/opt/llvm/bin/llvm-config
            LIBCLANG_PATH=/opt/llvm/lib
            PATH=/opt/llvm/bin:$PATH
          # macOS: install via the shell script
          CIBW_BEFORE_ALL_MACOS: >
            curl -sSL https://raw.githubusercontent.com/yasuo-ozu/llvm-full/main/install-llvm.sh
            | bash -s -- 18.1.8
          CIBW_ENVIRONMENT_MACOS: >
            LLVM_PREFIX=/opt/llvm
            LLVM_CONFIG=/opt/llvm/bin/llvm-config
            LIBCLANG_PATH=/opt/llvm/lib
            PATH=/opt/llvm/bin:$PATH
          # Windows: uses the host-installed LLVM (set by the action above)
```

### Rust Crate

Add to your `Cargo.toml` with a version feature flag:

```toml
[build-dependencies]
llvm-full = { version = "0.1", features = ["llvm18-1"] }
```

Available features: `llvm10-0`, `llvm11-0`, `llvm12-0`, `llvm13-0`, `llvm14-0`,
`llvm15-0`, `llvm16-0`, `llvm17-0`, `llvm18-1`, `llvm19-1`, `llvm20-1`, `llvm21-1`, `llvm22-1`.

The build script automatically downloads and extracts the correct LLVM archive for your target platform, and sets up `cargo:rustc-link-search` and `DEP_LLVM_FULL_ROOT` for downstream crates.

You can also use it as a git dependency:

```toml
[build-dependencies]
llvm-full = { git = "https://github.com/yasuo-ozu/llvm-full", features = ["llvm18-1"] }
```

To skip download and use a pre-installed LLVM, set `LLVM_FULL_PREFIX`:

```bash
LLVM_FULL_PREFIX=/usr/lib/llvm-18 cargo build
```

### Direct Download

Archives are available from [GitHub Releases](https://github.com/yasuo-ozu/llvm-full/releases):

```
https://github.com/yasuo-ozu/llvm-full/releases/download/v{VERSION}/llvm-{VERSION}-{TARGET}.{EXT}
```

#### Supported Targets

| Target | Rust triple | Archive format |
|--------|-------------|----------------|
| `linux-x86_64` | `x86_64-unknown-linux-gnu` | `.tar.xz` |
| `linux-aarch64` | `aarch64-unknown-linux-gnu` | `.tar.xz` |
| `linux-x86_64-musl` | `x86_64-unknown-linux-musl` | `.tar.xz` |
| `linux-aarch64-musl` | `aarch64-unknown-linux-musl` | `.tar.xz` |
| `linux-i686` | `i686-unknown-linux-gnu` | `.tar.xz` |
| `macos-aarch64` | `aarch64-apple-darwin` | `.tar.xz` |
| `macos-x86_64` | `x86_64-apple-darwin` | `.tar.xz` |
| `windows-msvc` | `x86_64-pc-windows-msvc` | `.zip` |
| `windows-gnu` | `x86_64-pc-windows-gnu` | `.zip` |
| `windows-aarch64-msvc` | `aarch64-pc-windows-msvc` | `.zip` |
| `windows-i686-msvc` | `i686-pc-windows-msvc` | `.zip` |
| `windows-i686-gnu` | `i686-pc-windows-gnu` | `.zip` |
| `freebsd-x86_64` | `x86_64-unknown-freebsd` | `.tar.xz` |
| `freebsd-i686` | `i686-unknown-freebsd` | `.tar.xz` |
| `netbsd-x86_64` | `x86_64-unknown-netbsd` | `.tar.xz` |
| `netbsd-aarch64` | `aarch64-unknown-netbsd` | `.tar.xz` |
| `netbsd-i686` | `i686-unknown-netbsd` | `.tar.xz` |
| `openbsd-x86_64` | `x86_64-unknown-openbsd` | `.tar.xz` |
| `openbsd-aarch64` | `aarch64-unknown-openbsd` | `.tar.xz` |
| `openbsd-i686` | `i686-unknown-openbsd` | `.tar.xz` |

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

The CI workflows will build LLVM from source for all targets, create archives, and upload them to GitHub Releases tagged `v18.1.8`.

## License

LLVM is distributed under the [Apache License 2.0 with LLVM Exceptions](https://llvm.org/LICENSE.txt).

<!-- Reference link definitions -->

[dl]: https://img.shields.io/badge/download-blue

[b-22.1.0]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b22.1.0
[b-21.1.4]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b21.1.4
[b-20.1.8]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b20.1.8
[b-19.1.7]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b19.1.7
[b-18.1.8]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b18.1.8
[b-17.0.6]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b17.0.6
[b-16.0.6]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b16.0.6
[b-15.0.7]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b15.0.7
[b-14.0.6]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b14.0.6
[b-13.0.1]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b13.0.1
[b-12.0.1]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b12.0.1
[b-11.0.1]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b11.0.1
[b-10.0.1]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml/badge.svg?branch=b10.0.1
[w-22.1.0]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab22.1.0
[w-21.1.4]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab21.1.4
[w-20.1.8]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab20.1.8
[w-19.1.7]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab19.1.7
[w-18.1.8]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab18.1.8
[w-17.0.6]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab17.0.6
[w-16.0.6]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab16.0.6
[w-15.0.7]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab15.0.7
[w-14.0.6]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab14.0.6
[w-13.0.1]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab13.0.1
[w-12.0.1]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab12.0.1
[w-11.0.1]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab11.0.1
[w-10.0.1]: https://github.com/yasuo-ozu/llvm-full/actions/workflows/build.yml?query=branch%3Ab10.0.1

[r-22.1.0]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v22.1.0
[r-21.1.4]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v21.1.4
[r-20.1.8]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v20.1.8
[r-19.1.7]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v19.1.7
[r-18.1.8]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v18.1.8
[r-17.0.6]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v17.0.6
[r-16.0.6]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v16.0.6
[r-15.0.7]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v15.0.7
[r-14.0.6]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v14.0.6
[r-13.0.1]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v13.0.1
[r-12.0.1]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v12.0.1
[r-11.0.1]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v11.0.1
[r-10.0.1]: https://github.com/yasuo-ozu/llvm-full/releases/tag/v10.0.1

[f-22.1.0-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-linux-x86_64.txt
[f-22.1.0-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-linux-aarch64.txt
[f-22.1.0-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-linux-x86_64-musl.txt
[f-22.1.0-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-linux-aarch64-musl.txt
[f-22.1.0-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-linux-i686.txt
[f-22.1.0-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-macos-aarch64.txt
[f-22.1.0-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-macos-x86_64.txt
[f-22.1.0-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-windows-msvc.txt
[f-22.1.0-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-windows-gnu.txt
[f-22.1.0-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-windows-aarch64-msvc.txt
[f-22.1.0-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-windows-i686-msvc.txt
[f-22.1.0-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-windows-i686-gnu.txt
[f-21.1.4-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-linux-x86_64.txt
[f-21.1.4-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-linux-aarch64.txt
[f-21.1.4-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-linux-x86_64-musl.txt
[f-21.1.4-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-linux-aarch64-musl.txt
[f-21.1.4-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-linux-i686.txt
[f-21.1.4-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-macos-aarch64.txt
[f-21.1.4-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-macos-x86_64.txt
[f-21.1.4-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-windows-msvc.txt
[f-21.1.4-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-windows-gnu.txt
[f-21.1.4-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-windows-aarch64-msvc.txt
[f-21.1.4-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-windows-i686-msvc.txt
[f-21.1.4-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-windows-i686-gnu.txt
[f-20.1.8-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-linux-x86_64.txt
[f-20.1.8-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-linux-aarch64.txt
[f-20.1.8-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-linux-x86_64-musl.txt
[f-20.1.8-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-linux-aarch64-musl.txt
[f-20.1.8-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-linux-i686.txt
[f-20.1.8-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-macos-aarch64.txt
[f-20.1.8-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-macos-x86_64.txt
[f-20.1.8-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-windows-msvc.txt
[f-20.1.8-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-windows-gnu.txt
[f-20.1.8-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-windows-aarch64-msvc.txt
[f-20.1.8-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-windows-i686-msvc.txt
[f-20.1.8-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-windows-i686-gnu.txt
[f-19.1.7-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-linux-x86_64.txt
[f-19.1.7-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-linux-aarch64.txt
[f-19.1.7-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-linux-x86_64-musl.txt
[f-19.1.7-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-linux-aarch64-musl.txt
[f-19.1.7-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-linux-i686.txt
[f-19.1.7-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-macos-aarch64.txt
[f-19.1.7-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-macos-x86_64.txt
[f-19.1.7-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-windows-msvc.txt
[f-19.1.7-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-windows-gnu.txt
[f-19.1.7-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-windows-aarch64-msvc.txt
[f-19.1.7-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-windows-i686-msvc.txt
[f-19.1.7-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-windows-i686-gnu.txt
[f-18.1.8-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-linux-x86_64.txt
[f-18.1.8-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-linux-aarch64.txt
[f-18.1.8-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-linux-x86_64-musl.txt
[f-18.1.8-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-linux-aarch64-musl.txt
[f-18.1.8-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-linux-i686.txt
[f-18.1.8-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-macos-aarch64.txt
[f-18.1.8-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-macos-x86_64.txt
[f-18.1.8-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-windows-msvc.txt
[f-18.1.8-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-windows-gnu.txt
[f-18.1.8-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-windows-aarch64-msvc.txt
[f-18.1.8-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-windows-i686-msvc.txt
[f-18.1.8-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-windows-i686-gnu.txt
[f-17.0.6-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-linux-x86_64.txt
[f-17.0.6-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-linux-aarch64.txt
[f-17.0.6-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-linux-x86_64-musl.txt
[f-17.0.6-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-linux-aarch64-musl.txt
[f-17.0.6-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-linux-i686.txt
[f-17.0.6-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-macos-aarch64.txt
[f-17.0.6-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-macos-x86_64.txt
[f-17.0.6-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-windows-msvc.txt
[f-17.0.6-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-windows-gnu.txt
[f-17.0.6-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-windows-aarch64-msvc.txt
[f-17.0.6-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-windows-i686-msvc.txt
[f-17.0.6-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-windows-i686-gnu.txt
[f-16.0.6-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-linux-x86_64.txt
[f-16.0.6-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-linux-aarch64.txt
[f-16.0.6-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-linux-x86_64-musl.txt
[f-16.0.6-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-linux-aarch64-musl.txt
[f-16.0.6-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-linux-i686.txt
[f-16.0.6-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-macos-aarch64.txt
[f-16.0.6-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-macos-x86_64.txt
[f-16.0.6-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-windows-msvc.txt
[f-16.0.6-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-windows-gnu.txt
[f-16.0.6-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-windows-aarch64-msvc.txt
[f-16.0.6-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-windows-i686-msvc.txt
[f-16.0.6-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-windows-i686-gnu.txt
[f-15.0.7-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-linux-x86_64.txt
[f-15.0.7-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-linux-aarch64.txt
[f-15.0.7-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-linux-x86_64-musl.txt
[f-15.0.7-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-linux-aarch64-musl.txt
[f-15.0.7-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-linux-i686.txt
[f-15.0.7-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-macos-aarch64.txt
[f-15.0.7-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-macos-x86_64.txt
[f-15.0.7-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-windows-msvc.txt
[f-15.0.7-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-windows-gnu.txt
[f-15.0.7-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-windows-aarch64-msvc.txt
[f-15.0.7-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-windows-i686-msvc.txt
[f-15.0.7-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-windows-i686-gnu.txt
[f-14.0.6-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-linux-x86_64.txt
[f-14.0.6-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-linux-aarch64.txt
[f-14.0.6-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-linux-x86_64-musl.txt
[f-14.0.6-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-linux-aarch64-musl.txt
[f-14.0.6-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-linux-i686.txt
[f-14.0.6-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-macos-aarch64.txt
[f-14.0.6-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-macos-x86_64.txt
[f-14.0.6-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-windows-msvc.txt
[f-14.0.6-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-windows-gnu.txt
[f-14.0.6-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-windows-aarch64-msvc.txt
[f-14.0.6-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-windows-i686-msvc.txt
[f-14.0.6-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-windows-i686-gnu.txt
[f-13.0.1-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-linux-x86_64.txt
[f-13.0.1-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-linux-aarch64.txt
[f-13.0.1-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-linux-x86_64-musl.txt
[f-13.0.1-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-linux-aarch64-musl.txt
[f-13.0.1-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-linux-i686.txt
[f-13.0.1-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-macos-aarch64.txt
[f-13.0.1-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-macos-x86_64.txt
[f-13.0.1-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-windows-msvc.txt
[f-13.0.1-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-windows-gnu.txt
[f-13.0.1-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-windows-aarch64-msvc.txt
[f-13.0.1-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-windows-i686-msvc.txt
[f-13.0.1-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-windows-i686-gnu.txt
[f-12.0.1-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-linux-x86_64.txt
[f-12.0.1-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-linux-aarch64.txt
[f-12.0.1-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-linux-x86_64-musl.txt
[f-12.0.1-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-linux-aarch64-musl.txt
[f-12.0.1-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-linux-i686.txt
[f-12.0.1-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-macos-aarch64.txt
[f-12.0.1-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-macos-x86_64.txt
[f-12.0.1-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-windows-msvc.txt
[f-12.0.1-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-windows-gnu.txt
[f-12.0.1-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-windows-aarch64-msvc.txt
[f-12.0.1-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-windows-i686-msvc.txt
[f-12.0.1-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-windows-i686-gnu.txt
[f-11.0.1-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-linux-x86_64.txt
[f-11.0.1-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-linux-aarch64.txt
[f-11.0.1-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-linux-x86_64-musl.txt
[f-11.0.1-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-linux-aarch64-musl.txt
[f-11.0.1-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-linux-i686.txt
[f-11.0.1-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-macos-aarch64.txt
[f-11.0.1-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-macos-x86_64.txt
[f-11.0.1-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-windows-msvc.txt
[f-11.0.1-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-windows-gnu.txt
[f-11.0.1-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-windows-aarch64-msvc.txt
[f-11.0.1-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-windows-i686-msvc.txt
[f-11.0.1-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-windows-i686-gnu.txt
[f-10.0.1-linux-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-linux-x86_64.txt
[f-10.0.1-linux-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-linux-aarch64.txt
[f-10.0.1-linux-x86_64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-linux-x86_64-musl.txt
[f-10.0.1-linux-aarch64-musl]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-linux-aarch64-musl.txt
[f-10.0.1-linux-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-linux-i686.txt
[f-10.0.1-macos-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-macos-aarch64.txt
[f-10.0.1-macos-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-macos-x86_64.txt
[f-10.0.1-windows-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-windows-msvc.txt
[f-10.0.1-windows-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-windows-gnu.txt
[f-10.0.1-windows-aarch64-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-windows-aarch64-msvc.txt
[f-10.0.1-windows-i686-msvc]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-windows-i686-msvc.txt
[f-10.0.1-windows-i686-gnu]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-windows-i686-gnu.txt

[f-22.1.0-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-freebsd-x86_64.txt
[f-22.1.0-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-freebsd-i686.txt
[f-22.1.0-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-netbsd-x86_64.txt
[f-22.1.0-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-netbsd-aarch64.txt
[f-22.1.0-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-netbsd-i686.txt
[f-22.1.0-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-openbsd-x86_64.txt
[f-22.1.0-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-openbsd-aarch64.txt
[f-22.1.0-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b22.1.0/files-v22.1.0-openbsd-i686.txt
[f-21.1.4-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-freebsd-x86_64.txt
[f-21.1.4-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-freebsd-i686.txt
[f-21.1.4-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-netbsd-x86_64.txt
[f-21.1.4-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-netbsd-aarch64.txt
[f-21.1.4-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-netbsd-i686.txt
[f-21.1.4-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-openbsd-x86_64.txt
[f-21.1.4-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-openbsd-aarch64.txt
[f-21.1.4-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b21.1.4/files-v21.1.4-openbsd-i686.txt
[f-20.1.8-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-freebsd-x86_64.txt
[f-20.1.8-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-freebsd-i686.txt
[f-20.1.8-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-netbsd-x86_64.txt
[f-20.1.8-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-netbsd-aarch64.txt
[f-20.1.8-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-netbsd-i686.txt
[f-20.1.8-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-openbsd-x86_64.txt
[f-20.1.8-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-openbsd-aarch64.txt
[f-20.1.8-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b20.1.8/files-v20.1.8-openbsd-i686.txt
[f-19.1.7-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-freebsd-x86_64.txt
[f-19.1.7-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-freebsd-i686.txt
[f-19.1.7-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-netbsd-x86_64.txt
[f-19.1.7-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-netbsd-aarch64.txt
[f-19.1.7-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-netbsd-i686.txt
[f-19.1.7-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-openbsd-x86_64.txt
[f-19.1.7-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-openbsd-aarch64.txt
[f-19.1.7-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b19.1.7/files-v19.1.7-openbsd-i686.txt
[f-18.1.8-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-freebsd-x86_64.txt
[f-18.1.8-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-freebsd-i686.txt
[f-18.1.8-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-netbsd-x86_64.txt
[f-18.1.8-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-netbsd-aarch64.txt
[f-18.1.8-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-netbsd-i686.txt
[f-18.1.8-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-openbsd-x86_64.txt
[f-18.1.8-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-openbsd-aarch64.txt
[f-18.1.8-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b18.1.8/files-v18.1.8-openbsd-i686.txt
[f-17.0.6-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-freebsd-x86_64.txt
[f-17.0.6-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-freebsd-i686.txt
[f-17.0.6-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-netbsd-x86_64.txt
[f-17.0.6-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-netbsd-aarch64.txt
[f-17.0.6-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-netbsd-i686.txt
[f-17.0.6-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-openbsd-x86_64.txt
[f-17.0.6-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-openbsd-aarch64.txt
[f-17.0.6-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b17.0.6/files-v17.0.6-openbsd-i686.txt
[f-16.0.6-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-freebsd-x86_64.txt
[f-16.0.6-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-freebsd-i686.txt
[f-16.0.6-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-netbsd-x86_64.txt
[f-16.0.6-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-netbsd-aarch64.txt
[f-16.0.6-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-netbsd-i686.txt
[f-16.0.6-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-openbsd-x86_64.txt
[f-16.0.6-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-openbsd-aarch64.txt
[f-16.0.6-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b16.0.6/files-v16.0.6-openbsd-i686.txt
[f-15.0.7-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-freebsd-x86_64.txt
[f-15.0.7-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-freebsd-i686.txt
[f-15.0.7-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-netbsd-x86_64.txt
[f-15.0.7-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-netbsd-aarch64.txt
[f-15.0.7-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-netbsd-i686.txt
[f-15.0.7-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-openbsd-x86_64.txt
[f-15.0.7-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-openbsd-aarch64.txt
[f-15.0.7-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b15.0.7/files-v15.0.7-openbsd-i686.txt
[f-14.0.6-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-freebsd-x86_64.txt
[f-14.0.6-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-freebsd-i686.txt
[f-14.0.6-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-netbsd-x86_64.txt
[f-14.0.6-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-netbsd-aarch64.txt
[f-14.0.6-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-netbsd-i686.txt
[f-14.0.6-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-openbsd-x86_64.txt
[f-14.0.6-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-openbsd-aarch64.txt
[f-14.0.6-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b14.0.6/files-v14.0.6-openbsd-i686.txt
[f-13.0.1-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-freebsd-x86_64.txt
[f-13.0.1-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-freebsd-i686.txt
[f-13.0.1-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-netbsd-x86_64.txt
[f-13.0.1-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-netbsd-aarch64.txt
[f-13.0.1-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-netbsd-i686.txt
[f-13.0.1-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-openbsd-x86_64.txt
[f-13.0.1-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-openbsd-aarch64.txt
[f-13.0.1-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b13.0.1/files-v13.0.1-openbsd-i686.txt
[f-12.0.1-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-freebsd-x86_64.txt
[f-12.0.1-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-freebsd-i686.txt
[f-12.0.1-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-netbsd-x86_64.txt
[f-12.0.1-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-netbsd-aarch64.txt
[f-12.0.1-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-netbsd-i686.txt
[f-12.0.1-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-openbsd-x86_64.txt
[f-12.0.1-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-openbsd-aarch64.txt
[f-12.0.1-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b12.0.1/files-v12.0.1-openbsd-i686.txt
[f-11.0.1-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-freebsd-x86_64.txt
[f-11.0.1-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-freebsd-i686.txt
[f-11.0.1-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-netbsd-x86_64.txt
[f-11.0.1-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-netbsd-aarch64.txt
[f-11.0.1-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-netbsd-i686.txt
[f-11.0.1-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-openbsd-x86_64.txt
[f-11.0.1-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-openbsd-aarch64.txt
[f-11.0.1-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b11.0.1/files-v11.0.1-openbsd-i686.txt
[f-10.0.1-freebsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-freebsd-x86_64.txt
[f-10.0.1-freebsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-freebsd-i686.txt
[f-10.0.1-netbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-netbsd-x86_64.txt
[f-10.0.1-netbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-netbsd-aarch64.txt
[f-10.0.1-netbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-netbsd-i686.txt
[f-10.0.1-openbsd-x86_64]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-openbsd-x86_64.txt
[f-10.0.1-openbsd-aarch64]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-openbsd-aarch64.txt
[f-10.0.1-openbsd-i686]: https://github.com/yasuo-ozu/llvm-full/blob/b10.0.1/files-v10.0.1-openbsd-i686.txt
