use bcrypt;
use diesel;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MibigError {
    #[allow(dead_code)]
    NotImplemented,
    Io(io::Error),
    InvalidTaxID(String),
    Password(bcrypt::BcryptError),
    DatabaseError(diesel::result::Error),
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
implement_custom_error_from!(bcrypt::BcryptError, MibigError::Password);
implement_custom_error_from!(diesel::result::Error, MibigError::DatabaseError);

impl fmt::Display for MibigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MibigError::Io(ref err) => write!(f, "IO error: {}", err),
            MibigError::NotImplemented => write!(f, "Not implemented"),
            MibigError::InvalidTaxID(ref err) => write!(f, "Invalid TaxID: {}", err),
            MibigError::Password(ref err) => write!(f, "Password error: {}", err),
            MibigError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
        }
    }
}

impl error::Error for MibigError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            MibigError::Io(ref err) => Some(err),
            MibigError::Password(ref err) => Some(err),
            MibigError::DatabaseError(ref err) => Some(err),
            MibigError::NotImplemented | MibigError::InvalidTaxID(_) => None,
        }
    }
}
