# Contributing

## Reporting an Issue

Issues are tracked in the Github issue tracker of the librespot repo.

If you have encountered a bug, please report it, as we rely on user reports to fix them.

Please also make sure that your issues are helpful. To ensure that your issue is helpful, please read over this brief checklist to avoid the more common pitfalls:

- Please take a moment to search/read previous similar issues to ensure you aren’t posting a duplicate. Duplicates will be closed immediately.
- Please include a clear description of what the issue is. Issues with descriptions such as ‘It hangs after 40 minutes’ will be closed immediately.
- Please include, where possible, steps to reproduce the bug, along with any other material that is related to the bug. For example, if librespot consistently crashes when you try to play a song, please include the Spotify URI of that song. This can be immensely helpful in quickly pinpointing and resolving issues.
- Please be alert and respond to questions asked by any project members. Stale issues will be closed.
- When your issue concerns audio playback, please first make sure that your audio system is set up correctly and can play audio from other applications. This project aims to provide correct audio backends, not to provide Linux support to end users.
- Lastly, and perhaps most importantly, please include a backtrace where possible. Recent versions of librespot should produce these automatically when it crashes, and print them to the console, but in some cases, you may need to run ‘export RUST_BACKTRACE=full’ before running librespot to enable backtraces.

## Contributing Code

If there is an issue that you would like to write a fix for, or a feature you would like to implement, we use the following flow for updating code in the librespot repo:

```
Fork -> Fix -> PR -> Review -> Merge
```

This is how all code is added to the repository, even by those with write access.

#### Steps before Committing

In order to prepare for a PR, you will need to do a couple of things first:

Make any changes that you are going to make to the code, but do not commit yet.

Unless your changes are negligible, please add an entry in the "Unreleased" section of `CHANGELOG.md`. Refer to [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) for instructions on how this entry should look like. If your changes break the API such that downstream packages that depend on librespot need to update their source to still compile, you should mark your changes as `(breaking)`.

Make sure that the code is correctly formatted by running:
```bash
cargo fmt --all
```

This command runs ```rustfmt```, a code formatting tool that will automatically correct any formatting that you have used that does not conform with the librespot code style. Once that command has run, you will need to rebuild the project:

```bash
cargo build
```

Once it has built, check for common code mistakes by running:
```bash
cargo clippy
```

Once you have confirmed there are no warnings or errors, you should commit your changes.

```bash
git commit -a -m "My fancy fix"
```

**N.B.** Please, for the sake of a readable history, do not bundle multiple major changes into a single commit. Instead, break it up into multiple commits.

Once you have made the commits you wish to have merged, push them to your forked repo:

```bash
git push
```

Then open a pull request on the main librespot repo.

Once a pull request is under way, it will be reviewed by one of the project maintainers, and either approved for merging, or have changes requested. Please be alert in the review period for possible questions about implementation decisions, implemented behaviour, and requests for changes. Once the PR is approved, it will be merged into the main repo.

Happy Contributing :)
