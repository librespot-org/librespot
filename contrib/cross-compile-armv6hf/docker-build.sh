#!/usr/bin/env bash
set -eux

PI1_TOOLS_DIR="/pi/tools/arm-bcm2708/arm-linux-gnueabihf"

PI1_LIB_DIRS=(
  "$PI1_TOOLS_DIR/arm-linux-gnueabihf/sysroot/lib"
  "$PI1_TOOLS_DIR/arm-linux-gnueabihf/sysroot/usr/lib"
  "/sysroot/usr/lib/arm-linux-gnueabihf"
  "/sysroot/lib/arm-linux-gnueabihf"
)
export RUSTFLAGS="-C linker=$PI1_TOOLS_DIR/bin/arm-linux-gnueabihf-gcc ${PI1_LIB_DIRS[*]/#/-L}"

cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features "alsa-backend"
