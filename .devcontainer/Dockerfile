# syntax=docker/dockerfile:1
ARG debian_version=slim-bookworm
ARG rust_version=1.74.0
FROM rust:${rust_version}-${debian_version}

ARG DEBIAN_FRONTEND=noninteractive
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL="sparse"
ENV RUST_BACKTRACE=1
ENV RUSTFLAGS="-D warnings -A renamed-and-removed-lints"


RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        git \
        nano\
        openssh-server  \
        # for rust-analyzer vscode plugin
        pkg-config \
        # developer dependencies
        libunwind-dev \
        libpulse-dev \
        portaudio19-dev \
        libasound2-dev \
        libsdl2-dev \
        gstreamer1.0-dev \
        libgstreamer-plugins-base1.0-dev \
        libavahi-compat-libdnssd-dev && \
    rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt && \
    rustup component add clippy && \
    cargo install cargo-hack
