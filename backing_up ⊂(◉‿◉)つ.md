# To play a playlist
From `./librespot/` 

`cd oauth`
`cargo run --example oauth_sync`

Copy the entire `access_token` string

`cd ..`

`cargo run --example play_playlist <ACCESS_TOKEN> <SPOTIFY_URI>`