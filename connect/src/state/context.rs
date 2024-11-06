use crate::state::consts::METADATA_ENTITY_URI;
use crate::state::provider::Provider;
use crate::state::{ConnectState, StateError, METADATA_CONTEXT_URI};
use librespot_core::{Error, SpotifyId};
use librespot_protocol::player::{Context, ContextIndex, ContextTrack, ProvidedTrack};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StateContext {
    pub tracks: Vec<ProvidedTrack>,
    pub metadata: HashMap<String, String>,
    pub index: ContextIndex,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum ContextType {
    #[default]
    Default,
    Shuffle,
    Autoplay,
}

impl ConnectState {
    pub fn find_index_in_context<F: Fn(&ProvidedTrack) -> bool>(
        context: Option<&StateContext>,
        f: F,
    ) -> Result<usize, StateError> {
        let ctx = context
            .as_ref()
            .ok_or(StateError::NoContext(ContextType::Default))?;

        ctx.tracks
            .iter()
            .position(f)
            .ok_or(StateError::CanNotFindTrackInContext(None, ctx.tracks.len()))
    }

    pub(super) fn get_current_context(&self) -> Result<&StateContext, StateError> {
        match self.active_context {
            ContextType::Default => self.context.as_ref(),
            ContextType::Shuffle => self.shuffle_context.as_ref(),
            ContextType::Autoplay => self.autoplay_context.as_ref(),
        }
        .ok_or(StateError::NoContext(self.active_context))
    }

    pub fn context_uri(&self) -> &String {
        &self.player.context_uri
    }

    pub fn reset_context(&mut self, new_context: Option<&str>) {
        self.active_context = ContextType::Default;

        self.autoplay_context = None;
        self.shuffle_context = None;

        if matches!(new_context, Some(ctx) if self.player.context_uri != ctx) {
            self.context = None;
        } else if let Some(ctx) = self.context.as_mut() {
            ctx.index.track = 0;
            ctx.index.page = 0;
        }

        self.update_restrictions()
    }

    pub fn update_context(&mut self, mut context: Context) -> Result<(), Error> {
        debug!("context: {}, {}", context.uri, context.url);

        self.player.context_url = format!("context://{}", context.uri);
        self.player.context_uri = context.uri.clone();

        if context.restrictions.is_some() {
            self.player.context_restrictions = context.restrictions;
        }

        if !context.metadata.is_empty() {
            self.player.context_metadata = context.metadata;
        }

        let page = match context.pages.pop() {
            None => return Ok(()),
            Some(page) => page,
        };

        let tracks = page
            .tracks
            .iter()
            .flat_map(|track| {
                match self.context_to_provided_track(track, context.uri.clone(), None) {
                    Ok(t) => Some(t),
                    Err(_) => {
                        error!("couldn't convert {track:#?} into ProvidedTrack");
                        None
                    }
                }
            })
            .collect();

        self.context = Some(StateContext {
            tracks,
            metadata: page.metadata,
            index: ContextIndex::new(),
        });

        Ok(())
    }

    pub fn update_autoplay_context(&mut self, mut context: Context) -> Result<(), Error> {
        debug!(
            "autoplay-context: {}, pages: {}",
            context.uri,
            context.pages.len()
        );
        let page = context
            .pages
            .pop()
            .ok_or(StateError::NoContext(ContextType::Autoplay))?;
        debug!("autoplay-context size: {}", page.tracks.len());

        let tracks = page
            .tracks
            .iter()
            .flat_map(|track| {
                match self.context_to_provided_track(
                    track,
                    context.uri.clone(),
                    Some(Provider::Autoplay),
                ) {
                    Ok(t) => Some(t),
                    Err(_) => {
                        error!("couldn't convert {track:#?} into ProvidedTrack");
                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        // add the tracks to the context if we already have an autoplay context
        if let Some(autoplay_context) = self.autoplay_context.as_mut() {
            for track in tracks {
                autoplay_context.tracks.push(track)
            }
        } else {
            self.autoplay_context = Some(StateContext {
                tracks,
                metadata: page.metadata,
                index: ContextIndex::new(),
            })
        }

        Ok(())
    }

    pub(super) fn update_context_index(&mut self, new_index: usize) -> Result<(), StateError> {
        let context = match self.active_context {
            ContextType::Default => self.context.as_mut(),
            ContextType::Shuffle => self.shuffle_context.as_mut(),
            ContextType::Autoplay => self.autoplay_context.as_mut(),
        }
        .ok_or(StateError::NoContext(self.active_context))?;

        context.index.track = new_index as u32;
        Ok(())
    }

    pub fn context_to_provided_track(
        &self,
        ctx_track: &ContextTrack,
        context_uri: String,
        provider: Option<Provider>,
    ) -> Result<ProvidedTrack, Error> {
        let provider = if self.unavailable_uri.contains(&ctx_track.uri) {
            Provider::Unavailable
        } else {
            provider.unwrap_or(Provider::Context)
        };

        let id = if !ctx_track.uri.is_empty() {
            SpotifyId::from_uri(&ctx_track.uri)
        } else if !ctx_track.gid.is_empty() {
            SpotifyId::from_raw(&ctx_track.gid)
        } else {
            return Err(Error::unavailable("track not available"));
        }?;

        let mut metadata = HashMap::new();
        metadata.insert(METADATA_CONTEXT_URI.to_string(), context_uri.to_string());
        metadata.insert(METADATA_ENTITY_URI.to_string(), context_uri.to_string());

        if !ctx_track.metadata.is_empty() {
            for (k, v) in &ctx_track.metadata {
                metadata.insert(k.to_string(), v.to_string());
            }
        }

        let uid = if !ctx_track.uid.is_empty() {
            ctx_track.uid.clone()
        } else {
            // todo: this will never work, it is sadly not as simple :/
            String::from_utf8(id.to_raw().to_vec()).unwrap_or_else(|_| String::new())
        };

        Ok(ProvidedTrack {
            uri: id.to_uri()?.replace("unknown", "track"),
            uid,
            metadata,
            provider: provider.to_string(),
            ..Default::default()
        })
    }
}
