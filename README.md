# fkspot

## What's with the name?
Well, fu*k spotify (pls don't sue me)

## What in the name of satan is this repo?
fkspot will be soon used for retrieval of audio decryption key in [muffon](https://github.com/staniel359/muffon)'s backend

# Building/Running fkspot
>[!NOTE]
>You will need a fkspot.cfg file in your current directory. See [example](https://github.com/xyloflake/fkspot/blob/dev/fkspot.cfg)

### Preliminary Steps:
- Install [rust](https://rust-lang.org) (ofc lol)
- Clone this repo. Here's the command if you're too lazy to type, thank me later :)
```bash
git clone https://github.com/xyloflake/fkspot
```
- cd into the cloned project
  
### Run with Development mode (faster compilation, slower runtime, recommended if you're running it locally for testing)
```bash
cargo run --bin fkspot
```
### Run with Release mode (slower compilation, faster runtime, recommended if you're running it in production)
```bash
cargo run --release --bin fkspot
```
### Configure logging
Recommended: Debug + Info
```bash
export RUST_LOG = "fkspot::connection=debug,info"
```

This will (if you have the correct configuration) start the server at https://localhost:3745/

# Using fkspot
After starting the server, you can make requests to get audio keys.
For example you can request the following url:
```
http://localhost:3745/audiokey/5B5M9o7xEcq6FdEeXrByY0*513ec76d1265b56b305dd21fdb4f43f93fccb5e
```
- `5B5M9o7xEcq6FdEeXrByY0` is the track id
-  `513ec76d1265b56b305dd21fdb4f43f93fccb5e` is the file id

After requesting, check the status code. As usual
- if status code is `200`, everything is okay and you may proceed
- if status code is `400`, you sent the wrong track id and/or file id

To proceed, parse the response body as a buffer of **unsigned 8 bit integers, over 16 bytes** or **[u8,16]**

Finally, you may use the key as intended :)
