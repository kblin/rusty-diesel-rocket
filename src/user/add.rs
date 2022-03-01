use diesel::pg::PgConnection;
use structopt::StructOpt;

use super::common::{interactive_user_edit, NewUser};
use crate::models::db::submitters::Submitter;
use crate::utils;

#[derive(Debug, StructOpt)]
pub struct UserAddOpts {
    #[structopt(short, long, help = "Added account is active")]
    active: bool,

    #[structopt(short = "C", long, help = "How to address the user")]
    call_name: Option<String>,

    #[structopt(short, long, help = "Email address of the user")]
    email: Option<String>,

    #[structopt(short, long, help = "Added account consents to us using the data")]
    gdpr_consent: bool,

    #[structopt(short, long, help = "Name of user's institute/company")]
    institution: Option<String>,

    #[structopt(short, long, help = "Name of user")]
    name: Option<String>,

    #[structopt(short, long, help = "Password of user")]
    password: Option<String>,

    #[structopt(short = "P", long, help = "Added account is public")]
    public: bool,
}

pub fn user_add(opts: UserAddOpts, conn: PgConnection) {
    let new_uid = utils::generate_uid(15).expect("Generating a uid failed");
    let mut new_user = NewUser {
        email: opts.email,
        name: opts.name,
        call_name: opts.call_name,
        institution: opts.institution,
        is_public: opts.public,
        gdpr_consent: opts.gdpr_consent,
        active: opts.active,
    };
    let password = opts.password.unwrap_or("".to_string());

    if new_user.email.is_none() || new_user.name.is_none() || password == "" {
        loop {
            let password = interactive_user_edit(&mut new_user);
            if new_user.email.is_some() && new_user.name.is_some() && password != "" {
                break;
            }
            println!("*** Invalid user data, please try again ***")
        }
    }

    let password_hash = utils::generate_password(password).unwrap();

    let submitter = Submitter {
        user_id: new_uid,
        email: new_user.email.unwrap().into(),
        name: new_user.name,
        call_name: new_user.call_name,
        institution: new_user.institution,
        password_hash: Some(password_hash),
        is_public: new_user.is_public,
        gdpr_consent: new_user.gdpr_consent,
        active: new_user.active,
        version: 0,
    };

    match Submitter::insert(submitter, &conn) {
        Ok(_) => println!("User added"),
        Err(e) => println!("Adding user failed: {e}"),
    }
}
