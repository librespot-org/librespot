# Authentication
Once the connection is setup, the client can authenticate with the AP. For this, it sends an
`ClientResponseEncrypted` message, using packet type `0xab`.

A few different authentication methods are available. They are described below.

The AP will then reply with either a `APWelcome` message using packet type `0xac` if authentication
is successful, or an `APLoginFailed` with packet type `0xad` otherwise.

## Password based Authentication
Password authentication is trivial.
The `ClientResponseEncrypted` message's `LoginCredentials` is simply filled with the username
and setting the password as the `auth_data`, and type `AUTHENTICATION_USER_PASS`.

## Zeroconf based Authentication
Rather than relying on the user entering a username and password, devices can use zeroconf based
authentication. This is especially useful for headless Spotify Connect devices.

In this case, an already authenticated device, a phone or computer for example, discovers Spotify
Connect receivers on the local network using Zeroconf. The receiver exposes an HTTP server with
service type `_spotify-connect._tcp`,

Two actions on the HTTP server are exposed, `getInfo` and `addUser`.
The former returns information about the receiver, including its DH public key, in JSON format.
The latter is used to send the username, the controller's DH public key, as well as the encrypted
blob used to authenticate with Spotify's servers.

The blob is decrypted using the following algorithm.

```
# encrypted_blob is the blob sent by the controller, decoded using base64
# shared_secret is the result of the DH key exchange

IV = encrypted_blob[:0x10]
expected_mac = encrypted_blob[-0x14:]
encrypted = encrypted_blob[0x10:-0x14]

base_key       = SHA1(shared_secret)
checksum_key   = HMAC-SHA1(base_key, "checksum")
encryption_key = HMAC-SHA1(base_key, "encryption")[:0x10]

mac = HMAC-SHA1(checksum_key, encrypted)
assert mac == expected_mac

blob = AES128-CTR-DECRYPT(encryption_key, IV, encrypted)
```

The blob is then used as described in the next section.

## Blob based Authentication

```
data = b64_decode(blob)
base_key = PBKDF2(SHA1(deviceID), username, 0x100, 1)
key = SHA1(base_key) || htonl(len(base_key))
login_data = AES192-DECRYPT(key, data)
```

## Facebook based Authentication
The client starts an HTTPS server, and makes the user visit
`https://login.spotify.com/login-facebook-sso/?csrf=CSRF&port=PORT`
in their browser, where CSRF is a random token, and PORT is the HTTPS server's port.

This will redirect to Facebook, where the user must login and authorize Spotify, and
finally make a GET request to
`https://login.spotilocal.com:PORT/login/facebook_login_sso.json?csrf=CSRF&access_token=TOKEN`,
where PORT and CSRF are the same as sent earlier, and TOKEN is the facebook authentication token.

Since `login.spotilocal.com` resolves the 127.0.0.1, the request is received by the client.

The client must then contact Facebook's API at
`https://graph.facebook.com/me?fields=id&access_token=TOKEN`
in order to retrieve the user's Facebook ID.

The Facebook ID is the `username`, the TOKEN the `auth_data`, and `auth_type` is set to `AUTHENTICATION_FACEBOOK_TOKEN`.

