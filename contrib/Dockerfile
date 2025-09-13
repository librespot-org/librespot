# Cross compilation environment for librespot
# Build the docker image from the root of the project with the following command :
# $ docker build -t librespot-cross -f contrib/Dockerfile .
#
# The resulting image can be used to build librespot for linux x86_64, armhf, armel, aarch64
# $ docker run -v /tmp/librespot-build:/build librespot-cross
#
# The compiled binaries will be located in /tmp/librespot-build
#
# If only one architecture is desired, cargo can be invoked directly with the appropriate options :
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --no-default-features --features "alsa-backend with-libmdns"
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features "alsa-backend with-libmdns"
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --target arm-unknown-linux-gnueabi --no-default-features --features "alsa-backend with-libmdns"
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --target aarch64-unknown-linux-gnu --no-default-features --features "alsa-backend with-libmdns"

FROM debian:bookworm

RUN echo "deb http://deb.debian.org/debian bookworm main" > /etc/apt/sources.list && \
	echo "deb http://deb.debian.org/debian bookworm-updates main" >> /etc/apt/sources.list && \
	echo "deb http://deb.debian.org/debian-security bookworm-security main" >> /etc/apt/sources.list

RUN dpkg --add-architecture arm64 && \
	dpkg --add-architecture armhf && \
	dpkg --add-architecture armel && \
	apt-get update && \
	apt-get install -y \
	build-essential \
	cmake \
	crossbuild-essential-arm64 \
	crossbuild-essential-armel \
	crossbuild-essential-armhf \
	curl \
	git \
	libasound2-dev \
	libasound2-dev:arm64 \
	libasound2-dev:armel \
	libasound2-dev:armhf \
	libclang-dev \
	libpulse0 \
	libpulse0:arm64 \
	libpulse0:armel \
	libpulse0:armhf \
	pkg-config

ENV PATH="/root/.cargo/bin/:${PATH}"
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.85 -y && \
	rustup target add aarch64-unknown-linux-gnu && \
	rustup target add arm-unknown-linux-gnueabi && \
	rustup target add arm-unknown-linux-gnueabihf && \
	cargo install bindgen-cli && \
	mkdir /.cargo && \
	echo '[target.aarch64-unknown-linux-gnu]\nlinker = "aarch64-linux-gnu-gcc"' > /.cargo/config && \
	echo '[target.arm-unknown-linux-gnueabihf]\nlinker = "arm-linux-gnueabihf-gcc"' >> /.cargo/config && \
	echo '[target.arm-unknown-linux-gnueabi]\nlinker = "arm-linux-gnueabi-gcc"' >> /.cargo/config

ENV CARGO_TARGET_DIR=/build
ENV CARGO_HOME=/build/cache
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH_aarch64-unknown-linux-gnu=/usr/lib/aarch64-linux-gnu/pkgconfig/
ENV PKG_CONFIG_PATH_arm-unknown-linux-gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig/
ENV PKG_CONFIG_PATH_arm-unknown-linux-gnueabi=/usr/lib/arm-linux-gnueabi/pkgconfig/

ADD . /src
WORKDIR /src
CMD ["/src/contrib/docker-build.sh"]
