#!/usr/bin/env bash
#
# Install prebuilt LLVM from llvm-full GitHub Releases.
#
# Usage:
#   curl -sSL https://raw.githubusercontent.com/yasuo-ozu/llvm-full/main/install-llvm.sh | bash -s -- <version> [prefix]
#
#   bash install-llvm.sh 18.1.8              # installs to /opt/llvm
#   bash install-llvm.sh 18.1.8 /usr/local   # installs to /usr/local
#
# Environment variables set after install (printed to stdout):
#   LLVM_PREFIX, LLVM_CONFIG, LIBCLANG_PATH, LLVM_INCLUDE_DIR, LLVM_LIBRARY_DIR

set -euo pipefail

VERSION="${1:-}"
PREFIX="${2:-/opt/llvm}"

if [ -z "$VERSION" ]; then
  echo "Usage: $0 <llvm-version> [install-prefix]" >&2
  echo "Example: $0 18.1.8 /opt/llvm" >&2
  exit 1
fi

REPO="yasuo-ozu/llvm-full"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    TARGET="linux-x86_64"
    EXT="tar.xz"
    ;;
  Darwin)
    case "$ARCH" in
      arm64|aarch64)
        TARGET="macos-aarch64"
        ;;
      *)
        TARGET="macos-x86_64"
        ;;
    esac
    EXT="tar.xz"
    ;;
  MINGW*|MSYS*|CYGWIN*)
    TARGET="windows-msvc"
    EXT="zip"
    ;;
  *)
    echo "Unsupported OS: $OS" >&2
    exit 1
    ;;
esac

ARCHIVE_NAME="llvm-${VERSION}-${TARGET}.${EXT}"
ARCHIVE_URL="https://github.com/${REPO}/releases/download/v${VERSION}/${ARCHIVE_NAME}"

echo "Downloading LLVM ${VERSION} for ${TARGET}..."
echo "  URL: ${ARCHIVE_URL}"
echo "  Prefix: ${PREFIX}"

TMPDIR="${TMPDIR:-/tmp}"
ARCHIVE_PATH="${TMPDIR}/${ARCHIVE_NAME}"

if ! curl -fSL --retry 3 --retry-delay 2 -o "$ARCHIVE_PATH" "$ARCHIVE_URL"; then
  echo "Failed to download archive. Check that version ${VERSION} exists for target ${TARGET}." >&2
  echo "Available releases: https://github.com/${REPO}/releases" >&2
  exit 1
fi

mkdir -p "$PREFIX"

echo "Extracting to ${PREFIX}..."
if [ "$EXT" = "zip" ]; then
  if command -v 7z >/dev/null 2>&1; then
    7z x "$ARCHIVE_PATH" -o"$PREFIX" -y
  elif command -v unzip >/dev/null 2>&1; then
    unzip -o "$ARCHIVE_PATH" -d "$PREFIX"
  else
    echo "No zip extractor found (need 7z or unzip)" >&2
    exit 1
  fi
else
  tar xJf "$ARCHIVE_PATH" -C "$PREFIX"
fi

rm -f "$ARCHIVE_PATH"

# Verify
if [ -x "$PREFIX/bin/llvm-config" ] || [ -f "$PREFIX/bin/llvm-config.exe" ]; then
  echo "LLVM ${VERSION} installed successfully to ${PREFIX}"
else
  echo "Warning: llvm-config not found in ${PREFIX}/bin" >&2
fi

echo ""
echo "Set these environment variables to use LLVM:"
echo "  export LLVM_PREFIX=\"${PREFIX}\""
echo "  export LLVM_CONFIG=\"${PREFIX}/bin/llvm-config\""
echo "  export LIBCLANG_PATH=\"${PREFIX}/lib\""
echo "  export LLVM_INCLUDE_DIR=\"${PREFIX}/include\""
echo "  export LLVM_LIBRARY_DIR=\"${PREFIX}/lib\""
echo "  export PATH=\"${PREFIX}/bin:\$PATH\""
