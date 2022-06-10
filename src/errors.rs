use bcrypt;
use diesel;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response, Result};
use rocket::Request;
use std::error;
use std::fmt;
use std::io;
use std::io::Cursor;

#[derive(Debug)]
pub enum MibigError {
    NotImplemented,
    Io(io::Error),
    InvalidTaxID(String),
    Password(bcrypt::BcryptError),
    DatabaseError(diesel::result::Error),
    Unauthorised,
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
            MibigError::Unauthorised => write!(f, "Unauthorised"),
        }
    }
}

impl error::Error for MibigError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            MibigError::Io(ref err) => Some(err),
            MibigError::Password(ref err) => Some(err),
            MibigError::DatabaseError(ref err) => Some(err),
            MibigError::NotImplemented | MibigError::InvalidTaxID(_) | MibigError::Unauthorised => {
                None
            }
        }
    }
}

impl<'r> Responder<'r, 'static> for MibigError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        match self {
            MibigError::DatabaseError(_)
            | MibigError::InvalidTaxID(_)
            | MibigError::Io(_)
            | MibigError::NotImplemented => {
                let body = format!("Server error: {}", self);

                let res = Response::build()
                    .status(Status::InternalServerError)
                    .header(ContentType::Plain)
                    .sized_body(body.len(), Cursor::new(body))
                    .finalize();
                return Ok(res);
            }
            MibigError::Password(_) | MibigError::Unauthorised => {
                let body = format!("{}", self);

                let res = Response::build()
                    .status(Status::Unauthorized)
                    .header(ContentType::Plain)
                    .sized_body(body.len(), Cursor::new(body))
                    .finalize();
                return Ok(res);
            }
        }
    }
}
