extern crate bcrypt;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rpassword;
extern crate structopt;

pub use structopt::StructOpt;
mod errors;
mod models;
mod repo;
#[allow(unused_imports)]
mod schema;
mod user;
mod utils;

use dotenv::dotenv;
use rocket::figment::map;
use rocket::figment::value::{Map, Value};
use rocket_sync_db_pools::database;
use std::env;

#[database("mibig_db")]
pub struct DBPool(diesel::PgConnection);

#[derive(Debug, StructOpt)]
#[structopt(name = "mibig-api", about = "Manage the MIBiG database")]
pub struct Opts {
    #[structopt(subcommand)]
    cmd: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "serve", about = "Runs the web API")]
    Serve,
    #[structopt(name = "repo", about = "Manages the repository")]
    Repo(repo::RepoOpts),
    #[structopt(name = "user", about = "Manage MIBiG users")]
    User(user::UserOpts),
}

mod web;

fn rocket() -> rocket::Rocket<rocket::Build> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();

    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10.into(),
    };
    let figment = rocket::Config::figment().merge(("databases", map!["mibig_db" => db]));

    rocket::custom(figment)
        .mount("/api/v1", web::routes::get_routes())
        .attach(DBPool::fairing())
}

#[rocket::main]
async fn main() {
    let args = Opts::from_args();

    match args.cmd {
        Subcommand::Serve => {
            let _ = rocket().launch().await;
        }
        Subcommand::Repo(cfg) => repo::repo(cfg),
        Subcommand::User(cfg) => user::user(cfg),
    };
}
