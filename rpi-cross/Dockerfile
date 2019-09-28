FROM arm32v7/rust:1-stretch

RUN apt-get update && apt-get install -y curl git build-essential libasound2-dev

RUN mkdir /build

ENV CARGO_TARGET_DIR /build
ENV CARGO_HOME /build/cache

ADD . /src
WORKDIR /src
RUN cargo build --release --no-default-features --features "alsa-backend"
