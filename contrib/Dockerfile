# Cross compilation environment for librespot
# Build the docker image from the root of the project with the following command :
# $ docker build -t librespot-cross -f contrib/Dockerfile .
#
# The resulting image can be used to build librespot for linux x86_64, armhf(with support for armv6hf), armel, mipsel, aarch64
# $ docker run -v /tmp/librespot-build:/build librespot-cross
#
# The compiled binaries will be located in /tmp/librespot-build
#
# If only one architecture is desired, cargo can be invoked directly with the appropriate options :
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --no-default-features --features "alsa-backend"
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features alsa-backend
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --target arm-unknown-linux-gnueabi --no-default-features --features alsa-backend
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --target aarch64-unknown-linux-gnu --no-default-features --features alsa-backend

# $ docker run -v /tmp/librespot-build:/build librespot-cross contrib/docker-build-pi-armv6hf.sh

FROM debian:stretch

RUN dpkg --add-architecture arm64
RUN dpkg --add-architecture armhf
RUN dpkg --add-architecture armel
RUN dpkg --add-architecture mipsel
RUN apt-get update

RUN apt-get install -y curl git build-essential crossbuild-essential-arm64 crossbuild-essential-armel crossbuild-essential-armhf crossbuild-essential-mipsel pkg-config
RUN apt-get install -y libasound2-dev libasound2-dev:arm64 libasound2-dev:armel libasound2-dev:armhf libasound2-dev:mipsel
RUN apt-get install -y libpulse0 libpulse0:arm64 libpulse0:armel libpulse0:armhf

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin/:${PATH}"
RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup target add arm-unknown-linux-gnueabi
RUN rustup target add arm-unknown-linux-gnueabihf
RUN rustup target add mipsel-unknown-linux-gnu

RUN mkdir /.cargo && \
    echo '[target.aarch64-unknown-linux-gnu]\nlinker = "aarch64-linux-gnu-gcc"' > /.cargo/config && \
    echo '[target.arm-unknown-linux-gnueabihf]\nlinker = "arm-linux-gnueabihf-gcc"' >> /.cargo/config && \
    echo '[target.arm-unknown-linux-gnueabi]\nlinker = "arm-linux-gnueabi-gcc"' >> /.cargo/config && \
    echo '[target.mipsel-unknown-linux-gnu]\nlinker = "mipsel-linux-gnu-gcc"' >> /.cargo/config

RUN mkdir /build && \
    mkdir /pi-tools && \
    curl -L https://github.com/raspberrypi/tools/archive/648a6eeb1e3c2b40af4eb34d88941ee0edeb3e9a.tar.gz | tar xz --strip-components 1 -C /pi-tools

ENV CARGO_TARGET_DIR /build
ENV CARGO_HOME /build/cache
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH_aarch64-unknown-linux-gnu=/usr/lib/aarch64-linux-gnu/pkgconfig/
ENV PKG_CONFIG_PATH_arm-unknown-linux-gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig/
ENV PKG_CONFIG_PATH_arm-unknown-linux-gnueabi=/usr/lib/arm-linux-gnueabi/pkgconfig/
ENV PKG_CONFIG_PATH_mipsel-unknown-linux-gnu=/usr/lib/mipsel-linux-gnu/pkgconfig/

ADD . /src
WORKDIR /src
CMD ["/src/contrib/docker-build.sh"]
