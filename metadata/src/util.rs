macro_rules! from_repeated_message {
    ($src:ty, $dst:ty) => {
        impl From<&[$src]> for $dst {
            fn from(src: &[$src]) -> Self {
                let result = src.iter().map(From::from).collect();
                Self(result)
            }
        }
    };
}

pub(crate) use from_repeated_message;

macro_rules! from_repeated_enum {
    ($src:ty, $dst:ty) => {
        impl From<&[$src]> for $dst {
            fn from(src: &[$src]) -> Self {
                let result = src.iter().map(|x| <$src>::from(*x)).collect();
                Self(result)
            }
        }
    };
}

pub(crate) use from_repeated_enum;

macro_rules! try_from_repeated_message {
    ($src:ty, $dst:ty) => {
        impl TryFrom<&[$src]> for $dst {
            type Error = MetadataError;
            fn try_from(src: &[$src]) -> Result<Self, Self::Error> {
                let result: Result<Vec<_>, _> = src.iter().map(TryFrom::try_from).collect();
                Ok(Self(result?))
            }
        }
    };
}

pub(crate) use try_from_repeated_message;
