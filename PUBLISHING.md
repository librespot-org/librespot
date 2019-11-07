# Publishing

Publishing librespot to crates.io is a slightly convoluted affair due to the various dependencies that each package has on other local packages. The order of publising that has been found to work is as follows:

`protocol -> core -> audio -> metadata -> playback -> connect -> librespot`

The `protocol` package needs to be published with `cargo publish --no-verify` due to the build script modifying the source during compile time.

Publishing can be done using the command `cargo publish` in each of the directories of the respecive crate.