use std::cell::OnceCell;
use std::str::FromStr;

use thiserror::Error;
use tokio::sync::mpsc;
use url::Url;

use crate::dealer::{
    Builder, Dealer, Request, RequestHandler, Responder, Response, Subscription, WsError,
};
use crate::Error;

component! {
    DealerManager: DealerManagerInner {
        builder: OnceCell<Builder> = OnceCell::from(Builder::new()),
        dealer: OnceCell<Dealer> = OnceCell::new(),
    }
}

#[derive(Error, Debug)]
enum DealerError {
    #[error("Builder wasn't available")]
    BuilderNotAvailable,
    #[error("Websocket couldn't be started because: {0}")]
    LaunchFailure(WsError),
    #[error("Failed to set dealer")]
    CouldNotSetDealer,
}

impl From<DealerError> for Error {
    fn from(err: DealerError) -> Self {
        Error::failed_precondition(err)
    }
}

#[derive(Debug)]
pub enum Reply {
    Success,
    Failure,
    Unanswered,
}

pub type RequestReply = (Request, mpsc::UnboundedSender<Reply>);
type RequestReceiver = mpsc::UnboundedReceiver<RequestReply>;
type RequestSender = mpsc::UnboundedSender<RequestReply>;

struct DealerRequestHandler(RequestSender);

impl DealerRequestHandler {
    pub fn new() -> (Self, RequestReceiver) {
        let (tx, rx) = mpsc::unbounded_channel();
        (DealerRequestHandler(tx), rx)
    }
}

impl RequestHandler for DealerRequestHandler {
    fn handle_request(&self, request: Request, responder: Responder) {
        let (tx, mut rx) = mpsc::unbounded_channel();

        if let Err(why) = self.0.send((request, tx)) {
            error!("failed sending dealer request {why}");
            responder.send(Response { success: false });
            return;
        }

        tokio::spawn(async move {
            let reply = rx.recv().await.unwrap_or(Reply::Failure);
            debug!("replying to ws request: {reply:?}");
            match reply {
                Reply::Unanswered => responder.force_unanswered(),
                Reply::Success | Reply::Failure => responder.send(Response {
                    success: matches!(reply, Reply::Success),
                }),
            }
        });
    }
}

impl DealerManager {
    async fn get_url(&self) -> Result<Url, Error> {
        let session = self.session();

        let (host, port) = session.apresolver().resolve("dealer").await?;
        let token = session
            .token_provider()
            .get_token("streaming")
            .await?
            .access_token;
        let url = format!("wss://{host}:{port}/?access_token={token}");

        let url = Url::from_str(&url)?;
        Ok(url)
    }

    pub fn listen_for(&self, url: impl Into<String>) -> Result<Subscription, Error> {
        let url = url.into();
        self.lock(|inner| {
            if let Some(dealer) = inner.dealer.get() {
                dealer.subscribe(&[&url])
            } else if let Some(builder) = inner.builder.get_mut() {
                builder.subscribe(&[&url])
            } else {
                Err(DealerError::BuilderNotAvailable.into())
            }
        })
    }

    pub fn handle_for(&self, url: impl Into<String>) -> Result<RequestReceiver, Error> {
        let url = url.into();

        let (handler, receiver) = DealerRequestHandler::new();
        self.lock(|inner| {
            if let Some(dealer) = inner.dealer.get() {
                dealer.add_handler(&url, handler).map(|_| receiver)
            } else if let Some(builder) = inner.builder.get_mut() {
                builder.add_handler(&url, handler).map(|_| receiver)
            } else {
                Err(DealerError::BuilderNotAvailable.into())
            }
        })
    }

    pub async fn start(&self) -> Result<(), Error> {
        let url = self.get_url().await?;
        debug!("Launching dealer");

        let get_url = move || {
            let url = url.clone();
            async move { url }
        };

        let dealer = self
            .lock(move |inner| inner.builder.take())
            .ok_or(DealerError::BuilderNotAvailable)?
            .launch(get_url, None)
            .await
            .map_err(DealerError::LaunchFailure)?;

        self.lock(|inner| inner.dealer.set(dealer))
            .map_err(|_| DealerError::CouldNotSetDealer)?;

        Ok(())
    }

    pub async fn close(&self) {
        if let Some(dealer) = self.lock(|inner| inner.dealer.take()) {
            dealer.close().await
        }
    }
}
