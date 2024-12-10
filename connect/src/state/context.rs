use crate::state::{metadata::Metadata, provider::Provider, ConnectState, StateError};
use librespot_core::{Error, SpotifyId};
use librespot_protocol::player::{
    Context, ContextIndex, ContextPage, ContextTrack, ProvidedTrack, Restrictions,
};
use protobuf::MessageField;
use std::collections::HashMap;
use uuid::Uuid;

const LOCAL_FILES_IDENTIFIER: &str = "spotify:local-files";
const SEARCH_IDENTIFIER: &str = "spotify:search";

#[derive(Debug, Clone)]
pub struct StateContext {
    pub tracks: Vec<ProvidedTrack>,
    pub metadata: HashMap<String, String>,
    pub restrictions: Option<Restrictions>,
    /// is used to keep track which tracks are already loaded into the next_tracks
    pub index: ContextIndex,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum ContextType {
    #[default]
    Default,
    Shuffle,
    Autoplay,
}

#[derive(Debug)]
pub enum UpdateContext {
    Default,
    Autoplay,
}

pub enum ResetContext<'s> {
    Completely,
    DefaultIndex,
    WhenDifferent(&'s str),
}

/// Extracts the spotify uri from a given page_url
///
/// Just extracts "spotify/album/5LFzwirfFwBKXJQGfwmiMY" and replaces the slash's with colon's
///
/// Expected `page_url` should look something like the following:
/// `hm://artistplaycontext/v1/page/spotify/album/5LFzwirfFwBKXJQGfwmiMY/km_artist`
fn page_url_to_uri(page_url: &str) -> String {
    let split = if let Some(rest) = page_url.strip_prefix("hm://") {
        rest.split('/')
    } else {
        warn!("page_url didn't started with hm://. got page_url: {page_url}");
        page_url.split('/')
    };

    split
        .skip_while(|s| s != &"spotify")
        .take(3)
        .collect::<Vec<&str>>()
        .join(":")
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
        &self.player().context_uri
    }

    pub fn reset_context(&mut self, mut reset_as: ResetContext) {
        self.set_active_context(ContextType::Default);
        self.fill_up_context = ContextType::Default;

        if matches!(reset_as, ResetContext::WhenDifferent(ctx) if self.context_uri() != ctx) {
            reset_as = ResetContext::Completely
        }
        self.shuffle_context = None;

        match reset_as {
            ResetContext::Completely => {
                self.context = None;
                self.autoplay_context = None;
            }
            ResetContext::WhenDifferent(_) => debug!("context didn't change, no reset"),
            ResetContext::DefaultIndex => {
                for ctx in [self.context.as_mut(), self.autoplay_context.as_mut()]
                    .into_iter()
                    .flatten()
                {
                    ctx.index.track = 0;
                    ctx.index.page = 0;
                }
            }
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

    pub fn set_active_context(&mut self, new_context: ContextType) {
        self.active_context = new_context;

        let ctx = match self.get_context(&new_context) {
            Err(why) => {
                debug!("couldn't load context info because: {why}");
                return;
            }
            Ok(ctx) => ctx,
        };

        let mut restrictions = ctx.restrictions.clone();
        let metadata = ctx.metadata.clone();

        let player = self.player_mut();

        player.context_metadata.clear();
        player.restrictions.clear();

        if let Some(restrictions) = restrictions.take() {
            player.restrictions = MessageField::some(restrictions);
        }

        for (key, value) in metadata {
            player.context_metadata.insert(key, value);
        }
    }

    pub fn update_context(
        &mut self,
        mut context: Context,
        ty: UpdateContext,
    ) -> Result<Option<Vec<String>>, Error> {
        if context.pages.iter().all(|p| p.tracks.is_empty()) {
            error!("context didn't have any tracks: {context:#?}");
            return Err(StateError::ContextHasNoTracks.into());
        } else if context.uri.starts_with(LOCAL_FILES_IDENTIFIER) {
            return Err(StateError::UnsupportedLocalPlayBack.into());
        }

        let mut next_contexts = Vec::new();
        let mut first_page = None;
        for page in context.pages {
            if first_page.is_none() && !page.tracks.is_empty() {
                first_page = Some(page);
            } else {
                next_contexts.push(page)
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
            "updated context {ty:?} from <{}> ({} tracks) to <{}> ({} tracks)",
            self.context_uri(),
            prev_context
                .map(|c| c.tracks.len().to_string())
                .unwrap_or_else(|| "-".to_string()),
            context.uri,
            page.tracks.len()
        );

        match ty {
            UpdateContext::Default => {
                let mut new_context = self.state_context_from_page(
                    page,
                    context.restrictions.take(),
                    Some(&context.uri),
                    None,
                );

                // when we update the same context, we should try to preserve the previous position
                // otherwise we might load the entire context twice
                if !self.context_uri().contains(SEARCH_IDENTIFIER)
                    && self.context_uri() == &context.uri
                {
                    match Self::find_index_in_context(Some(&new_context), |t| {
                        self.current_track(|t| &t.uri) == &t.uri
                    }) {
                        Ok(new_pos) => {
                            debug!("found new index of current track, updating new_context index to {new_pos}");
                            new_context.index.track = (new_pos + 1) as u32;
                        }
                        // the track isn't anymore in the context
                        Err(_) if matches!(self.active_context, ContextType::Default) => {
                            warn!("current track was removed, setting pos to last known index");
                            new_context.index.track = self.player().index.track
                        }
                        Err(_) => {}
                    }
                    // enforce reloading the context
                    self.clear_next_tracks(true);
                }

                self.context = Some(new_context);

                if !context.url.contains(SEARCH_IDENTIFIER) {
                    self.player_mut().context_url = context.url;
                } else {
                    self.player_mut().context_url.clear()
                }
                self.player_mut().context_uri = context.uri;
            }
            UpdateContext::Autoplay => {
                self.autoplay_context = Some(self.state_context_from_page(
                    page,
                    context.restrictions.take(),
                    Some(&context.uri),
                    Some(Provider::Autoplay),
                ))
            }
        }

        if next_contexts.is_empty() {
            return Ok(None);
        }

        // load remaining contexts
        let next_contexts = next_contexts
            .into_iter()
            .flat_map(|page| {
                if !page.tracks.is_empty() {
                    self.fill_context_from_page(page).ok()?;
                    None
                } else if !page.page_url.is_empty() {
                    Some(page_url_to_uri(&page.page_url))
                } else {
                    warn!("unhandled context page: {page:#?}");
                    None
                }
            })
            .collect();

        Ok(Some(next_contexts))
    }

    fn state_context_from_page(
        &mut self,
        page: ContextPage,
        restrictions: Option<Restrictions>,
        new_context_uri: Option<&str>,
        provider: Option<Provider>,
    ) -> StateContext {
        let new_context_uri = new_context_uri.unwrap_or(self.context_uri());

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
            restrictions,
            metadata: page.metadata,
            index: ContextIndex::new(),
        }
    }

    pub fn merge_context(&mut self, context: Option<Context>) -> Option<()> {
        let mut context = context?;
        if self.context_uri() != &context.uri {
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
                if !new_track.uid.is_empty() {
                    context_track.uid = new_track.uid;
                }
            }
        }

        Some(())
    }

    pub(super) fn update_context_index(
        &mut self,
        ty: ContextType,
        new_index: usize,
    ) -> Result<(), StateError> {
        let context = match ty {
            ContextType::Default => self.context.as_mut(),
            ContextType::Shuffle => self.shuffle_context.as_mut(),
            ContextType::Autoplay => self.autoplay_context.as_mut(),
        }
        .ok_or(StateError::NoContext(ty))?;

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
        let context = self.state_context_from_page(page, None, None, None);
        let ctx = self
            .context
            .as_mut()
            .ok_or(StateError::NoContext(ContextType::Default))?;

        for t in context.tracks {
            ctx.tracks.push(t)
        }

        Ok(())
    }
}
