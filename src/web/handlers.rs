use chrono::Duration;
use diesel::result::Error as DieselError;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::errors::MibigError;
use crate::models::db::submitters::Submitter;
use crate::models::db::tokens::Token;
use crate::utils;
use crate::DBPool;

#[get("/version")]
pub fn version() -> &'static str {
    "version"
}

#[get("/stats")]
pub fn stats() -> &'static str {
    "stats"
}

#[get("/repository")]
pub fn repository() -> &'static str {
    "repository"
}

#[derive(Deserialize)]
pub struct UserLoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserTokenData {
    pub token: String,
    pub expiry: String,
}

#[post("/user/login", format = "json", data = "<user_data>")]
pub async fn login(
    conn: DBPool,
    user_data: Json<UserLoginData>,
) -> Result<Json<UserTokenData>, MibigError> {
    let token = conn
        .run(move |c| {
            let id: String;

            match Submitter::get_id_by_email(user_data.email.clone().into(), c) {
                Ok(val) => id = val,
                Err(MibigError::DatabaseError(DieselError::NotFound)) => {
                    return Err(MibigError::Unauthorised);
                }
                Err(e) => {
                    return Err(e);
                }
            };
            let user: Submitter;
            match Submitter::show(&id, c) {
                Ok(val) => user = val,
                Err(e) => return Err(e),
            }
            let valid_auth = utils::check_password(
                user_data.password.clone(),
                user.password_hash.map_or_else(|| "".to_string(), |v| v),
            )?;
            if !valid_auth {
                return Err(MibigError::Unauthorised);
            }
            let token = Token::new(
                user.user_id,
                Duration::days(1),
                "Authentication".to_string(),
            )?;
            Token::insert(&token, &c)?;
            Ok(token)
        })
        .await?;

    Ok(Json(UserTokenData {
        token: token.hash.to_owned(),
        expiry: token.expiry.format("%+").to_string(),
    }))
}

#[post("/user/logout")]
pub fn logout() -> &'static str {
    "logout"
}

#[post("/user/register")]
pub fn register() -> &'static str {
    "register"
}

#[put("/user/activate")]
pub fn activate() -> &'static str {
    "activate"
}
