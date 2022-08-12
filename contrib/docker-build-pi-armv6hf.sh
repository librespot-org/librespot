#!/usr/bin/env bash

# largerly inspired by https://github.com/Spotifyd/spotifyd/blob/993336f7/.github/workflows/cd.yml#L109

set -eux

# See https://github.com/raspberrypi/tools/commit/5caa7046
# Since this commit is not (yet) contained in what is downloaded in Dockerfile, we use the target of the symlink directly
PI1_TOOLS_DIR="/pi-tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf"

PI1_LIB_DIRS=(
  "$PI1_TOOLS_DIR/arm-linux-gnueabihf/sysroot/lib"
  "$PI1_TOOLS_DIR/arm-linux-gnueabihf/sysroot/usr/lib"
)
export RUSTFLAGS="-C linker=$PI1_TOOLS_DIR/bin/arm-linux-gnueabihf-gcc ${PI1_LIB_DIRS[@]/#/-L}"

cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features "alsa-backend"
