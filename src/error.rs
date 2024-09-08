use crate::{error_from, LimitReaderOutputBuilderError};
use std::{
    error::Error as StdError,
    fmt::{self},
    num::TryFromIntError,
    string::FromUtf8Error,
};

/// Boxed error, a ptr to the Error via dynamic dispatch allocated on the heap at run time.
#[allow(clippy::module_name_repetitions)]
pub type BoxError = Box<dyn StdError + Send + Sync>;

/// Error type
pub struct Error {
    kind: ErrorKind,
    error: BoxError,
}

#[derive(Debug)]
#[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::enum_variant_names)]
pub enum ErrorKind {
    ReadError,
    IoError,
    LimitReaderOutputBuilderError,
    Utf8Error,
    TryFromIntError,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        #[allow(clippy::enum_glob_use)]
        use ErrorKind::*;
        // tidy-alphabetical-start
        match *self {
            ReadError => "read error",
            IoError => "io error",
            Utf8Error => "invalid utf-8",
            LimitReaderOutputBuilderError => "builder error",
            TryFromIntError => "conversion error",
        }
    }
}

impl fmt::Display for ErrorKind {
    /// Shows a human-readable description of the `ErrorKind`.
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.error)
    }
}

#[allow(dead_code)]
impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
    where
        E: Into<BoxError>,
    {
        Self::_new(kind, error.into())
    }

    fn _new(kind: ErrorKind, error: BoxError) -> Error {
        Error { kind, error }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_tuple("better-limit-reader::Error");
        f.field(&self.kind);
        f.field(&self.error);
        f.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", &self.error)
    }
}

error_from!(std::io::Error, ErrorKind::IoError);
error_from!(FromUtf8Error, ErrorKind::Utf8Error);
error_from!(
    LimitReaderOutputBuilderError,
    ErrorKind::LimitReaderOutputBuilderError
);
error_from!(TryFromIntError, ErrorKind::TryFromIntError);

#[macro_use]
pub mod macros {
    /// Implement provided Error type with a suitable `ErrorKind`
    #[macro_export]
    macro_rules! error_from {
        ($typ:ty, $kind:expr) => {
            impl From<$typ> for Error {
                fn from(error: $typ) -> Self {
                    Self {
                        error: error.into(),
                        kind: $kind,
                    }
                }
            }
        };
    }
}

/// Default error for [`crate::prelude::LimitReader`]
#[allow(clippy::module_name_repetitions)]
pub type LimitReaderError = Error;
