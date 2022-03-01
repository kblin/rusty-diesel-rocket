use std::io::{self, Write};

use rpassword::read_password_from_tty;

#[derive(Debug)]
pub struct NewUser {
    pub email: Option<String>,
    pub name: Option<String>,
    pub call_name: Option<String>,
    pub institution: Option<String>,
    pub is_public: bool,
    pub gdpr_consent: bool,
    pub active: bool,
}

pub fn interactive_user_edit(user: &mut NewUser) -> String {
    user.email = read_string_value("Email", &user.email, false);
    user.name = read_string_value("Name", &user.name, true);
    if user.call_name.is_none() && user.name.is_some() {
        if let Some(first_part) = user.name.clone().unwrap().splitn(2, " ").next() {
            user.call_name = Some(first_part.to_string())
        }
    }
    user.call_name = read_string_value("Call name", &user.call_name, true);
    user.institution = read_string_value("Organisation", &user.institution, true);
    let new_password = read_password();
    user.is_public = read_bool("Public profile", user.is_public);
    user.gdpr_consent = read_bool("GDPR consent given", user.gdpr_consent);
    user.active = read_bool("Active", user.active);
    new_password
}

fn read_string_value(
    question: &str,
    old_val_opt: &Option<String>,
    optional: bool,
) -> Option<String> {
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
        if optional {
            return None;
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
