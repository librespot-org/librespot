use crate::{
    context_player_options::ContextPlayerOptions,
    play_origin::PlayOrigin,
    player::{
        ContextPlayerOptions as PlayerContextPlayerOptions,
        ModeRestrictions as PlayerModeRestrictions, PlayOrigin as PlayerPlayOrigin,
        RestrictionReasons as PlayerRestrictionReasons, Restrictions as PlayerRestrictions,
        Suppressions as PlayerSuppressions,
    },
    restrictions::{ModeRestrictions, RestrictionReasons, Restrictions},
    suppressions::Suppressions,
};
use std::collections::HashMap;

fn hashmap_into<T: Into<V>, V>(map: HashMap<String, T>) -> HashMap<String, V> {
    map.into_iter().map(|(k, v)| (k, v.into())).collect()
}

impl From<ContextPlayerOptions> for PlayerContextPlayerOptions {
    fn from(value: ContextPlayerOptions) -> Self {
        PlayerContextPlayerOptions {
            shuffling_context: value.shuffling_context.unwrap_or_default(),
            repeating_context: value.repeating_context.unwrap_or_default(),
            repeating_track: value.repeating_track.unwrap_or_default(),
            modes: value.modes,
            playback_speed: value.playback_speed,
            special_fields: value.special_fields,
        }
    }
}

impl From<PlayerRestrictions> for Restrictions {
    fn from(value: PlayerRestrictions) -> Self {
        Restrictions {
            disallow_pausing_reasons: value.disallow_pausing_reasons,
            disallow_resuming_reasons: value.disallow_resuming_reasons,
            disallow_seeking_reasons: value.disallow_seeking_reasons,
            disallow_peeking_prev_reasons: value.disallow_peeking_prev_reasons,
            disallow_peeking_next_reasons: value.disallow_peeking_next_reasons,
            disallow_skipping_prev_reasons: value.disallow_skipping_prev_reasons,
            disallow_skipping_next_reasons: value.disallow_skipping_next_reasons,
            disallow_toggling_repeat_context_reasons: value
                .disallow_toggling_repeat_context_reasons,
            disallow_toggling_repeat_track_reasons: value.disallow_toggling_repeat_track_reasons,
            disallow_toggling_shuffle_reasons: value.disallow_toggling_shuffle_reasons,
            disallow_set_queue_reasons: value.disallow_set_queue_reasons,
            disallow_interrupting_playback_reasons: value.disallow_interrupting_playback_reasons,
            disallow_transferring_playback_reasons: value.disallow_transferring_playback_reasons,
            disallow_remote_control_reasons: value.disallow_remote_control_reasons,
            disallow_inserting_into_next_tracks_reasons: value
                .disallow_inserting_into_next_tracks_reasons,
            disallow_inserting_into_context_tracks_reasons: value
                .disallow_inserting_into_context_tracks_reasons,
            disallow_reordering_in_next_tracks_reasons: value
                .disallow_reordering_in_next_tracks_reasons,
            disallow_reordering_in_context_tracks_reasons: value
                .disallow_reordering_in_context_tracks_reasons,
            disallow_removing_from_next_tracks_reasons: value
                .disallow_removing_from_next_tracks_reasons,
            disallow_removing_from_context_tracks_reasons: value
                .disallow_removing_from_context_tracks_reasons,
            disallow_updating_context_reasons: value.disallow_updating_context_reasons,
            disallow_add_to_queue_reasons: value.disallow_add_to_queue_reasons,
            disallow_setting_playback_speed: value.disallow_setting_playback_speed_reasons,
            disallow_setting_modes: hashmap_into(value.disallow_setting_modes),
            disallow_signals: hashmap_into(value.disallow_signals),
            special_fields: value.special_fields,
        }
    }
}

impl From<Restrictions> for PlayerRestrictions {
    fn from(value: Restrictions) -> Self {
        PlayerRestrictions {
            disallow_pausing_reasons: value.disallow_pausing_reasons,
            disallow_resuming_reasons: value.disallow_resuming_reasons,
            disallow_seeking_reasons: value.disallow_seeking_reasons,
            disallow_peeking_prev_reasons: value.disallow_peeking_prev_reasons,
            disallow_peeking_next_reasons: value.disallow_peeking_next_reasons,
            disallow_skipping_prev_reasons: value.disallow_skipping_prev_reasons,
            disallow_skipping_next_reasons: value.disallow_skipping_next_reasons,
            disallow_toggling_repeat_context_reasons: value
                .disallow_toggling_repeat_context_reasons,
            disallow_toggling_repeat_track_reasons: value.disallow_toggling_repeat_track_reasons,
            disallow_toggling_shuffle_reasons: value.disallow_toggling_shuffle_reasons,
            disallow_set_queue_reasons: value.disallow_set_queue_reasons,
            disallow_interrupting_playback_reasons: value.disallow_interrupting_playback_reasons,
            disallow_transferring_playback_reasons: value.disallow_transferring_playback_reasons,
            disallow_remote_control_reasons: value.disallow_remote_control_reasons,
            disallow_inserting_into_next_tracks_reasons: value
                .disallow_inserting_into_next_tracks_reasons,
            disallow_inserting_into_context_tracks_reasons: value
                .disallow_inserting_into_context_tracks_reasons,
            disallow_reordering_in_next_tracks_reasons: value
                .disallow_reordering_in_next_tracks_reasons,
            disallow_reordering_in_context_tracks_reasons: value
                .disallow_reordering_in_context_tracks_reasons,
            disallow_removing_from_next_tracks_reasons: value
                .disallow_removing_from_next_tracks_reasons,
            disallow_removing_from_context_tracks_reasons: value
                .disallow_removing_from_context_tracks_reasons,
            disallow_updating_context_reasons: value.disallow_updating_context_reasons,
            disallow_add_to_queue_reasons: value.disallow_add_to_queue_reasons,
            disallow_setting_playback_speed_reasons: value.disallow_setting_playback_speed,
            disallow_setting_modes: hashmap_into(value.disallow_setting_modes),
            disallow_signals: hashmap_into(value.disallow_signals),
            disallow_playing_reasons: vec![],
            disallow_stopping_reasons: vec![],
            special_fields: value.special_fields,
        }
    }
}

impl From<PlayerModeRestrictions> for ModeRestrictions {
    fn from(value: PlayerModeRestrictions) -> Self {
        ModeRestrictions {
            values: hashmap_into(value.values),
            special_fields: value.special_fields,
        }
    }
}

impl From<ModeRestrictions> for PlayerModeRestrictions {
    fn from(value: ModeRestrictions) -> Self {
        PlayerModeRestrictions {
            values: hashmap_into(value.values),
            special_fields: value.special_fields,
        }
    }
}

impl From<PlayerRestrictionReasons> for RestrictionReasons {
    fn from(value: PlayerRestrictionReasons) -> Self {
        RestrictionReasons {
            reasons: value.reasons,
            special_fields: value.special_fields,
        }
    }
}

impl From<RestrictionReasons> for PlayerRestrictionReasons {
    fn from(value: RestrictionReasons) -> Self {
        PlayerRestrictionReasons {
            reasons: value.reasons,
            special_fields: value.special_fields,
        }
    }
}

impl From<PlayOrigin> for PlayerPlayOrigin {
    fn from(value: PlayOrigin) -> Self {
        PlayerPlayOrigin {
            feature_identifier: value.feature_identifier.unwrap_or_default(),
            feature_version: value.feature_version.unwrap_or_default(),
            view_uri: value.view_uri.unwrap_or_default(),
            external_referrer: value.external_referrer.unwrap_or_default(),
            referrer_identifier: value.referrer_identifier.unwrap_or_default(),
            device_identifier: value.device_identifier.unwrap_or_default(),
            feature_classes: value.feature_classes,
            restriction_identifier: value.restriction_identifier.unwrap_or_default(),
            special_fields: value.special_fields,
        }
    }
}

impl From<Suppressions> for PlayerSuppressions {
    fn from(value: Suppressions) -> Self {
        PlayerSuppressions {
            providers: value.providers,
            special_fields: value.special_fields,
        }
    }
}
