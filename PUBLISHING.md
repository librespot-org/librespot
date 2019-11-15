# Publishing

## How To

The bash script in the root of the project, named `publish.sh` can be used to publish a new version of librespot and it's corresponding crates. the command should be used as follows: `./publish 0.1.0` from the project root, substituting the new version number that you wish to publish. *Note the lack of a v prefix on the version number. This is important, do not add one.* The v prefix is added where appropriate by the script.

## What the script does

This is briefly how the script works:

  - Change to branch master, pull latest version, merge development branch.
  - CD to working dir.
  - Change version number in all files.
  - Commit and tag changes.
  - Publish crates in given order.
  - Push version commit and tags to master.

## Notes

Publishing librespot to crates.io is a slightly convoluted affair due to the various dependencies that each package has on other local packages. The order of publising that has been found to work is as follows:

`protocol -> core -> audio -> metadata -> playback -> connect -> librespot`

The `protocol` package needs to be published with `cargo publish --no-verify` due to the build script modifying the source during compile time.

Publishing can be done using the command `cargo publish` in each of the directories of the respective crate.

The script is meant to cover the standard publishing process. There are various improvements that could be made, such as adding options such as the user being able to add a change log, though this is not the main focus, as the script is intended to be run by a CI. Feel free to improve and extend functionality, keeping in mind that it should always be possible for the script to be run in a non-interactive fashion.
