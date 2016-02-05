use eventual::Async;
use std::sync::Arc;
use std::ffi::CStr;
use std::cell::UnsafeCell;

use librespot::metadata::{MetadataTrait, MetadataRef};

use cstring_cache::CStringCache;

use session::SpSession;

pub struct UnsafeSyncCell<T> {
    cell: UnsafeCell<T>
}

impl <T> UnsafeSyncCell<T> {
    fn new(value: T) -> UnsafeSyncCell<T> {
        UnsafeSyncCell { cell: UnsafeCell::new(value) }
    }

    fn get(&self) -> *mut T {
        self.cell.get()
    }
}

unsafe impl<T> Sync for UnsafeSyncCell<T> {}

pub enum SpMetadataState<T: MetadataTrait> {
    Loading,
    Error,
    Loaded(T),
}

pub struct SpMetadata<T: MetadataTrait> {
    state: Arc<UnsafeSyncCell<SpMetadataState<T>>>,
    cache: CStringCache,
}

impl <T: MetadataTrait> SpMetadata<T> {
    pub fn from_future(future: MetadataRef<T>) -> SpMetadata<T> {
        let state = Arc::new(UnsafeSyncCell::new(SpMetadataState::Loading));

        {
            let state = state.clone();
            SpSession::receive(future, move |session, result| {
                let state = unsafe {
                    &mut *state.get()
                };

                *state = match result {
                    Ok(data) => SpMetadataState::Loaded(data),
                    Err(_) => SpMetadataState::Error,
                };

                unsafe {
                    if let Some(f) = session.callbacks.metadata_updated {
                        f(session as *mut _)
                    }
                }
            });
        }

        SpMetadata {
            state: state,
            cache: CStringCache::new(),
        }
    }

    pub fn is_loaded(&self) -> bool {
        unsafe {
            self.get().is_some()
        }
    }

    pub unsafe fn get(&self) -> Option<&'static T> {
        let state = &*self.state.get();

        match *state {
            SpMetadataState::Loaded(ref metadata) => Some(metadata),
            _ => None,
        }
    }

    pub fn intern(&mut self, string: &str) -> &CStr {
        self.cache.intern(string)
    }
}
