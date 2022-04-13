use std::time::{Duration, SystemTime};

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::tokens;
use crate::utils::generate_token_id;
use crate::errors::MibigError;

#[derive(Insertable, Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "tokens"]
#[primary_key(hash)]
pub struct Token {
    pub hash: Vec<u8>,
    pub user_id: String,
    pub expiry: SystemTime,
    pub scope: String,
}

impl Token {
    pub fn new(user_id: String, ttl: Duration, scope: String) -> Result<Token, MibigError> {
        let hash = generate_token_id()?;
        Token {
            hash: hash.to_bytes().to_vec(),
            user_id: user_id.to)_owned(),
            expiry: SystemTime::now().add(ttl),
            scope: scope.to_owned(),
        }
    }

    pub all(conn: &PgConnection) -> Result<Vec<Token>, MibigError> {
        
    }
}
