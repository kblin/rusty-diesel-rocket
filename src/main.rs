#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate structopt;

pub use structopt::StructOpt;
mod errors;
mod models;
mod repo;
mod schema;
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
    #[structopt(short, long, help = "Set verbose output")]
    verbose: bool,

    #[structopt(subcommand)]
    cmd: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "serve", about = "Runs the web API")]
    Serve(ServeOpts),
    #[structopt(name = "repo", about = "Manages the repository")]
    Repo(repo::RepoOpts),
}

#[derive(Debug, StructOpt)]
pub struct ServeOpts {
    #[structopt(short, long, help = "Start server in development mode")]
    develop: bool,
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
        Subcommand::Serve(_) => {
            let _ = rocket().launch().await;
        }
        Subcommand::Repo(cfg) => repo::repo(cfg),
    };
}
