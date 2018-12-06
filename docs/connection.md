# Connection Setup
## Access point Connection
The first step to connecting to Spotify's servers is finding an Access Point (AP) to do so.
Clients make an HTTP GET request to `http://apresolve.spotify.com` to retrieve a list of hostname an port combination in JSON format.
An AP is randomly picked from that list to connect to.

The connection is done using a bare TCP socket. Despite many APs using ports 80 and 443, neither HTTP nor TLS are used to connect.

If `http://apresolve.spotify.com` is unresponsive, `ap.spotify.com:443` is used as a fallback.

## Connection Hello
The first 3 packets exchanged are unencrypted, and have the following format :

header   | length | payload
---------|--------|---------
variable |   32   | variable

Length is a 32 bit, big endian encoded, integer.
It is the length of the entire packet, ie `len(header) + 4 + len(payload)`.

The header is only present in the very first packet sent by the client, and is two bytes long, `[0, 4]`.
It probably corresponds to the protocol version used.

The payload is a protobuf encoded message.

The client starts by sending a `ClientHello` message, describing the client info, a random nonce and client's Diffie Hellman public key.

The AP replies by a `APResponseMessage` message, containing a random nonce and the server's DH key.

The client solves a challenge based on these two packets, and sends it back using a `ClientResponsePlaintext`.
It also computes the shared keys used to encrypt the rest of the communication.

## Login challenge and cipher key computation.
The client starts by computing the DH shared secret using it's private key and the server's public key.
HMAC-SHA1 is then used to compute the send and receive keys, as well as the login challenge.

```
data = []
for i in 1..6 {
    data += HMAC(client_hello || ap_response || [ i ], shared)
}

challenge = HMAC(client_hello || ap_response, data[:20])
send_key = data[20:52]
recv_key = data[52:84]
```

`client_hello` and `ap_response` are the first packets sent respectively by the client and the AP.
These include the header and length fields.

## Encrypted packets
Every packet after ClientResponsePlaintext is encrypted using a Shannon cipher.

The cipher is setup with 4 bytes big endian nonce, incremented after each packet, starting at zero.
Two independent ciphers and accompanying nonces are used, one for transmission and one for reception,
using respectively `send_key` and `recv_key` as keys.

The packet format is as followed :

cmd | length | payload  | mac
----|--------|----------|----
 8  |   16   | variable | 32

Each packet has a type identified by the 8 bit `cmd` field.
The 16 bit big endian length only includes the length of the payload.

