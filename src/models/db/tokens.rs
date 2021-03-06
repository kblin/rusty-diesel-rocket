use chrono::{DateTime, Duration, Utc};

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::outcome::try_outcome;
use rocket::request::{self, FromRequest, Outcome, Request};

use crate::errors::MibigError;
use crate::schema::tokens;
use crate::schema::tokens::dsl::tokens as all_tokens;
use crate::utils::generate_token_id;
use crate::DBPool;

#[derive(Insertable, Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "tokens"]
#[primary_key(hash)]
pub struct Token {
    pub hash: String,
    pub user_id: String,
    pub expiry: DateTime<Utc>,
    pub scope: String,
}

impl Token {
    pub fn new(user_id: String, ttl: Duration, scope: String) -> Result<Token, MibigError> {
        let hash = generate_token_id()?;
        let token = Token {
            hash: hash.to_owned(),
            user_id: user_id.to_owned(),
            expiry: Utc::now() + ttl,
            scope: scope.to_owned(),
        };
        Ok(token)
    }

    pub fn show(hash: String, conn: &PgConnection) -> Result<Token, MibigError> {
        let token = all_tokens.find(hash).first(conn)?;
        Ok(token)
    }

    pub fn all(conn: &PgConnection) -> Result<Vec<Token>, MibigError> {
        let res = all_tokens
            .order(tokens::user_id.desc())
            .load::<Token>(conn)?;
        Ok(res)
    }

    pub fn all_by_scope(scope: String, conn: &PgConnection) -> Result<Vec<Token>, MibigError> {
        let res = all_tokens
            .filter(tokens::scope.eq(scope))
            .order(tokens::user_id.desc())
            .load::<Token>(conn)?;
        Ok(res)
    }

    pub fn insert(token: &Token, conn: &PgConnection) -> Result<(), MibigError> {
        diesel::insert_into(tokens::table)
            .values(token)
            .execute(conn)?;
        Ok(())
    }

    pub fn delete_all_for_user(
        scope: String,
        user_id: String,
        conn: &PgConnection,
    ) -> Result<(), MibigError> {
        diesel::delete(
            all_tokens
                .filter(tokens::scope.eq(&scope))
                .filter(tokens::user_id.eq(&user_id)),
        )
        .execute(conn)?;
        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let token_id: String;
        let pool = try_outcome!(req.guard::<DBPool>().await);

        if let Some(cookie) = req.cookies().get_private("token") {
            token_id = cookie.value().to_string();
        } else {
            match req.headers().get_one("authorization") {
                None => token_id = "".to_string(),
                Some(key) => {
                    let mut splitter = key.splitn(2, " ");
                    splitter.next().unwrap_or("");
                    token_id = splitter.next().unwrap_or("").to_string();
                }
            }
        }
        if let Ok(token) = pool.run(|c| Token::show(token_id, c)).await {
            Outcome::Success(token)
        } else {
            Outcome::Failure((Status::BadRequest, ()))
        }
    }
}
