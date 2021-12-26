use crate::MetadataError;

use librespot_core::{Error, Session};

pub type RequestResult = Result<bytes::Bytes, Error>;

#[async_trait]
pub trait MercuryRequest {
    async fn request(session: &Session, uri: &str) -> RequestResult {
        let request = session.mercury().get(uri)?;
        let response = request.await?;
        match response.payload.first() {
            Some(data) => {
                let data = data.to_vec().into();
                trace!("Received metadata: {:?}", data);
                Ok(data)
            }
            None => Err(Error::unavailable(MetadataError::Empty)),
        }
    }
}
