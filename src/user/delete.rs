use diesel::pg::PgConnection;
use structopt::StructOpt;

use crate::models::db::submitters::Submitter;

#[derive(Debug, StructOpt)]
pub struct UserDeleteOpts {
    #[structopt(help = "user ID or email of user to delete")]
    identifier: String,
}

pub fn user_delete(opts: UserDeleteOpts, conn: PgConnection) {
    let id: String;
    if opts.identifier.contains("@") {
        id = Submitter::get_id_by_email(opts.identifier.into(), &conn).unwrap();
    } else {
        id = opts.identifier;
    }

    match Submitter::delete(&id, &conn) {
        Ok(_) => println!("Deleted user with id {id}."),
        Err(e) => println!("Deleting user failed: {e}"),
    }
}
