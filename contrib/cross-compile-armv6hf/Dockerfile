# Cross compilation environment for librespot in armv6hf.
# Build the docker image from the root of the project with the following command:
# $ docker build -t librespot-cross-armv6hf -f contrib/cross-compile-armv6hf/Dockerfile .
#
# The resulting image can be used to build librespot for armv6hf:
# $ docker run -v /tmp/librespot-build-armv6hf:/build librespot-cross-armv6hf
#
# The compiled binary will be located in /tmp/librespot-build-armv6hf/arm-unknown-linux-gnueabihf/release/librespot

FROM --platform=linux/amd64 ubuntu:18.04

# Install common packages.
RUN apt-get update
RUN apt-get install -y -qq git curl build-essential libasound2-dev libssl-dev libpulse-dev libdbus-1-dev

# Install armhf packages.
RUN echo "deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ bionic main" | tee -a /etc/apt/sources.list
RUN apt-get update
RUN apt-get download libasound2:armhf libasound2-dev:armhf libssl-dev:armhf libssl1.1:armhf
RUN mkdir /sysroot && \
    dpkg -x libasound2_*.deb /sysroot/ && \
    dpkg -x libssl-dev*.deb /sysroot/ && \
    dpkg -x libssl1.1*.deb /sysroot/ && \
    dpkg -x libasound2-dev*.deb /sysroot/

# Install rust.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin/:${PATH}"
RUN rustup target add arm-unknown-linux-gnueabihf
RUN mkdir /.cargo && \
    echo '[target.arm-unknown-linux-gnueabihf]\nlinker = "arm-linux-gnueabihf-gcc"' >> /.cargo/config

# Install Pi tools for armv6.
RUN mkdir /pi && \
    git -C /pi clone --depth=1 https://github.com/raspberrypi/tools.git

# Build env variables.
ENV CARGO_TARGET_DIR /build
ENV CARGO_HOME /build/cache
ENV PATH="/pi/tools/arm-bcm2708/arm-linux-gnueabihf/bin:${PATH}"
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH_arm-unknown-linux-gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig/
ENV C_INCLUDE_PATH=/sysroot/usr/include
ENV OPENSSL_LIB_DIR=/sysroot/usr/lib/arm-linux-gnueabihf
ENV OPENSSL_INCLUDE_DIR=/sysroot/usr/include/arm-linux-gnueabihf

ADD . /src
WORKDIR /src
CMD ["/src/contrib/cross-compile-armv6hf/docker-build.sh"]
