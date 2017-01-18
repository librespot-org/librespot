macro_rules! component {
    ($name:ident : $inner:ident { $($key:ident : $ty:ty = $value:expr,)* }) => {
        #[derive(Clone)]
        pub struct $name(::std::sync::Arc<($crate::session::SessionWeak, ::std::sync::Mutex<$inner>)>);
        impl $name {
            #[allow(dead_code)]
            pub fn new(session: $crate::session::SessionWeak) -> $name {
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
    }
}
