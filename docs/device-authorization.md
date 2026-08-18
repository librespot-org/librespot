# Device Authorization

Spotify implements the OAuth 2.0 Device Authorization Grant
([RFC 8628](https://datatracker.ietf.org/doc/html/rfc8628)). Instead of redirecting a
user agent back to the client, the authorization server issues a short user code which
the user types in at <https://spotify.com/pair> from any other device. Nothing has to
listen on a port and the device itself needs no browser.

librespot exposes it as `--enable-device-auth`, implemented by `DeviceAuthClient` in the
`librespot-oauth` crate.

The device makes two calls, and waits in between:

```
POST /oauth2/device/authorize   -> device_code (secret) + user_code (shown to the user)
POST /api/token                 -> repeat until it stops saying authorization_pending
```

Between those, the user opens <https://spotify.com/pair> on some other device and enters
the `user_code`. The device plays no part in that and learns the outcome only by polling.

Both device endpoints take `application/x-www-form-urlencoded` bodies and answer with
JSON, including on error. Neither needs an `Authorization` header, a client secret, PKCE,
or cookies.

## 1. Device authorization request

`POST https://accounts.spotify.com/oauth2/device/authorize`

| Parameter | Required | Notes |
| --- | --- | --- |
| `client_id` | yes | Must be a client ID Spotify has enabled for this flow. |
| `scope` | no | Separated by either commas or spaces. Omitting it is accepted. |
| `creation_point` | no | A URL identifying where the flow began. The desktop client sends one; the server does not require it. |
| `intent` | no | The desktop client sends `login`. Optional. |

Response:

```json
{
  "device_code": "QUJDREVGLDAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMA==",
  "user_code": "ABCDEF",
  "verification_uri": "https://spotify.com/pair",
  "verification_uri_complete": "https://spotify.com/pair?code=ABCDEF",
  "expires_in": 3599,
  "interval": 5
}
```

The field names are exactly RFC 8628's, so a standards-compliant OAuth client library can
parse this response unmodified. `device_code` is base64 of `<user_code>,<uuid>`.

Errors are RFC 6749 shaped, at HTTP 400:

| `error` | Cause |
| --- | --- |
| `invalid_client` | `client_id` missing or not a real client. |
| `unauthorized_client` | Real client ID, but not enabled for the device flow. |
| `invalid_scope` | At least one requested scope is not recognised for this client. |

Scopes are validated before the client's eligibility for the flow, so a request that
is wrong in both ways reports `invalid_scope` and never mentions the client.

## 2. User approval

The user opens `verification_uri_complete` (or types `user_code` at `verification_uri`)
on another device and approves. This half is between the browser and Spotify; the device
takes no part in it and only observes the result by polling.

For reference, the page resolves the code via
`POST https://accounts.spotify.com/pair/api/code?flow_ctx=<uuid>:<timestamp>` with
`{"code": "<user_code>"}`, which returns the client's display name and the scopes being
requested, grouped for presentation.

## 3. Token polling

`POST https://accounts.spotify.com/api/token`

| Parameter | Value |
| --- | --- |
| `client_id` | Same as step 1. |
| `grant_type` | `urn:ietf:params:oauth:grant-type:device_code` |
| `device_code` | From step 1. |

Poll no faster than `interval` seconds. While the user has not yet acted, each poll
returns HTTP 400:

```json
{"error": "authorization_pending"}
```

A malformed or unknown `device_code` gives `invalid_request`. The remaining outcomes are
the ones RFC 8628 section 3.5 defines — `slow_down`, `access_denied` and `expired_token` —
which have not been observed directly here; only `authorization_pending` and success have.

On success, HTTP 200:

```json
{
  "access_token": "BQ...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "refresh_token": "AQ...",
  "scope": "streaming playlist-read user-read-email ..."
}
```

The granted `scope` is **space**-separated regardless of which separator the request used.

The `refresh_token` is exchanged at the same endpoint with `grant_type=refresh_token` and
`refresh_token=<token>`; a bad token there gives `invalid_grant`.

## Client IDs

Spotify enables this flow per client. Of the IDs librespot ships in
`core/src/config.rs`, only the desktop one is accepted:

| Client ID | Constant | Device flow |
| --- | --- | --- |
| `65b708073fc0480ea92a077233ca87bd` | `KEYMASTER_CLIENT_ID` | Yes |
| `9a8d2f0ce77a4e248bb71fefcb557637` | `ANDROID_CLIENT_ID` | No |
| `58bd3c95768941ea9eb4350aaa033eb3` | `IOS_CLIENT_ID` | No |

`--enable-device-auth` therefore only works with a client ID Spotify has enabled, which
by default means running on a platform where `SessionConfig` picks the desktop ID.

The mobile IDs are not OAuth clients at all; they exist only for Login5
(`core/src/login5.rs`).

The refusal is reported as `unauthorized_client`, but only for scopes the client is
otherwise allowed. Each client has its own scope registry, and requesting anything
outside it reports `invalid_scope` first. Of librespot's 26 `OAUTH_SCOPES` the Android
client does not recognise nine — `app-remote-control`, `user-modify-playback-state`,
`user-personalized`, `user-read-currently-playing`, `user-read-play-history`,
`user-read-playback-position`, `user-read-playback-state`, `user-read-recently-played`
and `user-top-read` — and the iOS client does not recognise `user-personalized`. So
attempting this flow on those platforms surfaces as `invalid_scope`, which points at the
scopes rather than at the real problem.

## Scopes

An unrecognised scope fails the whole request with `invalid_scope` rather than being
dropped, so scopes cannot be probed by requesting extras and seeing what comes back.

The desktop client requests 28 scopes. librespot requests 26 of them, leaving out
`sts-content-management` and `transfer-auth-session`. All 26 are accepted.

Note that `default` appears in the scope list Spotify's own pair page reports for the
desktop client, but sending it is rejected with `invalid_scope`. It is not requestable.

## Headers

The desktop client sends `client-token` and `spotify-installation-id` on both requests.
Neither is required: the flow completes with no headers beyond `Content-Type`. Responses
set a `__Host-device_id` cookie which is likewise not needed, and the flow succeeds
without ever sending it back.
