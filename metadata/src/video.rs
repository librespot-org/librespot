use std::{fmt::Debug, ops::Deref};

use crate::util::from_repeated_message;

use librespot_core::FileId;

use librespot_protocol as protocol;
use protocol::metadata::VideoFile as VideoFileMessage;

#[derive(Debug, Clone, Default)]
pub struct VideoFiles(pub Vec<FileId>);

impl Deref for VideoFiles {
    type Target = Vec<FileId>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

from_repeated_message!(VideoFileMessage, VideoFiles);
