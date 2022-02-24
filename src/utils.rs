pub mod db;
pub mod num_as_string;
pub mod taxa;
pub mod typedefs;

use bcrypt::{hash, DEFAULT_COST};
use data_encoding::BASE32_NOPAD;
use rand::Rng;

use crate::errors::MibigError;

pub fn generate_uid(length: usize) -> Result<String, MibigError> {
    let mut rng = rand::thread_rng();
    let mut token_bytes: Vec<u8> = vec![0; length];

    rng.fill(&mut token_bytes[..]);

    let token = BASE32_NOPAD.encode(&token_bytes);

    Ok(token)
}

pub fn generate_password(password: String) -> Result<String, MibigError> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}
