use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{impl_deref_wrapped, impl_from_repeated_copy};

use librespot_protocol as protocol;
use protocol::playlist_permission::Capabilities as CapabilitiesMessage;
use protocol::playlist_permission::PermissionLevel;

#[derive(Debug, Clone)]
pub struct Capabilities {
    pub can_view: bool,
    pub can_administrate_permissions: bool,
    pub grantable_levels: PermissionLevels,
    pub can_edit_metadata: bool,
    pub can_edit_items: bool,
    pub can_cancel_membership: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PermissionLevels(pub Vec<PermissionLevel>);

impl_deref_wrapped!(PermissionLevels, Vec<PermissionLevel>);

impl From<&CapabilitiesMessage> for Capabilities {
    fn from(playlist: &CapabilitiesMessage) -> Self {
        Self {
            can_view: playlist.get_can_view(),
            can_administrate_permissions: playlist.get_can_administrate_permissions(),
            grantable_levels: playlist.get_grantable_level().into(),
            can_edit_metadata: playlist.get_can_edit_metadata(),
            can_edit_items: playlist.get_can_edit_items(),
            can_cancel_membership: playlist.get_can_cancel_membership(),
        }
    }
}

impl_from_repeated_copy!(PermissionLevel, PermissionLevels);
