use std::fmt::Debug;

use chrono::Local;

use crate::{
    availability::{AudioItemAvailability, Availabilities, UnavailabilityReason},
    episode::Episode,
    error::MetadataError,
    restriction::Restrictions,
    track::{Track, Tracks},
};

use super::file::AudioFiles;

use librespot_core::session::{Session, UserData};
use librespot_core::spotify_id::{SpotifyId, SpotifyItemType};

pub type AudioItemResult = Result<AudioItem, MetadataError>;

// A wrapper with fields the player needs
#[derive(Debug, Clone)]
pub struct AudioItem {
    pub id: SpotifyId,
    pub spotify_uri: String,
    pub files: AudioFiles,
    pub name: String,
    pub duration: i32,
    pub availability: AudioItemAvailability,
    pub alternatives: Option<Tracks>,
}

impl AudioItem {
    pub async fn get_file(session: &Session, id: SpotifyId) -> AudioItemResult {
        match id.item_type {
            SpotifyItemType::Track => Track::get_audio_item(session, id).await,
            SpotifyItemType::Episode => Episode::get_audio_item(session, id).await,
            _ => Err(MetadataError::NonPlayable),
        }
    }
}

#[async_trait]
pub trait InnerAudioItem {
    async fn get_audio_item(session: &Session, id: SpotifyId) -> AudioItemResult;

    fn allowed_for_user(
        user_data: &UserData,
        restrictions: &Restrictions,
    ) -> AudioItemAvailability {
        let country = &user_data.country;
        let user_catalogue = match user_data.attributes.get("catalogue") {
            Some(catalogue) => catalogue,
            None => "premium",
        };

        for premium_restriction in restrictions.iter().filter(|restriction| {
            restriction
                .catalogue_strs
                .iter()
                .any(|restricted_catalogue| restricted_catalogue == user_catalogue)
        }) {
            if let Some(allowed_countries) = &premium_restriction.countries_allowed {
                // A restriction will specify either a whitelast *or* a blacklist,
                // but not both. So restrict availability if there is a whitelist
                // and the country isn't on it.
                if allowed_countries.iter().any(|allowed| country == allowed) {
                    return Ok(());
                } else {
                    return Err(UnavailabilityReason::NotWhitelisted);
                }
            }

            if let Some(forbidden_countries) = &premium_restriction.countries_forbidden {
                if forbidden_countries
                    .iter()
                    .any(|forbidden| country == forbidden)
                {
                    return Err(UnavailabilityReason::Blacklisted);
                } else {
                    return Ok(());
                }
            }
        }

        Ok(()) // no restrictions in place
    }

    fn available(availability: &Availabilities) -> AudioItemAvailability {
        if availability.is_empty() {
            // not all items have availability specified
            return Ok(());
        }

        if !(availability
            .iter()
            .any(|availability| Local::now() >= availability.start.as_utc()))
        {
            return Err(UnavailabilityReason::Embargo);
        }

        Ok(())
    }

    fn available_for_user(
        user_data: &UserData,
        availability: &Availabilities,
        restrictions: &Restrictions,
    ) -> AudioItemAvailability {
        Self::available(availability)?;
        Self::allowed_for_user(user_data, restrictions)?;
        Ok(())
    }
}
