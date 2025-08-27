# Publishing

## How To

1. [prepare the release](#prepare-the-release)
2. [create a github-release](#creating-a-github-release)

### Prepare the release

For preparing the release a manuel workflow should be available that takes care of the common preparation. But 
this can also be done manually if so desired. The workflow does:
- upgrade the version according to the targeted release (`major`, `minor`, `patch`)
  - `major` and `minor` require all crates to be updated
  - `patch` instead only upgrades the crates that had any changes
- updates the changelog according to Keep-A-Changelog convention
- commits and pushes the changes to remote

### Creating a github-release

After everything is prepared for the new version. A [new release can be created](https://github.com/librespot-org/librespot/releases/new) 
from the ui. The tag will not be available as it isn't set by the prepare workflow, so a new tag needs to be created.
The tag should be named like `v<version>` where `version` is the version of the binary.

> insert convention what the release title and release notes should be and what checkboxes to check etc.

The release should be created as draft, which will trigger the workflow that will publish the changed crates and binary.
The workflow will:
- check if all crates needs to be published or only certain crates
- publish the crates in a specific order while excluding crates that didn't have any changes
- publish the binary

After the workflow was successful the version can be published.

## Notes

Publishing librespot to crates.io is a slightly convoluted affair due to the various dependencies that each package has 
on other local packages. The order of publishing that has been found to work is as follows:
> `protocol -> core -> audio -> metadata -> playback -> connect -> librespot`

The `protocol` package needs to be published with `cargo publish --no-verify` due to the build script modifying the 
source during compile time. Publishing can be done using the command `cargo publish` in each of the directories of the 
respective crate.
