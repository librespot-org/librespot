# Publishing

## How To

Read through this paragraph in its entirety before running anything.

The Bash script in the root of the project, named `publish.sh` can be used to publish a new version of librespot and its corresponding crates. the command should be used as follows from the project root: `./publish 0.1.0` from the project root, substituting the new version number that you wish to publish. *Note the lack of a v prefix on the version number. This is important, do not add one.* The v prefix is added where appropriate by the script.

Make sure that you are are starting from a clean working directory for both `dev` and `master`, completely up to date with remote and all local changes either committed and pushed or stashed.

Note that the script will update the crates and lockfile, so in case you did not do so before, you really should to make sure none of the dependencies introduce some SemVer breaking change. Then commit so you again have a clean working directory.

Also don't forget to update `CHANGELOG.md` with the version number, release date, and at the bottom the comparison links.

You will want to perform a dry run first: `./publish --dry-run 0.1.0`. Please make note of any errors or warnings. In particular, you may need to explicitly inform Git which remote you want to track for the `master` branch like so: `git --track origin/master` (or whatever you have called the `librespot-org` remote `master` branch).

Depending on your system the script may fail to publish the main `librespot` crate after having published all the `librespot-xyz` sub-crates. If so then make sure the working directory is committed and pushed (watch `Cargo.toml`) and then run `cargo publish` manually after `publish.sh` finished.

To publish the crates your GitHub account needs to be authorized on `crates.io` by `librespot-org`. First time you should run `cargo login` and follow the on-screen instructions.

## What the script does

This is briefly how the script works:

  - Change to branch master, pull latest version, merge development branch.
  - Change to working directory.
  - Change version number in all files.
  - Update crates and lockfile.
  - Commit and tag changes.
  - Publish crates in given order.
  - Push version commit and tags to master.

## Notes

Publishing librespot to crates.io is a slightly convoluted affair due to the various dependencies that each package has on other local packages. The order of publising that has been found to work is as follows:

`protocol -> core -> audio -> metadata -> playback -> connect -> librespot`

The `protocol` package needs to be published with `cargo publish --no-verify` due to the build script modifying the source during compile time.

Publishing can be done using the command `cargo publish` in each of the directories of the respective crate.

The script is meant to cover the standard publishing process. There are various improvements that could be made, such as adding options such as the user being able to add a changelog, though this is not the main focus, as the script is intended to be run by a CI. Feel free to improve and extend functionality, keeping in mind that it should always be possible for the script to be run in a non-interactive fashion.
