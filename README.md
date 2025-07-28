# librespot-hqplayer
Fork of librespot that integrates Spotify Connect with HQPlayer5 Desktop via hqp5-control

## Config
```
# Change librespot settings
vi spotify-connect.sh
```

## Install
```
# If node is installed, update the first line of librespot-hqplayer-controller accordingly. Otherwise:
brew install nodejs

cd ~
git clone https://github.com/shagamemnon/librespot-hqplayer.git
cd librespot-hqplayer
chmod a+x librespot-hqplayer-controller
cargo build --release

```

## Run
```
sh spotify-connect.sh
```

## Librespot
*librespot* is an open source client library for Spotify. It enables applications to use Spotify's service to control and play music via various backends, and to act as a Spotify Connect receiver. It is an alternative to the official and [now deprecated](https://pyspotify.mopidy.com/en/latest/#libspotify-s-deprecation) closed-source `libspotify`. Additionally, it will provide extra features which are not available in the official library.