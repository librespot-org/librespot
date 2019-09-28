use ws_server::WebSocketServer;
use futures::sync::mpsc::{unbounded, UnboundedReceiver};
use librespot::playback::player::PlayerEvent;
use librespot::connect::spirc::SpircTask;
use futures::{Future, Poll, Stream};

pub enum PlayerCommandMessage {
    Prev,
    Next,
    Play,
    Pause,
    Reauth,
}

pub struct PlayerWsServer {
    server: WebSocketServer,
    receiver: UnboundedReceiver<PlayerCommandMessage>,
}

impl PlayerWsServer {
    pub fn new(port: u16) -> PlayerWsServer {
        let (server, _t1, _t2) = WebSocketServer::new(port);

        let (sender, receiver) = unbounded();

        server.register_listener(move |_, message| {
            if let ws::Message::Text(content) = message {
                match &content[..] {
                    "next" => sender.unbounded_send(PlayerCommandMessage::Next),
                    "prev" => sender.unbounded_send(PlayerCommandMessage::Prev),
                    "play" => sender.unbounded_send(PlayerCommandMessage::Play),
                    "pause" => sender.unbounded_send(PlayerCommandMessage::Pause),
                    "reauth" => sender.unbounded_send(PlayerCommandMessage::Reauth),
                    _ => Result::Ok(()),
                }
                .unwrap();
            }
        });

        PlayerWsServer {
            server,
            receiver,
        }
    }

    pub fn broadcast_event(&self, event: &PlayerEvent, spirc_task: &SpircTask) {
        match event {
            PlayerEvent::Changed {
                old_track_id,
                new_track_id,
            } => self.server.broadcast(
                json!({
                    "event": "track_update",
                    "old_track_id": old_track_id.to_base16(),
                    "new_track_id": new_track_id.to_base16(),
                    "current_queue": spirc_task.get_queue_uids(),
                    "current_index": spirc_task.get_current_index()
                })
                .to_string(),
            ),
            PlayerEvent::Started { track_id } => self.server.broadcast(
                json!({
                    "event": "track_started",
                    "track_id": track_id.to_base16(),
                    "current_queue": spirc_task.get_queue_uids(),
                    "current_index": spirc_task.get_current_index()
                })
                .to_string(),
            ),
            PlayerEvent::Stopped { track_id } => self.server.broadcast(
                json!({
                    "event": "track_stopped",
                    "track_id": track_id.to_base16(),
                    "current_queue": spirc_task.get_queue_uids(),
                    "current_index": spirc_task.get_current_index()
                })
                .to_string(),
            ),
        }
    }
}

impl Future for PlayerWsServer {
    type Item = Option<PlayerCommandMessage>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<PlayerCommandMessage>, ()> {
        self.receiver.poll()
    }
}