use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::ops::Deref;

use crate::{
    error::MetadataError,
    util::{from_repeated_message, try_from_repeated_message},
};

use librespot_core::file_id::FileId;
use librespot_core::spotify_id::SpotifyId;
use librespot_protocol as protocol;

use protocol::metadata::Image as ImageMessage;
use protocol::playlist4_external::PictureSize as PictureSizeMessage;
use protocol::playlist_annotate3::TranscodedPicture as TranscodedPictureMessage;

pub use protocol::metadata::Image_Size as ImageSize;

#[derive(Debug, Clone)]
pub struct Image {
    pub id: FileId,
    pub size: ImageSize,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct Images(pub Vec<Image>);

impl Deref for Images {
    type Target = Vec<Image>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct PictureSize {
    pub target_name: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct PictureSizes(pub Vec<PictureSize>);

impl Deref for PictureSizes {
    type Target = Vec<PictureSize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct TranscodedPicture {
    pub target_name: String,
    pub uri: SpotifyId,
}

#[derive(Debug, Clone)]
pub struct TranscodedPictures(pub Vec<TranscodedPicture>);

impl Deref for TranscodedPictures {
    type Target = Vec<TranscodedPicture>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&ImageMessage> for Image {
    fn from(image: &ImageMessage) -> Self {
        Self {
            id: image.into(),
            size: image.get_size(),
            width: image.get_width(),
            height: image.get_height(),
        }
    }
}

from_repeated_message!(ImageMessage, Images);

impl From<&PictureSizeMessage> for PictureSize {
    fn from(size: &PictureSizeMessage) -> Self {
        Self {
            target_name: size.get_target_name().to_owned(),
            url: size.get_url().to_owned(),
        }
    }
}

from_repeated_message!(PictureSizeMessage, PictureSizes);

impl TryFrom<&TranscodedPictureMessage> for TranscodedPicture {
    type Error = MetadataError;
    fn try_from(picture: &TranscodedPictureMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            target_name: picture.get_target_name().to_owned(),
            uri: picture.try_into()?,
        })
    }
}

try_from_repeated_message!(TranscodedPictureMessage, TranscodedPictures);
