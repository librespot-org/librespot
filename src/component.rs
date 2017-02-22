macro_rules! component {
    ($name:ident : $inner:ident { $($key:ident : $ty:ty = $value:expr,)* }) => {
        #[derive(Clone)]
        pub struct $name(::std::sync::Arc<($crate::session::SessionWeak, ::std::sync::Mutex<$inner>)>);
        impl $name {
            #[allow(dead_code)]
            pub fn new(session: $crate::session::SessionWeak) -> $name {
                debug!(target:"librespot::component", "new {}", stringify!($name));

                $name(::std::sync::Arc::new((session, ::std::sync::Mutex::new($inner {
                    $($key : $value,)*
                }))))
            }

            #[allow(dead_code)]
            fn lock<F: FnOnce(&mut $inner) -> R, R>(&self, f: F) -> R {
                let mut inner = (self.0).1.lock().expect("Mutex poisoned");
                f(&mut inner)
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

use std::sync::Mutex;
use std::cell::UnsafeCell;

pub struct Lazy<T>(Mutex<bool>, UnsafeCell<Option<T>>);
unsafe impl <T: Sync> Sync for Lazy<T> {}
unsafe impl <T: Send> Send for Lazy<T> {}

#[cfg_attr(feature = "cargo-clippy", allow(mutex_atomic))]
impl <T> Lazy<T> {
    pub fn new() -> Lazy<T> {
        Lazy(Mutex::new(false), UnsafeCell::new(None))
    }

    pub fn get<F: FnOnce() -> T>(&self, f: F) -> &T {
        let mut inner = self.0.lock().unwrap();
        if !*inner {
            unsafe {
                *self.1.get() = Some(f());
            }
            *inner = true;
        }

        unsafe { &*self.1.get() }.as_ref().unwrap()
    }
}
