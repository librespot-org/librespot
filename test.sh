#!/bin/sh

set -e

clean() {
    # some shells will call EXIT after the INT signal
    # causing EXIT trap to be executed, so we trap EXIT after INT
    trap '' EXIT

    cargo clean
}

trap clean INT QUIT TERM EXIT

# this script runs the tests and checks that also run as part of the`test.yml` github action workflow
cargo clean

cargo fmt --all -- --check

cargo hack clippy -p librespot-protocol --each-feature

cargo hack clippy -p librespot --each-feature --exclude-all-features --include-features native-tls --exclude-features rustls-tls-native-roots,rustls-tls-webpki-roots
cargo hack clippy -p librespot --each-feature --exclude-all-features --include-features rustls-tls-native-roots --exclude-features native-tls,rustls-tls-webpki-roots
cargo hack clippy -p librespot --each-feature --exclude-all-features --include-features rustls-tls-webpki-roots --exclude-features native-tls,rustls-tls-native-roots


cargo fetch --locked
cargo build --frozen --workspace --examples
cargo test --workspace

cargo hack check -p librespot-protocol --each-feature
cargo hack check -p librespot --each-feature --exclude-all-features --include-features native-tls --exclude-features rustls-tls-native-roots,rustls-tls-webpki-roots
cargo hack check -p librespot --each-feature --exclude-all-features --include-features rustls-tls-native-roots --exclude-features native-tls,rustls-tls-webpki-roots
run: cargo build --frozen
