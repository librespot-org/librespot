# Create a docker image for the RPI
# Build the docker image from the root of the project with the following command :
# $ docker build -t librespot-rpi -f .\contrib\Dockerfile.Rpi .
#
# This builds a docker image which is usable when running docker on the rpi.
# 
# This Dockerfile builds in windows without any requirements, for linux based systems you might need to run the following line:
# docker run --rm --privileged multiarch/qemu-user-static:register --reset
# (see here for more info: https://gist.github.com/PieterScheffers/d50f609d9628383e4c9d8d7d269b7643 )
#
# Save the docker image to a file:
# $ docker save -o contrib/librespot-rpi librespot-rpi
#
# Move it to the rpi and import it with:
# docker load -i librespot-rpi
#
# Run it with:
# docker run -d --restart unless-stopped $(for DEV in $(find /dev/snd -type c); do echo --device=$DEV:$DEV; done) --net=host --name librespot-rpi librespot-rpi --name {devicename} 

FROM debian:stretch

RUN dpkg --add-architecture armhf
RUN apt-get update

RUN apt-get install -y curl git build-essential crossbuild-essential-armhf
RUN apt-get install -y libasound2-dev libasound2-dev:armhf

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin/:${PATH}"
RUN rustup target add arm-unknown-linux-gnueabihf

RUN mkdir /.cargo && \
    echo '[target.arm-unknown-linux-gnueabihf]\nlinker = "arm-linux-gnueabihf-gcc"' >> /.cargo/config

RUN mkdir /build
ENV CARGO_TARGET_DIR /build
ENV CARGO_HOME /build/cache

ADD . /src
WORKDIR /src
RUN cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features "alsa-backend"


FROM resin/rpi-raspbian
RUN apt-get update && \
    apt-get install libasound2 && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir /librespot
WORKDIR /librespot

COPY --from=0 /build/arm-unknown-linux-gnueabihf/release/librespot .
RUN chmod +x librespot
ENTRYPOINT ["./librespot"]