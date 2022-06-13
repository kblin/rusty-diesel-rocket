use super::handlers;

pub fn get_routes() -> std::vec::Vec<rocket::Route> {
    routes![
        handlers::version,
        handlers::stats,
        handlers::repository,
        handlers::user::login,
        handlers::user::logout,
        handlers::user::register,
        handlers::user::activate,
    ]
}
