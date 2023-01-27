use std::fmt::Write;

use crate::MetadataError;

use librespot_core::{Error, Session};

pub type RequestResult = Result<bytes::Bytes, Error>;

#[async_trait]
pub trait MercuryRequest {
    async fn request(session: &Session, uri: &str) -> RequestResult {
        let mut metrics_uri = uri.to_owned();

        let separator = match metrics_uri.find('?') {
            Some(_) => "&",
            None => "?",
        };
        let _ = write!(metrics_uri, "{separator}country={}", session.country());

        if let Some(product) = session.get_user_attribute("type") {
            let _ = write!(metrics_uri, "&product={product}");
        }

        trace!("Requesting {}", metrics_uri);

        let request = session.mercury().get(metrics_uri)?;
        let response = request.await?;
        match response.payload.first() {
            Some(data) => {
                let data = data.to_vec().into();
                trace!("Received metadata: {data:?}");
                Ok(data)
            }
            None => Err(Error::unavailable(MetadataError::Empty)),
        }
    }
}
