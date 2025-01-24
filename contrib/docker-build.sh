#!/usr/bin/env bash
set -eux

cargo build --release --no-default-features --features "alsa-backend with-libmdns"
cargo build --release --target aarch64-unknown-linux-gnu --no-default-features --features "alsa-backend with-libmdns"
cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features "alsa-backend with-libmdns"
cargo build --release --target arm-unknown-linux-gnueabi --no-default-features --features "alsa-backend with-libmdns"
