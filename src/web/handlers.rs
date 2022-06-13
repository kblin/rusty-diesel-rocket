pub mod user;

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
