use diesel::pg::PgConnection;
use exitcode;
use std::process;

use crate::models::db::submitters::Submitter;

pub fn user_list(conn: PgConnection) {
    println!("ID\tEmail\tName\tPublic\tGDPR\tActive");
    match Submitter::all(&conn) {
        Ok(results) => {
            for submitter in results {
                println!(
                    "{id}\t{email}\t{name}\t{public}\t{gdpr}\t{active}",
                    id = submitter.user_id,
                    email = submitter.email,
                    name = submitter.name.unwrap_or("".to_string()),
                    public = submitter.is_public,
                    gdpr = submitter.gdpr_consent,
                    active = submitter.active
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(exitcode::DATAERR);
        }
    }
}
