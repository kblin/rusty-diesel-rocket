use std::io::{self, Write};

use diesel::pg::PgConnection;
use rpassword::read_password_from_tty;
use structopt::StructOpt;

use crate::models::db::submitters::Submitter;
use crate::utils;

#[derive(Debug, StructOpt)]
pub struct UserOpts {
    #[structopt(subcommand)]
    cmd: Option<UserSubcommand>,
}

#[derive(Debug, StructOpt)]
enum UserSubcommand {
    #[structopt(name = "list", about = "List users")]
    List,
    #[structopt(name = "add", about = "Add a user")]
    Add(UserAddOpts),
}

#[derive(Debug, StructOpt)]
struct UserAddOpts {
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

pub fn user(cfg: UserOpts) {
    let conn = utils::db::establish_connection();

    match cfg.cmd {
        Some(cmd) => match cmd {
            UserSubcommand::List => user_list(conn),
            UserSubcommand::Add(opts) => user_add(opts, conn),
        },
        None => user_list(conn),
    }
}

fn user_list(conn: PgConnection) {
    println!("Listing users");
    let results = Submitter::all(&conn);
    for submitter in results {
        println!("{submitter:?}");
    }
}

#[derive(Debug)]
struct NewUser {
    email: Option<String>,
    name: Option<String>,
    call_name: Option<String>,
    institution: Option<String>,
    is_public: bool,
    gdpr_consent: bool,
    active: bool,
}

fn user_add(opts: UserAddOpts, conn: PgConnection) {
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

    if Submitter::insert(submitter, &conn) {
        println!("User added");
    } else {
        println!("Adding user failed");
    }
}

fn interactive_user_edit(user: &mut NewUser) -> String {
    user.email = read_string_value("Email", &user.email);
    user.name = read_string_value("Name", &user.name);
    user.call_name = read_string_value("Call name", &user.call_name);
    // TODO: Derive call_name from name if not given?
    user.institution = read_string_value("Organisation", &user.institution);
    let new_password = read_password();
    user.is_public = read_bool("Public profile", user.is_public);
    user.gdpr_consent = read_bool("GDPR consent given", user.gdpr_consent);
    user.active = read_bool("Active", user.active);
    new_password
}

fn read_string_value(question: &str, old_val_opt: &Option<String>) -> Option<String> {
    let mut new_val: String;
    let old_val = match old_val_opt {
        Some(val) => val.as_str(),
        None => "",
    };

    loop {
        let mut tmp_val = String::new();
        print!("{question} [{old_val}]: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut tmp_val).unwrap();
        tmp_val = tmp_val.trim().to_string();
        if tmp_val == "" {
            tmp_val = old_val.to_string();
        }
        new_val = tmp_val;
        if new_val.len() > 0 {
            break;
        }
    }
    Some(new_val)
}

fn read_password() -> String {
    let mut password: String;
    let mut password_repeat: String;

    loop {
        password = read_password_from_tty(Some("Password (empty to keep old): ")).unwrap();
        if password == "" {
            break;
        }

        password_repeat = read_password_from_tty(Some("Repeat password: ")).unwrap();

        if password == password_repeat {
            break;
        }
        println!("Password mismatch");
    }

    password
}

fn read_bool(question: &str, old_val: bool) -> bool {
    let new_val: bool;
    loop {
        let mut tmp_val = String::new();
        print!("{question} (true/false) [{old_val}]: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut tmp_val).unwrap();
        match tmp_val.to_lowercase().trim() {
            "t" | "true" | "y" | "yes" => {
                new_val = true;
                break;
            }
            "f" | "false" | "n" | "no" => {
                new_val = false;
                break;
            }
            "" => {
                new_val = old_val;
                break;
            }
            _ => println!("Invalid input: {}", tmp_val.trim()),
        }
    }
    new_val
}
