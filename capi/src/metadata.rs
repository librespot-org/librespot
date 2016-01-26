use eventual::Async;
use owning_ref::MutexGuardRef;
use std::sync::{Mutex, Arc};

use librespot::metadata::{MetadataTrait, MetadataRef};

pub enum SpMetadataInner<T: MetadataTrait> {
    Loading,
    Error,
    Loaded(T),
}

pub struct SpMetadata<T: MetadataTrait>(Arc<Mutex<SpMetadataInner<T>>>);

impl <T: MetadataTrait> SpMetadata<T> {
    pub fn from_future(future: MetadataRef<T>) -> SpMetadata<T> {
        let metadata = Arc::new(Mutex::new(SpMetadataInner::Loading));

        {
            let metadata = metadata.clone();
            future.receive(move |result| {
                //let metadata = metadata.upgrade().unwrap();
                let mut metadata = metadata.lock().unwrap();

                *metadata = match result {
                    Ok(data) =>  SpMetadataInner::Loaded(data),
                    Err(_) => SpMetadataInner::Error,
                };
            });
        }

        SpMetadata(metadata)
    }

    pub fn is_loaded(&self) -> bool {
        self.get().is_some()
    }

    pub fn get(&self) -> Option<MutexGuardRef<SpMetadataInner<T>, T>> {
        let inner = self.0.lock().unwrap();

        match *inner {
            SpMetadataInner::Loaded(_) => {
                Some(MutexGuardRef::new(inner).map(|inner| {
                    match *inner {
                        SpMetadataInner::Loaded(ref metadata) => metadata,
                        _ => unreachable!(),
                    }
                }))
            }
            _ => None,
        }
    }
}
