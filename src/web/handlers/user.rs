use chrono::Duration;
use diesel::result::Error as DieselError;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::errors::MibigError;
use crate::models::db::submitters::Submitter;
use crate::models::db::tokens::Token;
use crate::DBPool;

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
    cookies: &CookieJar<'_>,
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
            let valid_auth = user.check_password(user_data.password.clone().to_string())?;
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

    let cookie = Cookie::build("token", token.hash.clone().to_string())
        // .expires(token.expiry.into())
        .finish();

    cookies.add_private(cookie);

    Ok(Json(UserTokenData {
        token: token.hash.to_owned(),
        expiry: token.expiry.format("%+").to_string(),
    }))
}

#[post("/user/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> &'static str {
    cookies.remove_private(Cookie::named("token"));
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
