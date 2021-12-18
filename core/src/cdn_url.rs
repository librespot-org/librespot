use chrono::Local;
use protobuf::{Message, ProtobufError};
use thiserror::Error;
use url::Url;

use std::convert::{TryFrom, TryInto};
use std::ops::{Deref, DerefMut};

use super::date::Date;
use super::file_id::FileId;
use super::session::Session;
use super::spclient::SpClientError;

use librespot_protocol as protocol;
use protocol::storage_resolve::StorageResolveResponse as CdnUrlMessage;
use protocol::storage_resolve::StorageResolveResponse_Result;

#[derive(Error, Debug)]
pub enum CdnUrlError {
    #[error("no URLs available")]
    Empty,
    #[error("all tokens expired")]
    Expired,
    #[error("error parsing response")]
    Parsing,
    #[error("could not parse protobuf: {0}")]
    Protobuf(#[from] ProtobufError),
    #[error("could not complete API request: {0}")]
    SpClient(#[from] SpClientError),
}

#[derive(Debug, Clone)]
pub struct MaybeExpiringUrl(pub String, pub Option<Date>);

#[derive(Debug, Clone)]
pub struct MaybeExpiringUrls(pub Vec<MaybeExpiringUrl>);

impl Deref for MaybeExpiringUrls {
    type Target = Vec<MaybeExpiringUrl>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaybeExpiringUrls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct CdnUrl {
    pub file_id: FileId,
    pub urls: MaybeExpiringUrls,
}

impl CdnUrl {
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            urls: MaybeExpiringUrls(Vec::new()),
        }
    }

    pub async fn resolve_audio(&self, session: &Session) -> Result<Self, CdnUrlError> {
        let file_id = self.file_id;
        let response = session.spclient().get_audio_urls(file_id).await?;
        let msg = CdnUrlMessage::parse_from_bytes(&response)?;
        let urls = MaybeExpiringUrls::try_from(msg)?;

        let cdn_url = Self { file_id, urls };

        trace!("Resolved CDN storage: {:#?}", cdn_url);

        Ok(cdn_url)
    }

    pub fn get_url(&mut self) -> Result<&str, CdnUrlError> {
        if self.urls.is_empty() {
            return Err(CdnUrlError::Empty);
        }

        // prune expired URLs until the first one is current, or none are left
        let now = Local::now();
        while !self.urls.is_empty() {
            let maybe_expiring = self.urls[0].1;
            if let Some(expiry) = maybe_expiring {
                if now < expiry.as_utc() {
                    break;
                } else {
                    self.urls.remove(0);
                }
            }
        }

        if let Some(cdn_url) = self.urls.first() {
            Ok(&cdn_url.0)
        } else {
            Err(CdnUrlError::Expired)
        }
    }
}

impl TryFrom<CdnUrlMessage> for MaybeExpiringUrls {
    type Error = CdnUrlError;
    fn try_from(msg: CdnUrlMessage) -> Result<Self, Self::Error> {
        if !matches!(msg.get_result(), StorageResolveResponse_Result::CDN) {
            return Err(CdnUrlError::Parsing);
        }

        let is_expiring = !msg.get_fileid().is_empty();

        let result = msg
            .get_cdnurl()
            .iter()
            .map(|cdn_url| {
                let url = Url::parse(cdn_url).map_err(|_| CdnUrlError::Parsing)?;

                if is_expiring {
                    let expiry_str = if let Some(token) = url
                        .query_pairs()
                        .into_iter()
                        .find(|(key, _value)| key == "__token__")
                    {
                        let start = token.1.find("exp=").ok_or(CdnUrlError::Parsing)?;
                        let slice = &token.1[start + 4..];
                        let end = slice.find('~').ok_or(CdnUrlError::Parsing)?;
                        String::from(&slice[..end])
                    } else if let Some(query) = url.query() {
                        let mut items = query.split('_');
                        String::from(items.next().ok_or(CdnUrlError::Parsing)?)
                    } else {
                        return Err(CdnUrlError::Parsing);
                    };

                    let mut expiry: i64 = expiry_str.parse().map_err(|_| CdnUrlError::Parsing)?;
                    expiry -= 5 * 60; // seconds

                    Ok(MaybeExpiringUrl(
                        cdn_url.to_owned(),
                        Some(expiry.try_into().map_err(|_| CdnUrlError::Parsing)?),
                    ))
                } else {
                    Ok(MaybeExpiringUrl(cdn_url.to_owned(), None))
                }
            })
            .collect::<Result<Vec<MaybeExpiringUrl>, CdnUrlError>>()?;

        Ok(Self(result))
    }
}
