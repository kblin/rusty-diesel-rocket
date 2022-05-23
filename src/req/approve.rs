use diesel::pg::PgConnection;
use exitcode;
use std::process;
use structopt::StructOpt;

use crate::models::db::submission_requests::SubmissionRequest;

#[derive(Debug, StructOpt)]
pub struct ReqApproveOpts {
    #[structopt(help = "ID of request to approve")]
    id: i64,
}

pub fn req_approve(opts: ReqApproveOpts, _conn: PgConnection) {
    eprintln!("Accepting {id}", id = opts.id)
}
