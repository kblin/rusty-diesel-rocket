use diesel::pg::PgConnection;
use exitcode;
use std::process;
use structopt::StructOpt;

use crate::models::db::submission_requests::SubmissionRequest;

#[derive(Debug, StructOpt)]
pub struct ReqCreateOpts {
    #[structopt(help = "User ID of the user creating the request")]
    user_id: String,
    #[structopt(help = "NCBI GenBank accession of the sequence the entry is found on")]
    accession: String,
    #[structopt(
        short,
        long,
        help = "Start coordinate of the cluster in the given record"
    )]
    start_nt: Option<i32>,
    #[structopt(
        short,
        long,
        help = "End coordinate of the cluster in the given record"
    )]
    end_nt: Option<i32>,
    #[structopt(min_values = 1, help = "Compound(s) created by the cluster")]
    compounds: Vec<String>,
}

pub fn req_create(opts: ReqCreateOpts, conn: PgConnection) {
    if opts.compounds.len() < 1 {
        eprintln!("Error: Need at least one compound name!");
        process::exit(exitcode::USAGE);
    }
    match SubmissionRequest::new(
        opts.user_id,
        opts.compounds,
        opts.accession,
        opts.start_nt,
        opts.end_nt,
        &conn,
    ) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(exitcode::DATAERR);
        }
    }
}
