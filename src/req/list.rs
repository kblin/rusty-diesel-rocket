use diesel::pg::PgConnection;
use exitcode;
use std::process;

use crate::models::db::submission_requests::SubmissionRequest;

pub fn req_list(conn: PgConnection) {
    println!("id\tuser\tcompounds\taccession\tstart\tend");
    match SubmissionRequest::list(&conn) {
        Ok(results) => {
            for request in results {
                println!(
                    "{id}\t{user}\t{compounds:?}\t{start:?}\t{end:?}",
                    id = request.id,
                    user = request.user_id,
                    compounds = request.compounds,
                    start = request.start_nt,
                    end = request.end_nt,
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(exitcode::DATAERR);
        }
    };
}
