#!/usr/bin/env bash
set -eux

cargo install --force --locked bindgen-cli

PI1_TOOLS_DIR=/pi/tools/arm-bcm2708/arm-linux-gnueabihf
PI1_TOOLS_SYSROOT_DIR=$PI1_TOOLS_DIR/arm-linux-gnueabihf/sysroot

PI1_LIB_DIRS=(
  "$PI1_TOOLS_SYSROOT_DIR/lib"
  "$PI1_TOOLS_SYSROOT_DIR/usr/lib"
  "/sysroot/usr/lib/arm-linux-gnueabihf"
)
export RUSTFLAGS="-C linker=$PI1_TOOLS_DIR/bin/arm-linux-gnueabihf-gcc ${PI1_LIB_DIRS[*]/#/-L}"
export BINDGEN_EXTRA_CLANG_ARGS=--sysroot=$PI1_TOOLS_SYSROOT_DIR

cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features "alsa-backend with-libmdns rustls-tls-native-roots"
