use diesel::pg::PgConnection;
use exitcode;
use std::process;
use structopt::StructOpt;

use crate::models::db::submission_requests::SubmissionRequest;

#[derive(Debug, StructOpt)]
pub struct ReqDeleteOpts {
    #[structopt(help = "ID of the request to delete")]
    id: i64,
}

pub fn req_delete(opts: ReqDeleteOpts, conn: PgConnection) {
    match SubmissionRequest::delete(opts.id, &conn) {
        Ok(_) => eprintln!("Deleted request with id {id}", id = opts.id),
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(exitcode::DATAERR);
        }
    }
}
