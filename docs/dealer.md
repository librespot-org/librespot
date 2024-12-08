# Dealer

When talking about the dealer, we are speaking about a websocket that represents the player as
spotify-connect device. The dealer is primarily used to receive updates and not to update the
state.

## Messages and Requests

There are two types of messages that are received via the dealer, Messages and Requests.
Messages are fire-and-forget and don't need a responses, while request expect a reply if the
request was processed successfully or failed.

Because we publish our device with support for gzip, the message payload might be BASE64 encoded
and gzip compressed. If that is the case, the related headers send an entry for "Transfer-Encoding"
with the value of "gzip".

### Messages

Most messages librespot handles send bytes that can be easily converted into their respective
protobuf definition. Some outliers send json that can be usually mapped to an existing protobuf
definition. We use `protobuf-json-mapping` to a similar protobuf definition

> Note: The json sometimes doesn't map exactly and can provide more fields than the protobuf
> definition expects. For messages, we usually ignore unknown fields.

There are two types of messages, "informational" and "fire and forget commands".

**Informational:**

Informational messages send any changes done by the current user or of a client where the current user
is logged in. These messages contain for example changes to a own playlist, additions to the liked songs
or any update that a client sends.

**Fire and Forget commands:**

These are messages that send information that are requests to the current player. These are only send to
the active player. Volume update requests and the logout request are send as fire-forget-commands.

### Requests

The request payload is sent as json. There are almost usable protobuf definitions (see
files named like `es_<command in snakecase>(_request).proto`) for the commands, but they don't
align up with the expected values and are missing some major information we need for handling some
commands. Because of that we have our own model for the specific commands, see
[core/src/dealer/protocol/request.rs](../core/src/dealer/protocol/request.rs).

All request modify the player-state.

## Details

This sections is for details and special hiccups in regards to handling that isn't completely intuitive.

### UIDs

A spotify item is identifiable by their uri. The `ContextTrack` and `ProvidedTrack` both have a `uid` 
field. When we receive a context via the `context-resolver` it can return items (`ContextTrack`) that
may have their respective uid set. Some context like the collection and albums don't provide this 
information.

When a `uid` is missing, resorting the next tracks in an official client gets confused and sends 
incorrect data via the `set_queue` request. To prevent this behavior we generate a uid for each 
track that doesn't have an uid. Queue items become a "queue-uid" which is just a `q` with an 
incrementing number.

### Metadata

For some client's (especially mobile) the metadata of a track is very important to display the 
context correct. For example the "autoplay" metadata is relevant to display the correct context 
info.

Metadata can also be used to store data like the iteration when repeating a context.

### Repeat

The context repeating implementation is partly mimicked from the official client. The official 
client allows skipping into negative iterations, this is currently not supported.

Repeating is realized by filling the next tracks with multiple contexts separated by delimiters.
By that we only have to handle the delimiter when skipping to the next and previous track.
