macro_rules! impl_from_repeated {
    ($src:ty, $dst:ty) => {
        impl From<&[$src]> for $dst {
            fn from(src: &[$src]) -> Self {
                let result = src.iter().map(From::from).collect();
                Self(result)
            }
        }
    };
}

pub(crate) use impl_from_repeated;

macro_rules! impl_from_repeated_copy {
    ($src:ty, $dst:ty) => {
        impl From<&[$src]> for $dst {
            fn from(src: &[$src]) -> Self {
                let result = src.iter().copied().collect();
                Self(result)
            }
        }
    };
}

pub(crate) use impl_from_repeated_copy;

macro_rules! impl_try_from_repeated {
    ($src:ty, $dst:ty) => {
        impl TryFrom<&[$src]> for $dst {
            type Error = librespot_core::Error;
            fn try_from(src: &[$src]) -> Result<Self, Self::Error> {
                let result: Result<Vec<_>, _> = src.iter().map(TryFrom::try_from).collect();
                Ok(Self(result?))
            }
        }
    };
}

pub(crate) use impl_try_from_repeated;

macro_rules! impl_deref_wrapped {
    ($wrapper:ty, $inner:ty) => {
        impl Deref for $wrapper {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

pub(crate) use impl_deref_wrapped;
