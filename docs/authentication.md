# Authentication
Once the connection is setup, the client can authenticate with the AP. For this, it sends an
`ClientResponseEncrypted` message, using packet type `0xab`.

A few different authentication methods are available, athough only one has really been tried, the
traditional user / password based authentication.

The AP will then reply with either a `APWelcome` message using packet type `0xac` if authentication
is successful, or an `APLoginFailed` with packet type `0xad` otherwise.

TODO: investigate other authentication methods (Zeroconf, facebook, stored credentials)
