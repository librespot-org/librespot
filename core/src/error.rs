use std::{
    error, fmt,
    num::{ParseIntError, TryFromIntError},
    str::Utf8Error,
    string::FromUtf8Error,
};

use base64::DecodeError;
use http::{
    header::{InvalidHeaderName, InvalidHeaderValue, ToStrError},
    method::InvalidMethod,
    status::InvalidStatusCode,
    uri::{InvalidUri, InvalidUriParts},
};
use protobuf::Error as ProtobufError;
use thiserror::Error;
use tokio::sync::{
    mpsc::error::SendError, oneshot::error::RecvError, AcquireError, TryAcquireError,
};
use url::ParseError;

use librespot_oauth::OAuthError;

#[cfg(feature = "with-dns-sd")]
use dns_sd::DNSError;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub error: Box<dyn error::Error + Send + Sync>,
}

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    #[error("The operation was cancelled by the caller")]
    Cancelled = 1,

    #[error("Unknown error")]
    Unknown = 2,

    #[error("Client specified an invalid argument")]
    InvalidArgument = 3,

    #[error("Deadline expired before operation could complete")]
    DeadlineExceeded = 4,

    #[error("Requested entity was not found")]
    NotFound = 5,

    #[error("Attempt to create entity that already exists")]
    AlreadyExists = 6,

    #[error("Permission denied")]
    PermissionDenied = 7,

    #[error("No valid authentication credentials")]
    Unauthenticated = 16,

    #[error("Resource has been exhausted")]
    ResourceExhausted = 8,

    #[error("Invalid state")]
    FailedPrecondition = 9,

    #[error("Operation aborted")]
    Aborted = 10,

    #[error("Operation attempted past the valid range")]
    OutOfRange = 11,

    #[error("Not implemented")]
    Unimplemented = 12,

    #[error("Internal error")]
    Internal = 13,

    #[error("Service unavailable")]
    Unavailable = 14,

    #[error("Unrecoverable data loss or corruption")]
    DataLoss = 15,

    #[error("Operation must not be used")]
    DoNotUse = -1,
}

#[derive(Debug, Error)]
struct ErrorMessage(String);

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind,
            error: error.into(),
        }
    }

    pub fn aborted<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Aborted,
            error: error.into(),
        }
    }

    pub fn already_exists<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::AlreadyExists,
            error: error.into(),
        }
    }

    pub fn cancelled<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Cancelled,
            error: error.into(),
        }
    }

    pub fn data_loss<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::DataLoss,
            error: error.into(),
        }
    }

    pub fn deadline_exceeded<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::DeadlineExceeded,
            error: error.into(),
        }
    }

    pub fn do_not_use<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::DoNotUse,
            error: error.into(),
        }
    }

    pub fn failed_precondition<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::FailedPrecondition,
            error: error.into(),
        }
    }

    pub fn internal<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Internal,
            error: error.into(),
        }
    }

    pub fn invalid_argument<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::InvalidArgument,
            error: error.into(),
        }
    }

    pub fn not_found<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::NotFound,
            error: error.into(),
        }
    }

    pub fn out_of_range<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::OutOfRange,
            error: error.into(),
        }
    }

    pub fn permission_denied<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::PermissionDenied,
            error: error.into(),
        }
    }

    pub fn resource_exhausted<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::ResourceExhausted,
            error: error.into(),
        }
    }

    pub fn unauthenticated<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Unauthenticated,
            error: error.into(),
        }
    }

    pub fn unavailable<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Unavailable,
            error: error.into(),
        }
    }

    pub fn unimplemented<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Unimplemented,
            error: error.into(),
        }
    }

    pub fn unknown<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Unknown,
            error: error.into(),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.source()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{} {{ ", self.kind)?;
        self.error.fmt(fmt)?;
        write!(fmt, " }}")
    }
}

impl From<OAuthError> for Error {
    fn from(err: OAuthError) -> Self {
        use OAuthError::*;
        match err {
            AuthCodeBadUri { .. }
            | AuthCodeNotFound { .. }
            | AuthCodeListenerRead
            | AuthCodeListenerParse => Error::unavailable(err),
            AuthCodeStdinRead
            | AuthCodeListenerBind { .. }
            | AuthCodeListenerTerminated
            | AuthCodeListenerWrite
            | Recv
            | ExchangeCode { .. } => Error::internal(err),
            _ => Error::failed_precondition(err),
        }
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

#[cfg(feature = "with-dns-sd")]
impl From<DNSError> for Error {
    fn from(err: DNSError) -> Self {
        Self::new(ErrorKind::Unavailable, err)
    }
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Self {
        if err.is::<InvalidHeaderName>()
            || err.is::<InvalidHeaderValue>()
            || err.is::<InvalidMethod>()
            || err.is::<InvalidUri>()
            || err.is::<InvalidUriParts>()
        {
            return Self::new(ErrorKind::InvalidArgument, err);
        }

        if err.is::<InvalidStatusCode>() {
            return Self::new(ErrorKind::FailedPrecondition, err);
        }

        Self::new(ErrorKind::Unknown, err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        if err.is_parse() || err.is_parse_status() || err.is_user() {
            return Self::new(ErrorKind::Internal, err);
        }

        if err.is_canceled() {
            return Self::new(ErrorKind::Cancelled, err);
        }

        if err.is_incomplete_message() {
            return Self::new(ErrorKind::DataLoss, err);
        }

        if err.is_body_write_aborted() || err.is_closed() {
            return Self::new(ErrorKind::Aborted, err);
        }

        if err.is_timeout() {
            return Self::new(ErrorKind::DeadlineExceeded, err);
        }

        Self::new(ErrorKind::Unknown, err)
    }
}

impl From<hyper_util::client::legacy::Error> for Error {
    fn from(err: hyper_util::client::legacy::Error) -> Self {
        if err.is_connect() {
            return Self::new(ErrorKind::Unavailable, err);
        }

        Self::new(ErrorKind::Unknown, err)
    }
}

impl From<time::error::Parse> for Error {
    fn from(err: time::error::Parse) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind as IoErrorKind;
        match err.kind() {
            IoErrorKind::NotFound => Self::new(ErrorKind::NotFound, err),
            IoErrorKind::PermissionDenied => Self::new(ErrorKind::PermissionDenied, err),
            IoErrorKind::AddrInUse | IoErrorKind::AlreadyExists => {
                Self::new(ErrorKind::AlreadyExists, err)
            }
            IoErrorKind::AddrNotAvailable
            | IoErrorKind::ConnectionRefused
            | IoErrorKind::NotConnected => Self::new(ErrorKind::Unavailable, err),
            IoErrorKind::BrokenPipe
            | IoErrorKind::ConnectionReset
            | IoErrorKind::ConnectionAborted => Self::new(ErrorKind::Aborted, err),
            IoErrorKind::Interrupted | IoErrorKind::WouldBlock => {
                Self::new(ErrorKind::Cancelled, err)
            }
            IoErrorKind::InvalidData | IoErrorKind::UnexpectedEof => {
                Self::new(ErrorKind::FailedPrecondition, err)
            }
            IoErrorKind::TimedOut => Self::new(ErrorKind::DeadlineExceeded, err),
            IoErrorKind::InvalidInput => Self::new(ErrorKind::InvalidArgument, err),
            IoErrorKind::WriteZero => Self::new(ErrorKind::ResourceExhausted, err),
            _ => Self::new(ErrorKind::Unknown, err),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Self {
        Self::new(ErrorKind::InvalidArgument, err)
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Self {
        Self::new(ErrorKind::InvalidArgument, err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<TryFromIntError> for Error {
    fn from(err: TryFromIntError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<ProtobufError> for Error {
    fn from(err: ProtobufError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<RecvError> for Error {
    fn from(err: RecvError) -> Self {
        Self::new(ErrorKind::Internal, err)
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(err: SendError<T>) -> Self {
        Self {
            kind: ErrorKind::Internal,
            error: ErrorMessage(err.to_string()).into(),
        }
    }
}

impl From<AcquireError> for Error {
    fn from(err: AcquireError) -> Self {
        Self {
            kind: ErrorKind::ResourceExhausted,
            error: ErrorMessage(err.to_string()).into(),
        }
    }
}

impl From<TryAcquireError> for Error {
    fn from(err: TryAcquireError) -> Self {
        Self {
            kind: ErrorKind::ResourceExhausted,
            error: ErrorMessage(err.to_string()).into(),
        }
    }
}

impl From<ToStrError> for Error {
    fn from(err: ToStrError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}
