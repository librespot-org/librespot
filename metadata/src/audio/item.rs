use std::fmt::Debug;

use crate::{
    artist::ArtistsWithRole,
    availability::{AudioItemAvailability, Availabilities, UnavailabilityReason},
    episode::Episode,
    error::MetadataError,
    image::{ImageSize, Images},
    restriction::Restrictions,
    track::{Track, Tracks},
    Metadata,
};

use super::file::AudioFiles;

use librespot_core::{
    date::Date, session::UserData, spotify_id::SpotifyItemType, Error, Session, SpotifyId,
};

pub type AudioItemResult = Result<AudioItem, Error>;

#[derive(Debug, Clone)]
pub struct CoverImage {
    pub url: String,
    pub size: ImageSize,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct AudioItem {
    pub track_id: SpotifyId,
    pub uri: String,
    pub files: AudioFiles,
    pub name: String,
    pub covers: Vec<CoverImage>,
    pub language: Vec<String>,
    pub duration_ms: u32,
    pub is_explicit: bool,
    pub availability: AudioItemAvailability,
    pub alternatives: Option<Tracks>,
    pub unique_fields: UniqueFields,
}

#[derive(Debug, Clone)]
pub enum UniqueFields {
    Track {
        artists: ArtistsWithRole,
        album: String,
        album_artists: Vec<String>,
        popularity: u8,
        number: u32,
        disc_number: u32,
    },
    Episode {
        description: String,
        publish_time: Date,
        show_name: String,
    },
}

impl AudioItem {
    pub async fn get_file(session: &Session, id: SpotifyId) -> AudioItemResult {
        let image_url = session
            .get_user_attribute("image-url")
            .unwrap_or_else(|| String::from("https://i.scdn.co/image/{file_id}"));

        match id.item_type {
            SpotifyItemType::Track => {
                let track = Track::get(session, &id).await?;

                if track.duration <= 0 {
                    return Err(Error::unavailable(MetadataError::InvalidDuration(
                        track.duration,
                    )));
                }

                if track.is_explicit && session.filter_explicit_content() {
                    return Err(Error::unavailable(MetadataError::ExplicitContentFiltered));
                }

                let track_id = track.id;
                let uri = track_id.to_uri()?;
                let album = track.album.name;

                let album_artists = track
                    .album
                    .artists
                    .0
                    .into_iter()
                    .map(|a| a.name)
                    .collect::<Vec<String>>();

                let covers = get_covers(track.album.covers, image_url);

                let alternatives = if track.alternatives.is_empty() {
                    None
                } else {
                    Some(track.alternatives)
                };

                let availability = if Date::now_utc() < track.earliest_live_timestamp {
                    Err(UnavailabilityReason::Embargo)
                } else {
                    available_for_user(
                        &session.user_data(),
                        &track.availability,
                        &track.restrictions,
                    )
                };

                let popularity = track.popularity.clamp(0, 100) as u8;
                let number = track.number.max(0) as u32;
                let disc_number = track.disc_number.max(0) as u32;

                let unique_fields = UniqueFields::Track {
                    artists: track.artists_with_role,
                    album,
                    album_artists,
                    popularity,
                    number,
                    disc_number,
                };

                Ok(Self {
                    track_id,
                    uri,
                    files: track.files,
                    name: track.name,
                    covers,
                    language: track.language_of_performance,
                    duration_ms: track.duration as u32,
                    is_explicit: track.is_explicit,
                    availability,
                    alternatives,
                    unique_fields,
                })
            }
            SpotifyItemType::Episode => {
                let episode = Episode::get(session, &id).await?;

                if episode.duration <= 0 {
                    return Err(Error::unavailable(MetadataError::InvalidDuration(
                        episode.duration,
                    )));
                }

                if episode.is_explicit && session.filter_explicit_content() {
                    return Err(Error::unavailable(MetadataError::ExplicitContentFiltered));
                }

                let track_id = episode.id;
                let uri = track_id.to_uri()?;

                let covers = get_covers(episode.covers, image_url);

                let availability = available_for_user(
                    &session.user_data(),
                    &episode.availability,
                    &episode.restrictions,
                );

                let unique_fields = UniqueFields::Episode {
                    description: episode.description,
                    publish_time: episode.publish_time,
                    show_name: episode.show_name,
                };

                Ok(Self {
                    track_id,
                    uri,
                    files: episode.audio,
                    name: episode.name,
                    covers,
                    language: vec![episode.language],
                    duration_ms: episode.duration as u32,
                    is_explicit: episode.is_explicit,
                    availability,
                    alternatives: None,
                    unique_fields,
                })
            }
            _ => Err(Error::unavailable(MetadataError::NonPlayable)),
        }
    }
}

fn get_covers(covers: Images, image_url: String) -> Vec<CoverImage> {
    let mut covers = covers;

    covers.sort_by(|a, b| b.width.cmp(&a.width));

    covers
        .iter()
        .filter_map(|cover| {
            let cover_id = cover.id.to_string();

            if !cover_id.is_empty() {
                let cover_image = CoverImage {
                    url: image_url.replace("{file_id}", &cover_id),
                    size: cover.size,
                    width: cover.width,
                    height: cover.height,
                };

                Some(cover_image)
            } else {
                None
            }
        })
        .collect()
}

fn allowed_for_user(user_data: &UserData, restrictions: &Restrictions) -> AudioItemAvailability {
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
        .any(|availability| Date::now_utc() >= availability.start))
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
    available(availability)?;
    allowed_for_user(user_data, restrictions)?;
    Ok(())
}
