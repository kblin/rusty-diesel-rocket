use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MibigError {
    NotImplemented,
    Io(io::Error),
    InvalidTaxID(String),
}

macro_rules! implement_custom_error_from {
    ($f: ty, $e: expr) => {
        impl From<$f> for MibigError {
            fn from(f: $f) -> MibigError {
                $e(f)
            }
        }
    };
}

implement_custom_error_from!(io::Error, MibigError::Io);

impl fmt::Display for MibigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MibigError::Io(ref err) => write!(f, "IO error: {}", err),
            MibigError::NotImplemented => write!(f, "Not implemented"),
            MibigError::InvalidTaxID(ref err) => write!(f, "Invalid TaxID: {}", err),
        }
    }
}

impl error::Error for MibigError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            MibigError::Io(ref err) => Some(err),
            MibigError::NotImplemented | MibigError::InvalidTaxID(_) => None,
        }
    }
}
