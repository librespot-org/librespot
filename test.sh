#!/bin/bash

set -e

# this script runs the tests and checks that also run as part of the`test.yml` github action workflow

cargo fmt --all -- --check
cargo clippy -p librespot-core --no-default-features
cargo clippy -p librespot-core

cargo hack clippy --each-feature -p librespot-discovery
cargo hack clippy --each-feature -p librespot-playback
cargo hack clippy --each-feature

cargo build --workspace --examples
cargo test --workspace
cargo check -p librespot-core --no-default-features
cargo check -p librespot-core
cargo hack check --no-dev-deps --each-feature -p librespot-discovery
cargo hack check --no-dev-deps --each-feature -p librespot-playback
cargo hack check --no-dev-deps --each-feature
