use crate::error::RequestError;

use librespot_core::session::Session;

pub type RequestResult = Result<bytes::Bytes, RequestError>;

#[async_trait]
pub trait MercuryRequest {
    async fn request(session: &Session, uri: &str) -> RequestResult {
        let response = session.mercury().get(uri).await?;
        match response.payload.first() {
            Some(data) => {
                let data = data.to_vec().into();
                trace!("Received metadata: {:?}", data);
                Ok(data)
            }
            None => Err(RequestError::Empty),
        }
    }
}
