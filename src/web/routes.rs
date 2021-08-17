use super::handlers;

pub fn get_routes() -> std::vec::Vec<rocket::Route> {
    routes![
        handlers::version,
        handlers::stats,
        handlers::repository,
        handlers::login,
        handlers::logout,
        handlers::register,
        handlers::activate,
    ]
}
