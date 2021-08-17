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

#[post("/user/login")]
pub fn login() -> &'static str {
    "login"
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
