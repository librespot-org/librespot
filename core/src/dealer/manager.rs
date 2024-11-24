use std::cell::OnceCell;
use std::str::FromStr;

use thiserror::Error;
use tokio::sync::mpsc;
use url::Url;

use super::{
    Builder, Dealer, GetUrlResult, Request, RequestHandler, Responder, Response, Subscription,
};
use crate::{Error, Session};

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
    LaunchFailure(Error),
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
    async fn get_url(session: Session) -> GetUrlResult {
        let (host, port) = session.apresolver().resolve("dealer").await?;
        let token = session.login5().auth_token().await?.access_token;
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

    pub fn handles(&self, uri: &str) -> bool {
        self.lock(|inner| {
            if let Some(dealer) = inner.dealer.get() {
                dealer.handles(uri)
            } else if let Some(builder) = inner.builder.get() {
                builder.handles(uri)
            } else {
                false
            }
        })
    }

    pub async fn start(&self) -> Result<(), Error> {
        debug!("Launching dealer");

        let session = self.session();
        // the url has to be a function that can retrieve a new url,
        // otherwise when we later try to reconnect with the initial url/token
        // and the token is expired we will just get 401 error
        let get_url = move || Self::get_url(session.clone());

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
