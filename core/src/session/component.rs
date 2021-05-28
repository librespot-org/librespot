pub(super) trait LockComponentInner<R> {
    fn lock<T>(&self, f: impl FnOnce(&mut R) -> T) -> T;
}

macro_rules! impl_components {
    ( $s:ty; $($t:ty : .$field:ident) , * ) => {
        $(
        impl $crate::session::component::LockComponentInner<$t> for $s {
            fn lock<T>(&self, f: impl FnOnce(&mut $t) -> T) -> T {
                let mut inner = self.$field.lock().expect("Mutex poisoned");
                f(&mut inner)
            }
        }) *
    };
}

macro_rules! component {
    ($name:ident<'_> : $inner:ident { $($key:ident : $ty:ty = $value:expr,)* }) => {
        #[derive(Clone)]
        pub struct $name<'a>(pub(in $crate::session) &'a $crate::session::SessionInternal);

        impl<'a> $name<'a> {
            #[allow(dead_code)]
            fn lock<F: FnOnce(&mut $inner) -> R, R>(&self, f: F) -> R {
                $crate::session::component::LockComponentInner::lock(self.0, f)
            }

            #[allow(dead_code)]
            fn send_packet(&self, cmd: u8, data: Vec<u8>) {
                self.0.send_packet(cmd, data);
            }
        }

        pub(in $crate::session) struct $inner {
            $($key : $ty,)*
        }

        impl Default for $inner {
            fn default() -> Self {
                Self {
                    $($key : $value,)*
                }
            }
        }
    }
}
