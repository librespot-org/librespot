use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{impl_deref_wrapped, impl_from_repeated, impl_try_from_repeated};

use librespot_core::{FileId, SpotifyId};

use librespot_protocol as protocol;
pub use protocol::metadata::image::Size as ImageSize;
use protocol::metadata::Image as ImageMessage;
use protocol::metadata::ImageGroup;
use protocol::playlist4_external::PictureSize as PictureSizeMessage;
use protocol::playlist_annotate3::TranscodedPicture as TranscodedPictureMessage;

#[derive(Debug, Clone)]
pub struct Image {
    pub id: FileId,
    pub size: ImageSize,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Images(pub Vec<Image>);

impl From<&ImageGroup> for Images {
    fn from(image_group: &ImageGroup) -> Self {
        Self(image_group.image.iter().map(|i| i.into()).collect())
    }
}

impl_deref_wrapped!(Images, Vec<Image>);

#[derive(Debug, Clone)]
pub struct PictureSize {
    pub target_name: String,
    pub url: String,
}

#[derive(Debug, Clone, Default)]
pub struct PictureSizes(pub Vec<PictureSize>);

impl_deref_wrapped!(PictureSizes, Vec<PictureSize>);

#[derive(Debug, Clone)]
pub struct TranscodedPicture {
    pub target_name: String,
    pub uri: SpotifyId,
}

#[derive(Debug, Clone)]
pub struct TranscodedPictures(pub Vec<TranscodedPicture>);

impl_deref_wrapped!(TranscodedPictures, Vec<TranscodedPicture>);

impl From<&ImageMessage> for Image {
    fn from(image: &ImageMessage) -> Self {
        Self {
            id: image.into(),
            size: image.size(),
            width: image.width(),
            height: image.height(),
        }
    }
}

impl_from_repeated!(ImageMessage, Images);

impl From<&PictureSizeMessage> for PictureSize {
    fn from(size: &PictureSizeMessage) -> Self {
        Self {
            target_name: size.target_name().to_owned(),
            url: size.url().to_owned(),
        }
    }
}

impl_from_repeated!(PictureSizeMessage, PictureSizes);

impl TryFrom<&TranscodedPictureMessage> for TranscodedPicture {
    type Error = librespot_core::Error;
    fn try_from(picture: &TranscodedPictureMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            target_name: picture.target_name().to_owned(),
            uri: picture.try_into()?,
        })
    }
}

impl_try_from_repeated!(TranscodedPictureMessage, TranscodedPictures);
