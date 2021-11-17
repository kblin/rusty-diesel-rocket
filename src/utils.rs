pub mod db;
pub mod num_as_string;
pub mod taxa;

pub use crate::errors::MibigError;

pub fn generate_uid(length: i32) -> Result<String, MibigError> {
    Err(MibigError::NotImplemented)
}

pub fn generate_password(password: String) -> Result<String, MibigError> {
    Err(MibigError::NotImplemented)
}
