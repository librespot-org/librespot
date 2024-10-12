macro_rules! component {
    ($name:ident : $inner:ident { $($key:ident : $ty:ty = $value:expr,)* }) => {
        #[derive(Clone)]
        pub struct $name(::std::sync::Arc<($crate::session::SessionWeak, ::parking_lot::Mutex<$inner>, ::tokio::sync::Semaphore)>);
        impl $name {
            #[allow(dead_code)]
            pub(crate) fn new(session: $crate::session::SessionWeak) -> $name {
                debug!(target:"librespot::component", "new {}", stringify!($name));

                $name(::std::sync::Arc::new((session, ::parking_lot::Mutex::new($inner {
                    $($key : $value,)*
                }), ::tokio::sync::Semaphore::new(1))))
            }

            #[allow(dead_code)]
            fn lock<F: FnOnce(&mut $inner) -> R, R>(&self, f: F) -> R {
                let mut inner = (self.0).1.lock();
                f(&mut inner)
            }

            /// See [::tokio::sync::Semaphore] for further infos.
            ///
            /// The returned permit has to be hold in scope. `let _ = ...?;` will drop the permit on
            /// the spot. Instead use `let _lock = ...?;` to hold the permit in scope without using it.
            #[allow(dead_code)]
            async fn unique_lock(&self) -> Result<::tokio::sync::SemaphorePermit<'_>, $crate::error::Error> {
                (self.0).2.acquire().await.map_err(Into::into)
            }

            #[allow(dead_code)]
            fn session(&self) -> $crate::session::Session {
                (self.0).0.upgrade()
            }
        }

        struct $inner {
            $($key : $ty,)*
        }

        impl Drop for $inner {
            fn drop(&mut self) {
                debug!(target:"librespot::component", "drop {}", stringify!($name));
            }
        }
    }
}
