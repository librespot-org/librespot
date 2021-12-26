use std::{
    convert::{TryFrom, TryInto},
    ops::{Deref, DerefMut},
};

use chrono::Local;
use protobuf::Message;
use thiserror::Error;
use url::Url;

use super::{date::Date, Error, FileId, Session};

use librespot_protocol as protocol;
use protocol::storage_resolve::StorageResolveResponse as CdnUrlMessage;
use protocol::storage_resolve::StorageResolveResponse_Result;

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

#[derive(Debug, Error)]
pub enum CdnUrlError {
    #[error("all URLs expired")]
    Expired,
    #[error("resolved storage is not for CDN")]
    Storage,
}

impl From<CdnUrlError> for Error {
    fn from(err: CdnUrlError) -> Self {
        match err {
            CdnUrlError::Expired => Error::deadline_exceeded(err),
            CdnUrlError::Storage => Error::unavailable(err),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CdnUrl {
    pub file_id: FileId,
    urls: MaybeExpiringUrls,
}

impl CdnUrl {
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            urls: MaybeExpiringUrls(Vec::new()),
        }
    }

    pub async fn resolve_audio(&self, session: &Session) -> Result<Self, Error> {
        let file_id = self.file_id;
        let response = session.spclient().get_audio_urls(file_id).await?;
        let msg = CdnUrlMessage::parse_from_bytes(&response)?;
        let urls = MaybeExpiringUrls::try_from(msg)?;

        let cdn_url = Self { file_id, urls };

        trace!("Resolved CDN storage: {:#?}", cdn_url);

        Ok(cdn_url)
    }

    pub fn try_get_url(&self) -> Result<&str, Error> {
        let now = Local::now();
        let url = self.urls.iter().find(|url| match url.1 {
            Some(expiry) => now < expiry.as_utc(),
            None => true,
        });

        if let Some(url) = url {
            Ok(&url.0)
        } else {
            Err(CdnUrlError::Expired.into())
        }
    }
}

impl TryFrom<CdnUrlMessage> for MaybeExpiringUrls {
    type Error = crate::Error;
    fn try_from(msg: CdnUrlMessage) -> Result<Self, Self::Error> {
        if !matches!(msg.get_result(), StorageResolveResponse_Result::CDN) {
            return Err(CdnUrlError::Storage.into());
        }

        let is_expiring = !msg.get_fileid().is_empty();

        let result = msg
            .get_cdnurl()
            .iter()
            .map(|cdn_url| {
                let url = Url::parse(cdn_url)?;

                if is_expiring {
                    let expiry_str = if let Some(token) = url
                        .query_pairs()
                        .into_iter()
                        .find(|(key, _value)| key == "__token__")
                    {
                        if let Some(mut start) = token.1.find("exp=") {
                            start += 4;
                            if token.1.len() >= start {
                                let slice = &token.1[start..];
                                if let Some(end) = slice.find('~') {
                                    // this is the only valid invariant for akamaized.net
                                    String::from(&slice[..end])
                                } else {
                                    String::from(slice)
                                }
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        }
                    } else if let Some(query) = url.query() {
                        let mut items = query.split('_');
                        if let Some(first) = items.next() {
                            // this is the only valid invariant for scdn.co
                            String::from(first)
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    let mut expiry: i64 = expiry_str.parse()?;

                    expiry -= 5 * 60; // seconds

                    Ok(MaybeExpiringUrl(
                        cdn_url.to_owned(),
                        Some(expiry.try_into()?),
                    ))
                } else {
                    Ok(MaybeExpiringUrl(cdn_url.to_owned(), None))
                }
            })
            .collect::<Result<Vec<MaybeExpiringUrl>, Error>>()?;

        Ok(Self(result))
    }
}
