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
Facebook authentication is currently broken due to Spotify changing the authentication flow. The details of how the new flow works are detailed in https://github.com/librespot-org/librespot/issues/244 and will be implemented at some point in the future.
