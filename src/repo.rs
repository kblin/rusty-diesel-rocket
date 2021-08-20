use serde_json;
use structopt::clap::arg_enum;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct RepoOpts {
    #[structopt(subcommand)]
    cmd: Option<RepoSubcommand>,
}

#[derive(Debug, StructOpt)]
enum RepoSubcommand {
    #[structopt(name = "list", about = "List repository entries")]
    List(RepoListOpts),

    #[structopt(name = "import", about = "Import a new entry")]
    Import(RepoImportOpts),
}

#[derive(Debug, StructOpt)]
pub struct RepoListOpts {
    #[structopt(short, long, help = "Status of the entry", possible_values = &RepoListEntryStatus::variants(), case_insensitive = true, default_value = "Published")]
    status: RepoListEntryStatus,
}

structopt::clap::arg_enum! {
    #[derive(Debug)]
    enum RepoListEntryStatus {
        Published,
        Retired,
        Embargoed,
        Reserved,
    }
}

#[derive(Debug, StructOpt)]
pub struct RepoImportOpts {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
}

pub fn repo(cfg: RepoOpts) {
    match cfg.cmd {
        Some(cmd) => match cmd {
            RepoSubcommand::List(opts) => repo_list(opts.status),
            RepoSubcommand::Import(opts) => repo_import(opts.infile),
        },
        None => repo_list(RepoListEntryStatus::Published),
    }
}

use super::models::schema;

fn repo_import(infile: std::path::PathBuf) {
    let content = std::fs::read_to_string(&infile).expect("could not read file");
    let entry: schema::MibigEntry = serde_json::from_str(&content).unwrap();
    //println!("{:?}", entry);
    let return_value = serde_json::to_string_pretty(&entry).expect("failed to serialize");
    println!("{}", return_value)
}

fn repo_list(status: RepoListEntryStatus) {
    println!("Listing stuff on level {:?}", status)
}
