#!/usr/bin/env bash
set -eux

cargo build --release --no-default-features --features alsa-backend
cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features alsa-backend
cargo build --release --target arm-unknown-linux-gnueabi --no-default-features --features alsa-backend
