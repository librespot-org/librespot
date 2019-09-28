use std::collections::BTreeMap;
use std::sync::mpsc;
use std::thread;
use ws::{listen, CloseCode, Handler, Result, Sender};

struct WebSocketHandler {
    id: usize,
    message_sender: mpsc::Sender<Message>,
}

impl Handler for WebSocketHandler {
    fn on_message(&mut self, msg: ws::Message) -> Result<()> {
        self.message_sender
            .send(Message::ClientMessage(self.id, msg))
            .unwrap();
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
        self.message_sender.send(Message::RemoveClient(self.id)).unwrap();
        println!("Client removed");
    }
}

trait Listener {
    fn call(self: &Self, id: usize, msg: &ws::Message);
}

impl<F: Fn(usize, &ws::Message)> Listener for F {
    fn call(self: &F, id: usize, msg: &ws::Message) {
        (*self)(id, msg);
    }
}

pub struct WebSocketServer {
    message_sender: mpsc::Sender<Message>,
}

enum Message {
    AddClient(usize, Sender),
    RemoveClient(usize),
    Broadcast(String),
    ClientMessage(usize, ws::Message),
    RegisterListener(Box<dyn Listener + Send + 'static>),
}

impl WebSocketServer {
    pub fn new(port: u16) -> (WebSocketServer, thread::JoinHandle<()>, thread::JoinHandle<()>) {
        let (message_sender, receiver) = mpsc::channel::<Message>();
        let manager_thread = thread::spawn(|| {
            let mut clients: BTreeMap<usize, Sender> = BTreeMap::new();
            let mut listeners: Vec<Box<dyn Listener + Send + 'static>> = Vec::new();
            for message in receiver {
                match message {
                    Message::AddClient(id, sender) => {
                        clients.insert(id, sender);
                    }
                    Message::RemoveClient(id) => {
                        clients.remove(&id);
                    }
                    Message::Broadcast(content) => {
                        for (_, client) in clients.iter() {
                            client.send(ws::Message::Text(content.clone())).unwrap();
                        }
                    }
                    Message::ClientMessage(id, content) => {
                        for listener in &listeners {
                            (*listener).call(id, &content);
                        }
                    }
                    Message::RegisterListener(listener) => listeners.push(listener),
                }
            }
        });
        let sender_for_server_thread = message_sender.clone();
        let message_sender_for_server = message_sender.clone();
        let server_thread = thread::spawn(move || {
            let mut last_id = 0;
            listen(format!("127.0.0.1:{}", port), |out| {
                // sender_for_server_thread.send(Message::AddClient(out)).unwrap();
                let id = last_id;
                last_id += 1;
                sender_for_server_thread
                    .send(Message::AddClient(id, out))
                    .unwrap();
                WebSocketHandler {
                    id,
                    message_sender: message_sender_for_server.clone(),
                }
            })
            .unwrap();
        });

        (
            WebSocketServer {
                message_sender: message_sender.clone(),
            },
            manager_thread,
            server_thread,
        )
    }

    pub fn broadcast(&self, msg: String) {
        self.message_sender.send(Message::Broadcast(msg)).unwrap();
    }

    pub fn register_listener<F>(&self, listener: F)
    where
        F: Fn(usize, &ws::Message) -> () + Send + 'static,
    {
        self.message_sender
            .send(Message::RegisterListener(Box::new(listener)))
            .unwrap();
    }
}
