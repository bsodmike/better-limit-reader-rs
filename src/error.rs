use crate::{error_from, LimitReaderOutputBuilderError};
use std::{
    error::Error as StdError,
    fmt::{self},
    string::FromUtf8Error,
};

/// Boxed error, a ptr to the Error via dynamic dispatch allocated on the heap at run time.
pub type BoxError = Box<dyn StdError + Send + Sync>;

/// Default error type for create.
pub type LimitReaderError = Error;

/// Error type
pub struct Error {
    kind: ErrorKind,
    error: BoxError,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    IoError,
    Utf8Error,
    LimitReaderOutputBuilderError,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        // tidy-alphabetical-start
        match *self {
            IoError => "io error",
            Utf8Error => "invalid utf-8",
            LimitReaderOutputBuilderError => "builder error",
        }
    }
}

impl fmt::Display for ErrorKind {
    /// Shows a human-readable description of the `ErrorKind`.
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

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
