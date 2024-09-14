use crate::dealer::{Builder, Dealer, Subscription, WsError};
use crate::Error;
use std::cell::OnceCell;
use std::str::FromStr;
use thiserror::Error;
use url::Url;

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

    pub async fn start(&self) -> Result<(), Error> {
        let url = self.get_url().await?;
        debug!("Launching dealer at {url}");

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
