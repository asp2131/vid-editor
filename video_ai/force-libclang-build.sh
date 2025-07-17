#!/bin/bash
# force-libclang-build.sh

echo "🔧 Forcing libclang for OpenCV build..."

# Set DYLD_LIBRARY_PATH (macOS specific for dynamic library loading)
export DYLD_LIBRARY_PATH="/opt/homebrew/opt/llvm/lib:$DYLD_LIBRARY_PATH"

# Alternative: Use DYLD_FALLBACK_LIBRARY_PATH which is less restrictive
export DYLD_FALLBACK_LIBRARY_PATH="/opt/homebrew/opt/llvm/lib:$DYLD_FALLBACK_LIBRARY_PATH"

# Force clang-sys to use dynamic linking
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"

# For some M1 Macs, this specific environment variable helps
export CLANG_PATH="/opt/homebrew/opt/llvm/bin/clang"

# Make sure bindgen can find clang headers
export BINDGEN_EXTRA_CLANG_ARGS="-I/opt/homebrew/opt/llvm/include"

# Debug: Check if libclang is actually there
echo "Checking libclang location..."
ls -la /opt/homebrew/opt/llvm/lib/libclang.dylib

# Try building with explicit library path
cd ~/Documents/experiments/vid-editor/video_ai
cargo clean

# Build with all environment variables
echo "Building with forced libclang path..."
DYLD_LIBRARY_PATH="/opt/homebrew/opt/llvm/lib:$DYLD_LIBRARY_PATH" \
LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib" \
cargo build