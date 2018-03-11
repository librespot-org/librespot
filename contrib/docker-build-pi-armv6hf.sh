#!/usr/bin/env bash

# Snipped and tucked from https://github.com/plietar/librespot/pull/202/commits/21549641d39399cbaec0bc92b36c9951d1b87b90
# and further inputs from https://github.com/kingosticks/librespot/commit/c55dd20bd6c7e44dd75ff33185cf50b2d3bd79c3

set -eux
# Get alsa lib and headers
ALSA_VER="1.0.25-4"
DEPS=( \
  "http://mirrordirector.raspbian.org/raspbian/pool/main/a/alsa-lib/libasound2_${ALSA_VER}_armhf.deb" \
  "http://mirrordirector.raspbian.org/raspbian/pool/main/a/alsa-lib/libasound2-dev_${ALSA_VER}_armhf.deb" \
)

# Collect Paths
SYSROOT="/pi-tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/arm-bcm2708hardfp-linux-gnueabi/sysroot"
GCC="/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin"
GCC_SYSROOT="$GCC/gcc-sysroot"


export PATH=/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/:$PATH

# Link the compiler
export TARGET_CC="$GCC/arm-linux-gnueabihf-gcc"

# Create wrapper around gcc to point to rpi sysroot
echo -e '#!/bin/bash' "\n$TARGET_CC --sysroot $SYSROOT \"\$@\"" > $GCC_SYSROOT
chmod +x $GCC_SYSROOT

# Add extra target dependencies to our rpi sysroot
for path in "${DEPS[@]}"; do
  curl -OL $path
  dpkg -x $(basename $path) $SYSROOT
done

# i don't why this is neccessary
# ln -s ld-linux.so.3 $SYSROOT/lib/ld-linux-armhf.so.3

# point cargo to use gcc wrapper as linker
echo -e '[target.arm-unknown-linux-gnueabihf]\nlinker = "gcc-sysroot"' > /.cargo/config

# Build
cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features "alsa-backend"
