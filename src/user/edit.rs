use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use exitcode;
use std::process;
use structopt::StructOpt;

use super::common::{interactive_user_edit, NewUser};
use crate::errors::MibigError;
use crate::models::db::submitters::Submitter;
use crate::utils::generate_password;

#[derive(Debug, StructOpt)]
pub struct UserEditOpts {
    #[structopt(help = "user ID or email of user to edit")]
    identifier: String,
}

pub fn user_edit(opts: UserEditOpts, conn: PgConnection) {
    let id: String;
    if opts.identifier.contains("@") {
        match Submitter::get_id_by_email(opts.identifier.clone().into(), &conn) {
            Ok(val) => id = val,
            Err(MibigError::DatabaseError(DieselError::NotFound)) => {
                eprintln!("User with email {} not found", opts.identifier);
                process::exit(exitcode::DATAERR)
            }
            Err(e) => {
                eprintln!("Error: {e}");
                process::exit(exitcode::DATAERR)
            }
        }
    } else {
        id = opts.identifier;
    }

    let old_submitter_res = Submitter::show(&id, &conn);

    if let Err(e) = old_submitter_res {
        match e {
            MibigError::DatabaseError(DieselError::NotFound) => {
                eprintln!("User with id {id} not found");
                process::exit(exitcode::DATAERR)
            }
            _ => {
                eprintln!("Error: {e}");
                process::exit(exitcode::DATAERR)
            }
        }
    }

    let old_submitter = old_submitter_res.unwrap();

    let mut new_user = NewUser {
        email: Some(old_submitter.email.to_string()),
        name: old_submitter.name,
        call_name: old_submitter.call_name,
        institution: old_submitter.institution,
        is_public: old_submitter.is_public,
        gdpr_consent: old_submitter.gdpr_consent,
        active: old_submitter.active,
    };

    let password = interactive_user_edit(&mut new_user);

    let password_hash: Option<String>;
    if password == "" {
        password_hash = old_submitter.password_hash;
    } else {
        password_hash = Some(generate_password(password).unwrap());
    }

    let new_submitter = Submitter {
        user_id: old_submitter.user_id,
        email: new_user.email.unwrap().into(),
        name: new_user.name,
        call_name: new_user.call_name,
        institution: new_user.institution,
        password_hash: password_hash,
        is_public: new_user.is_public,
        gdpr_consent: new_user.gdpr_consent,
        active: new_user.active,
        version: old_submitter.version,
    };

    match Submitter::update(&id, new_submitter, &conn) {
        Ok(version) => println!("Updated {id} to version {version}"),
        Err(e) => println!("Failed to update {id}: {e}"),
    }
}
