use std::{
    convert::TryFrom,
    ops::{Deref, DerefMut},
};

use protobuf::Message;
use thiserror::Error;
use url::Url;

use super::{date::Date, Error, FileId, Session};

use librespot_protocol as protocol;
use protocol::storage_resolve::storage_resolve_response::Result as StorageResolveResponse_Result;
use protocol::storage_resolve::StorageResolveResponse as CdnUrlMessage;

#[derive(Debug, Clone)]
pub struct MaybeExpiringUrl(pub String, pub Option<Date>);

const CDN_URL_EXPIRY_MARGIN_SECONDS: i64 = 5 * 60;

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
    #[error("no URLs resolved")]
    Unresolved,
}

impl From<CdnUrlError> for Error {
    fn from(err: CdnUrlError) -> Self {
        match err {
            CdnUrlError::Expired => Error::deadline_exceeded(err),
            CdnUrlError::Storage | CdnUrlError::Unresolved => Error::unavailable(err),
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
        let response = session.spclient().get_audio_storage(&file_id).await?;
        let msg = CdnUrlMessage::parse_from_bytes(&response)?;
        let urls = MaybeExpiringUrls::try_from(msg)?;

        let cdn_url = Self { file_id, urls };

        trace!("Resolved CDN storage: {:#?}", cdn_url);

        Ok(cdn_url)
    }

    pub fn try_get_url(&self) -> Result<&str, Error> {
        if self.urls.is_empty() {
            return Err(CdnUrlError::Unresolved.into());
        }

        let now = Date::now_utc();
        let url = self.urls.iter().find(|url| match url.1 {
            Some(expiry) => now < expiry,
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
        if !matches!(
            msg.result.enum_value_or_default(),
            StorageResolveResponse_Result::CDN
        ) {
            return Err(CdnUrlError::Storage.into());
        }

        let is_expiring = !msg.fileid.is_empty();

        let result = msg
            .cdnurl
            .iter()
            .map(|cdn_url| {
                let url = Url::parse(cdn_url)?;

                if is_expiring {
                    let expiry_str = if let Some(token) = url
                        .query_pairs()
                        .into_iter()
                        .find(|(key, _value)| key == "__token__")
                    {
                        //"https://audio-ak-spotify-com.akamaized.net/audio/4712bc9e47f7feb4ee3450ef2bb545e1d83c3d54?__token__=exp=1688165560~hmac=4e661527574fab5793adb99cf04e1c2ce12294c71fe1d39ffbfabdcfe8ce3b41",
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
                    } else if let Some(token) = url
                        .query_pairs()
                        .into_iter()
                        .find(|(key, _value)| key == "Expires")
                    {
                        //"https://audio-gm-off.spotifycdn.com/audio/4712bc9e47f7feb4ee3450ef2bb545e1d83c3d54?Expires=1688165560~FullPath~hmac=IIZA28qptl8cuGLq15-SjHKHtLoxzpy_6r_JpAU4MfM=",
                        if let Some(end) = token.1.find('~') {
                            // this is the only valid invariant for spotifycdn.com
                            let slice = &token.1[..end];
                            String::from(&slice[..end])
                        } else {
                            String::new()
                        }
                    } else if let Some(query) = url.query() {
                        //"https://audio4-fa.scdn.co/audio/4712bc9e47f7feb4ee3450ef2bb545e1d83c3d54?1688165560_0GKSyXjLaTW1BksFOyI4J7Tf9tZDbBUNNPu9Mt4mhH4=",
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

                    let expiry = match expiry_str.parse::<i64>() {
                        Ok(exp) => {
                            let exp = exp - CDN_URL_EXPIRY_MARGIN_SECONDS;
                            Some(Date::from_timestamp_ms(exp * 1000)?)
                        }
                        Err(_) => {
                            warn!("Unknown CDN URL format: {:#?}", cdn_url);
                            None
                        }
                    };
                    Ok(MaybeExpiringUrl(cdn_url.to_owned(), expiry))
                } else {
                    Ok(MaybeExpiringUrl(cdn_url.to_owned(), None))
                }
            })
            .collect::<Result<Vec<MaybeExpiringUrl>, Error>>()?;

        Ok(Self(result))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maybe_expiring_urls() {
        let timestamp = 1688165560;
        let mut msg = CdnUrlMessage::new();
        msg.result = StorageResolveResponse_Result::CDN.into();
        msg.cdnurl = vec![
            format!("https://audio-ak-spotify-com.akamaized.net/audio/foo?__token__=exp={timestamp}~hmac=4e661527574fab5793adb99cf04e1c2ce12294c71fe1d39ffbfabdcfe8ce3b41"),
            format!("https://audio-gm-off.spotifycdn.com/audio/foo?Expires={timestamp}~FullPath~hmac=IIZA28qptl8cuGLq15-SjHKHtLoxzpy_6r_JpAU4MfM="),
            format!("https://audio4-fa.scdn.co/audio/foo?{timestamp}_0GKSyXjLaTW1BksFOyI4J7Tf9tZDbBUNNPu9Mt4mhH4="),
            "https://audio4-fa.scdn.co/foo?baz".to_string(),
        ];
        msg.fileid = vec![0];

        let urls = MaybeExpiringUrls::try_from(msg).expect("valid urls");
        assert_eq!(urls.len(), 4);
        assert!(urls[0].1.is_some());
        assert!(urls[1].1.is_some());
        assert!(urls[2].1.is_some());
        assert!(urls[3].1.is_none());
        let timestamp_margin = timestamp - CDN_URL_EXPIRY_MARGIN_SECONDS;
        assert_eq!(
            urls[0].1.unwrap().as_timestamp_ms(),
            timestamp_margin * 1000
        );
    }
}
