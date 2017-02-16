# Cross compilation environment for librespot
# Build the docker image from the root of the project with the following command :
# $ docker build -t librespot-cross -f contrib/Dockerfile .
#
# The resulting image can be used to build librespot for linux x86_64, armhf and armel.
# $ docker run -v /tmp/librespot-build:/build librespot-cross
#
# The compiled binaries will be located in /tmp/librespot-build
#
# If only one architecture is desired, cargo can be invoked directly with the appropriate options :
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --no-default-features --features "with-syntex alsa-backend"
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features "with-syntex alsa-backend"
# $ docker run -v /tmp/librespot-build:/build librespot-cross cargo build --release --target arm-unknown-linux-gnueabi --no-default-features --features "with-syntex alsa-backend"
#

FROM debian:stretch

RUN dpkg --add-architecture armhf
RUN dpkg --add-architecture armel
RUN apt-get update

RUN apt-get install -y curl build-essential crossbuild-essential-armhf crossbuild-essential-armel
RUN apt-get install -y libasound2-dev libasound2-dev:armhf libasound2-dev:armel

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin/:${PATH}"
RUN rustup target add arm-unknown-linux-gnueabi
RUN rustup target add arm-unknown-linux-gnueabihf

RUN echo '[target.arm-unknown-linux-gnueabihf]\nlinker = "arm-linux-gnueabihf-gcc"' > ~/.cargo/config
RUN echo '[target.arm-unknown-linux-gnueabi]\nlinker = "arm-linux-gnueabi-gcc"' > ~/.cargo/config

RUN mkdir /build
ENV CARGO_TARGET_DIR /build

ADD . /src
WORKDIR /src
CMD ["/src/contrib/docker-build.sh"]
