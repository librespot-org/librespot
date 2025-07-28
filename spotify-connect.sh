#!/usr/bin/env sh

LOCATION=$HOME/librespot-hqplayer
DEVICE=BlackHole\ 2ch

${LOCATION}/target/release/librespot \
  --normalisation-threshold -3.0 \
  --normalisation-release 100 \
  --normalisation-knee 5.0 \
  --normalisation-pregain 5.0 \
  --normalisation-attack 30 \
  --enable-volume-normalisation \
  --cache "$HOME/.librespot" \
  --bitrate 320 \
  --name "HQPL" \
  --onevent ${LOCATION}/librespot-hqplayer-controller \
  --mixer null \
  --initial-volume 70 \
  --device-type stb \
  --device "$DEVICE" \
  --enable-oauth \
  --format F32 \
  --volume-steps 30