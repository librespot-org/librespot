# Examples

This folder contains examples of how to use the `librespot` library for various purposes.

## How to run the examples

In general, to invoke an example, clone down the repo and use `cargo` as follows:

```
cargo run --example [filename]
```

in which `filename` is the file name of the example, for instance `get_token` or `play`.

### Acquiring an access token

Most examples require an access token as the first positional argument. **Note that an access token
gained by the client credentials flow will not work**. `librespot-oauth` provides a utility to 
acquire an access token using an OAuth flow, which will be able to run the examples. To invoke this, 
run:

```
cargo run --package librespot-oauth --example oauth_sync
```

A browser window will open and prompt you to authorize with Spotify. Once done, take the 
`access_token` property from the dumped object response and proceed to use it in examples. You may
find it convenient to save it in a shell variable like `$ACCESS_TOKEN`.

Once you have obtained the token you can proceed to run the example. Check each individual
file to see what arguments are expected. As a demonstration, here is how to invoke the `play` 
example to play a song -- the second argument is the URI of the track to play.

```
cargo run --example play "$ACCESS_TOKEN" 2WUy2Uywcj5cP0IXQagO3z
```