#[macro_use]
extern crate rocket;
extern crate structopt;

pub use structopt::StructOpt;
mod models;
mod repo;
mod utils;

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
    rocket::build().mount("/api/v1", web::routes::get_routes())
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
