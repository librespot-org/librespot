use crate::state::metadata::Metadata;
use crate::state::provider::Provider;
use crate::state::{ConnectState, StateError};
use librespot_core::{Error, SpotifyId};
use librespot_protocol::player::{Context, ContextIndex, ContextPage, ContextTrack, ProvidedTrack};
use std::collections::HashMap;
use uuid::Uuid;

const LOCAL_FILES_IDENTIFIER: &str = "spotify:local-files";
const SEARCH_IDENTIFIER: &str = "spotify:search";

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

pub enum LoadNext {
    Done,
    PageUrl(String),
    Empty,
}

#[derive(Debug)]
pub enum UpdateContext {
    Default,
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

    pub(super) fn get_context(&self, ty: &ContextType) -> Result<&StateContext, StateError> {
        match ty {
            ContextType::Default => self.context.as_ref(),
            ContextType::Shuffle => self.shuffle_context.as_ref(),
            ContextType::Autoplay => self.autoplay_context.as_ref(),
        }
        .ok_or(StateError::NoContext(*ty))
    }

    pub fn context_uri(&self) -> &String {
        &self.player.context_uri
    }

    pub fn reset_context(&mut self, new_context: Option<&str>) {
        self.active_context = ContextType::Default;
        self.fill_up_context = ContextType::Default;

        self.autoplay_context = None;
        self.shuffle_context = None;

        let reset_default_context = new_context.is_none()
            || matches!(new_context, Some(ctx) if self.player.context_uri != ctx);
        if reset_default_context {
            self.context = None;
            self.next_contexts.clear();
        } else if let Some(ctx) = self.context.as_mut() {
            ctx.index.track = 0;
            ctx.index.page = 0;
        }

        self.update_restrictions()
    }

    pub fn get_context_uri_from_context(context: &Context) -> Option<&String> {
        if !context.uri.starts_with(SEARCH_IDENTIFIER) {
            return Some(&context.uri);
        }

        context
            .pages
            .first()
            .and_then(|p| p.tracks.first().map(|t| &t.uri))
    }

    pub fn update_context(&mut self, context: Context, ty: UpdateContext) -> Result<(), Error> {
        if context.pages.iter().all(|p| p.tracks.is_empty()) {
            error!("context didn't have any tracks: {context:#?}");
            return Err(StateError::ContextHasNoTracks.into());
        } else if context.uri.starts_with(LOCAL_FILES_IDENTIFIER) {
            return Err(StateError::UnsupportedLocalPlayBack.into());
        }

        if matches!(ty, UpdateContext::Default) {
            self.next_contexts.clear();
        }

        let mut first_page = None;
        for page in context.pages {
            if first_page.is_none() && !page.tracks.is_empty() {
                first_page = Some(page);
            } else {
                self.next_contexts.push(page)
            }
        }

        let page = match first_page {
            None => Err(StateError::ContextHasNoTracks)?,
            Some(p) => p,
        };

        let prev_context = match ty {
            UpdateContext::Default => self.context.as_ref(),
            UpdateContext::Autoplay => self.autoplay_context.as_ref(),
        };

        debug!(
            "updated context {ty:?} from {} ({} tracks) to {} ({} tracks)",
            self.player.context_uri,
            prev_context
                .map(|c| c.tracks.len().to_string())
                .unwrap_or_else(|| "-".to_string()),
            context.uri,
            page.tracks.len()
        );

        if context.restrictions.is_some() {
            self.player.restrictions = context.restrictions.clone();
            self.player.context_restrictions = context.restrictions;
        } else {
            self.player.context_restrictions = Default::default();
            self.player.restrictions = Default::default()
        }

        self.player.context_metadata.clear();
        for (key, value) in context.metadata {
            self.player.context_metadata.insert(key, value);
        }

        match ty {
            UpdateContext::Default => {
                let mut new_context = self.state_context_from_page(page, Some(&context.uri), None);

                // when we update the same context, we should try to preserve the previous position
                // otherwise we might load the entire context twice
                if self.player.context_uri == context.uri {
                    match Self::find_index_in_context(Some(&new_context), |t| {
                        self.player.track.uri == t.uri
                    }) {
                        Ok(new_pos) => {
                            debug!("found new index of current track, updating new_context index to {new_pos}");
                            new_context.index.track = (new_pos + 1) as u32;
                        }
                        // the track isn't anymore in the context
                        Err(_) if matches!(self.active_context, ContextType::Default) => {
                            warn!("current track was removed, setting pos to last known index");
                            new_context.index.track = self.player.index.track
                        }
                        Err(_) => {}
                    }
                    // enforce reloading the context
                    self.clear_next_tracks(true);
                }

                self.context = Some(new_context);

                self.player.context_url = format!("context://{}", &context.uri);
                self.player.context_uri = context.uri;
            }
            UpdateContext::Autoplay => {
                self.autoplay_context = Some(self.state_context_from_page(
                    page,
                    Some(&context.uri),
                    Some(Provider::Autoplay),
                ))
            }
        }

        Ok(())
    }

    fn state_context_from_page(
        &mut self,
        page: ContextPage,
        new_context_uri: Option<&str>,
        provider: Option<Provider>,
    ) -> StateContext {
        let new_context_uri = new_context_uri.unwrap_or(&self.player.context_uri);

        let tracks = page
            .tracks
            .iter()
            .flat_map(|track| {
                match self.context_to_provided_track(track, Some(new_context_uri), provider.clone())
                {
                    Ok(t) => Some(t),
                    Err(why) => {
                        error!("couldn't convert {track:#?} into ProvidedTrack: {why}");
                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        StateContext {
            tracks,
            metadata: page.metadata,
            index: ContextIndex::new(),
        }
    }

    pub fn merge_context(&mut self, context: Option<Context>) -> Option<()> {
        let mut context = context?;
        if context.uri != self.player.context_uri {
            return None;
        }

        let current_context = self.context.as_mut()?;
        let new_page = context.pages.pop()?;

        for new_track in new_page.tracks {
            if new_track.uri.is_empty() {
                continue;
            }

            if let Ok(position) =
                Self::find_index_in_context(Some(current_context), |t| t.uri == new_track.uri)
            {
                let context_track = current_context.tracks.get_mut(position)?;

                for (key, value) in new_track.metadata {
                    warn!("merging metadata {key} {value}");
                    context_track.metadata.insert(key, value);
                }

                // the uid provided from another context might be actual uid of an item
                context_track.uid = new_track.uid;
            }
        }

        Some(())
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
        context_uri: Option<&str>,
        provider: Option<Provider>,
    ) -> Result<ProvidedTrack, Error> {
        let id = if !ctx_track.uri.is_empty() {
            if ctx_track.uri.contains(['?', '%']) {
                Err(StateError::InvalidTrackUri(ctx_track.uri.clone()))?
            }

            SpotifyId::from_uri(&ctx_track.uri)?
        } else if !ctx_track.gid.is_empty() {
            SpotifyId::from_raw(&ctx_track.gid)?
        } else {
            Err(StateError::InvalidTrackUri(String::new()))?
        };

        let provider = if self.unavailable_uri.contains(&ctx_track.uri) {
            Provider::Unavailable
        } else {
            provider.unwrap_or(Provider::Context)
        };

        // assumption: the uid is used as unique-id of any item
        //  - queue resorting is done by each client and orients itself by the given uid
        //  - if no uid is present, resorting doesn't work or behaves not as intended
        let uid = if ctx_track.uid.is_empty() {
            // so setting providing a unique id should allow to resort the queue
            Uuid::new_v4().as_simple().to_string()
        } else {
            ctx_track.uid.to_string()
        };

        let mut metadata = HashMap::new();
        for (k, v) in &ctx_track.metadata {
            metadata.insert(k.to_string(), v.to_string());
        }

        let mut track = ProvidedTrack {
            uri: id.to_uri()?.replace("unknown", "track"),
            uid,
            metadata,
            provider: provider.to_string(),
            ..Default::default()
        };

        if let Some(context_uri) = context_uri {
            track.set_context_uri(context_uri.to_string());
            track.set_entity_uri(context_uri.to_string());
        }

        if matches!(provider, Provider::Autoplay) {
            track.set_autoplay(true)
        }

        Ok(track)
    }

    pub fn fill_context_from_page(&mut self, page: ContextPage) -> Result<(), Error> {
        let context = self.state_context_from_page(page, None, None);
        let ctx = self
            .context
            .as_mut()
            .ok_or(StateError::NoContext(ContextType::Default))?;

        for t in context.tracks {
            ctx.tracks.push(t)
        }

        Ok(())
    }

    pub fn try_load_next_context(&mut self) -> Result<LoadNext, Error> {
        let next = match self.next_contexts.first() {
            None => return Ok(LoadNext::Empty),
            Some(_) => self.next_contexts.remove(0),
        };

        if next.tracks.is_empty() {
            self.update_current_index(|i| i.page += 1);
            return Ok(LoadNext::PageUrl(next.page_url));
        }

        self.fill_context_from_page(next)?;
        self.fill_up_next_tracks()?;

        Ok(LoadNext::Done)
    }
}
