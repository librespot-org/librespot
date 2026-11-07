# Jams

A Jam is Spotify's group listening session. Inside Spotify it is called a *social session* and
the endpoints live under `social-connect`. The protobuf definitions are in
[`social_connect_v2.proto`](../protocol/proto/social_connect_v2.proto), package `socialconnect`.

None of this is documented by Spotify. It describes what a working implementation (Sonora, a
librespot based client) sends and receives, and it can change without notice.

## What a Jam is, from a device's point of view

A Jam is a Spotify Connect session that several accounts share. One account owns it and
its active device plays. Everyone else in the Jam is a member, who can listen and, depending on
the Jam, control and add to the queue.

A device takes part in a Jam like it does in any connect session: it is listed as a device and it
listens on the [dealer](dealer.md). Joining and leaving are plain `spclient` requests, and changes
arrive on a dealer topic.

## Requests

All requests go to `spclient`, as the account that is signed in.

Every request carries the header `spotify-connection-id` with the id of the dealer connection the
device listens on (`Session::connection_id`). That ties the session to the device. The device must
already be listed as a connect device, and its id is sent as `local_device_id`.

A `POST` without a body must still send `Content-Length: 0`, or Spotify refuses it.

| Request | Path | Notes |
| --- | --- | --- |
| Read the Jam the account is in | `GET /social-connect/v2/sessions/current` | `404` when it is in none, which is not a failure |
| Read the Jam a link names | `GET /social-connect/v2/sessions/info/{token}` | does not join it |
| Join | `POST /social-connect/v2/sessions/join/{token}?playback_control=listen_and_control&join_type=deeplinking&local_device_id={device_id}` | |
| Start a Jam, or get the one that runs | `GET /social-connect/v2/sessions/current_or_new?activate=true&local_device_id={device_id}&type=REMOTE` | `activate=true` is what makes the Jam show on the other members' devices |
| Leave | `POST /social-connect/v2/sessions/leave` | for a member |
| End | `DELETE /social-connect/v3/sessions/{session_id}?local_device_id={device_id}` | for the owner, since an owner that only leaves hands the Jam on |
| Remove a member | `POST /social-connect/v3/sessions/{session_id}/member/{member_id}/kick?local_device_id={device_id}` | owner only |

`session_id` is the `session_id` of the `Session` that `current` returns. `token` comes from a
link, see below.

### The answer

The answer is a `Session`, as protobuf or as JSON. A reader should accept both. In JSON the keys
come in `snake_case` or `camelCase`, and an empty list is left out, so a Jam with nobody else in
it has no `session_members`.

- `join_session_url` is only there for the owner, and it is an address only Spotify's own apps
  open. The link to hand out is `https://open.spotify.com/socialsession/{token}`.
- `is_session_owner` says whether the account owns the Jam, and `session_owner_id` names the
  owner. Compare it with both a member's `id` and `username`.
- `host_active_device_id` is the device that plays. `spirc` already reads it to keep its own
  `session_id`.

### Links

A link is `https://open.spotify.com/socialsession/{token}`. The token is letters and digits only.
Spotify's apps also share a `https://spotify.link/…` short link. That one redirects to the long
link, and sometimes answers with a page that names the session instead. Fetch only
`spotify.link`, read just the start of the page, and put nothing but letters and digits in a
request path.

## Updates

Changes arrive on the dealer topic `social-connect/v2/session_update`, which `spirc` already
listens to. The payload is a `SessionUpdate`: the new `session`, and a `reason`.

| Reason | Meaning |
| --- | --- |
| `NEW_SESSION`, `YOU_JOINED` | the account is in a Jam now |
| `USER_JOINED`, `USER_LEFT`, `USER_KICKED` | someone else came or went |
| `PARTICIPANT_PROMOTED_TO_HOST` | the owner changed |
| `DISCOVERABILITY_CHANGED` | the Jam can be found, or no longer |
| `SESSION_MEMBER_PAUSE_CHANGED` | a member paused or resumed. Not in the proto |
| `SESSION_DELETED`, `YOU_LEFT`, `YOU_WERE_KICKED` | the account is out of the Jam |

Three things to know:

- Spotify sends reasons the proto does not have, such as `WIFI_BROADCAST_CHANGED` and
  `SESSION_MEMBER_PAUSE_CHANGED`, which arrives with every pause and resume. A typed parse
  fails on them, and the update is lost. Read the payload as loose JSON first, and look at
  `reason` as text.
- An update may carry no members. It says nothing about who is in the Jam, so keep the list you
  have.
- The three last reasons in the table end the Jam for this account, whatever `session` holds.

## Playback

The owner's playback reaches a device through dealer requests with the endpoint `transfer`, like
a transfer between two devices, with these differences seen in a Jam:

- `from_device_identifier` is `social-connect-{hex}` instead of a device id.
- `options` is `restore_paused: restore`, `restore_position: extrapolate`,
  `restore_track: always_play_something` and `retain_session: retain_original`.
- `data` is a base64 `TransferState`, and its queue can be empty (`is_playing_queue` false), with
  the tracks only in the context.
- A request of this kind also arrives when the account leaves, around `YOU_LEFT`, so a client
  must not take every one of them as a reason to start playing.

Asking the owner's device to transfer playback to this one the ordinary way fails with
`410 Gone` while the account is a Jam member.

## Not settled

- Which metadata key of a queued `ProvidedTrack` names the person who added it. The keys differ
  between Jams, and the person is not always one of the `session_members`.
- Whether `join_session_token` can stand in for the token in the link.

## Starting a Jam from librespot

A librespot device that is listed can start a Jam with `current_or_new` and hand out the link
above. Nothing in `librespot-connect` does this yet, so a client makes the requests itself with
`Session::spclient().request(…)`, as the table shows.
