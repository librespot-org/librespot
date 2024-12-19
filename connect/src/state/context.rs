use crate::{
    core::{Error, SpotifyId},
    protocol::player::{
        Context, ContextIndex, ContextPage, ContextTrack, ProvidedTrack, Restrictions,
    },
    state::{
        metadata::Metadata,
        provider::{IsProvider, Provider},
        ConnectState, StateError, SPOTIFY_MAX_NEXT_TRACKS_SIZE,
    },
};
use protobuf::MessageField;
use std::collections::HashMap;
use std::ops::Deref;
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

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum ContextType {
    #[default]
    Default,
    Shuffle,
    Autoplay,
}

#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub enum UpdateContext {
    Default,
    Autoplay,
}

impl Deref for UpdateContext {
    type Target = ContextType;

    fn deref(&self) -> &Self::Target {
        match self {
            UpdateContext::Default => &ContextType::Default,
            UpdateContext::Autoplay => &ContextType::Autoplay,
        }
    }
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
        warn!("page_url didn't start with hm://. got page_url: {page_url}");
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
        ctx: &StateContext,
        f: F,
    ) -> Result<usize, StateError> {
        ctx.tracks
            .iter()
            .position(f)
            .ok_or(StateError::CanNotFindTrackInContext(None, ctx.tracks.len()))
    }

    pub fn get_context(&self, ty: ContextType) -> Result<&StateContext, StateError> {
        match ty {
            ContextType::Default => self.context.as_ref(),
            ContextType::Shuffle => self.shuffle_context.as_ref(),
            ContextType::Autoplay => self.autoplay_context.as_ref(),
        }
        .ok_or(StateError::NoContext(ty))
    }

    pub fn context_uri(&self) -> &String {
        &self.player().context_uri
    }

    fn different_context_uri(&self, uri: &str) -> bool {
        // search identifier is always different
        self.context_uri() != uri || uri.starts_with(SEARCH_IDENTIFIER)
    }

    pub fn reset_context(&mut self, mut reset_as: ResetContext) {
        if matches!(reset_as, ResetContext::WhenDifferent(ctx) if self.different_context_uri(ctx)) {
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

        self.fill_up_context = ContextType::Default;
        self.set_active_context(ContextType::Default);
        self.update_restrictions()
    }

    pub fn valid_resolve_uri(uri: &str) -> Option<&str> {
        (!uri.starts_with(SEARCH_IDENTIFIER)).then_some(uri)
    }

    pub fn get_context_uri_from_context(context: &Context) -> Option<&str> {
        match Self::valid_resolve_uri(&context.uri) {
            Some(uri) => Some(uri),
            None => context
                .pages
                .first()
                .and_then(|p| p.tracks.first().map(|t| t.uri.as_ref())),
        }
    }

    pub fn set_active_context(&mut self, new_context: ContextType) {
        self.active_context = new_context;

        let player = self.player_mut();
        player.context_metadata.clear();
        player.restrictions.clear();

        let ctx = match self.get_context(new_context) {
            Err(why) => {
                warn!("couldn't load context info because: {why}");
                return;
            }
            Ok(ctx) => ctx,
        };

        let mut restrictions = ctx.restrictions.clone();
        let metadata = ctx.metadata.clone();

        let player = self.player_mut();

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
            Err(StateError::ContextHasNoTracks)?;
        } else if context.uri.starts_with(LOCAL_FILES_IDENTIFIER) {
            Err(StateError::UnsupportedLocalPlayBack)?;
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

        let mut page = match first_page {
            None => Err(StateError::ContextHasNoTracks)?,
            Some(p) => p,
        };

        debug!(
            "updated context {ty:?} to <{}> ({} tracks)",
            context.uri,
            page.tracks.len()
        );

        match ty {
            UpdateContext::Default => {
                let mut new_context = self.state_context_from_page(
                    page,
                    context.metadata,
                    context.restrictions.take(),
                    Some(&context.uri),
                    None,
                );

                // when we update the same context, we should try to preserve the previous position
                // otherwise we might load the entire context twice, unless it's the search context
                if !self.context_uri().starts_with(SEARCH_IDENTIFIER)
                    && self.context_uri() == &context.uri
                {
                    if let Some(new_index) = self.find_last_index_in_new_context(&new_context) {
                        new_context.index.track = match new_index {
                            Ok(i) => i,
                            Err(i) => {
                                self.player_mut().index = MessageField::none();
                                i
                            }
                        };

                        // enforce reloading the context
                        if let Some(autoplay_ctx) = self.autoplay_context.as_mut() {
                            autoplay_ctx.index.track = 0
                        }
                        self.clear_next_tracks();
                    }
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
                if matches!(self.context.as_ref(), Some(ctx) if ctx.tracks.len() == 1) {
                    if let Some(position) = page
                        .tracks
                        .iter()
                        .position(|p| self.current_track(|t| t.uri == p.uri))
                    {
                        debug!("removing track (of single track context) from autoplay context");
                        page.tracks.remove(position);
                    }
                }

                self.autoplay_context = Some(self.state_context_from_page(
                    page,
                    context.metadata,
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

    fn find_first_prev_track_index(&self, ctx: &StateContext) -> Option<usize> {
        let prev_tracks = self.prev_tracks();
        for i in (0..prev_tracks.len()).rev() {
            let prev_track = prev_tracks.get(i)?;
            if let Ok(idx) = Self::find_index_in_context(ctx, |t| prev_track.uri == t.uri) {
                return Some(idx);
            }
        }
        None
    }

    fn find_last_index_in_new_context(
        &self,
        new_context: &StateContext,
    ) -> Option<Result<u32, u32>> {
        let ctx = self.context.as_ref()?;

        let is_queued_item = self.current_track(|t| t.is_queue() || t.is_from_queue());

        let new_index = if ctx.index.track as usize >= SPOTIFY_MAX_NEXT_TRACKS_SIZE {
            Some(ctx.index.track as usize - SPOTIFY_MAX_NEXT_TRACKS_SIZE)
        } else if is_queued_item {
            self.find_first_prev_track_index(new_context)
        } else {
            Self::find_index_in_context(new_context, |current| {
                self.current_track(|t| t.uri == current.uri)
            })
            .ok()
        }
        .map(|i| i as u32 + 1);

        Some(new_index.ok_or_else(|| {
            info!(
                "couldn't distinguish index from current or previous tracks in the updated context"
            );
            let fallback_index = self
                .player()
                .index
                .as_ref()
                .map(|i| i.track)
                .unwrap_or_default();
            info!("falling back to index {fallback_index}");
            fallback_index
        }))
    }

    fn state_context_from_page(
        &mut self,
        page: ContextPage,
        metadata: HashMap<String, String>,
        restrictions: Option<Restrictions>,
        new_context_uri: Option<&str>,
        provider: Option<Provider>,
    ) -> StateContext {
        let new_context_uri = new_context_uri.unwrap_or(self.context_uri());

        let tracks = page
            .tracks
            .iter()
            .flat_map(|track| {
                match self.context_to_provided_track(
                    track,
                    Some(new_context_uri),
                    Some(&page.metadata),
                    provider.clone(),
                ) {
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
            metadata,
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
                Self::find_index_in_context(current_context, |t| t.uri == new_track.uri)
            {
                let context_track = current_context.tracks.get_mut(position)?;

                for (key, value) in new_track.metadata {
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
        page_metadata: Option<&HashMap<String, String>>,
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

        let mut metadata = page_metadata.cloned().unwrap_or_default();
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
        let context = self.state_context_from_page(page, HashMap::new(), None, None, None);
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
