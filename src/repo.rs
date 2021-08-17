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
    Import {},
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

pub fn repo(cfg: RepoOpts) {
    println!("Handle repo using {:?}", cfg)
}
