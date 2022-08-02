use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{from_repeated_message, impl_deref_wrapped};

use librespot_core::FileId;

use librespot_protocol as protocol;
use protocol::metadata::VideoFile as VideoFileMessage;

#[derive(Debug, Clone, Default)]
pub struct VideoFiles(pub Vec<FileId>);

impl_deref_wrapped!(VideoFiles, Vec<FileId>);

from_repeated_message!(VideoFileMessage, VideoFiles);
