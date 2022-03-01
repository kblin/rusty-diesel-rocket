use structopt::StructOpt;

use crate::utils;

mod add;
mod common;
mod delete;
mod edit;
mod list;

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
    Add(add::UserAddOpts),
    #[structopt(name = "delete", about = "Delete a user")]
    Delete(delete::UserDeleteOpts),
    #[structopt(name = "edit", about = "Edit a user")]
    Edit(edit::UserEditOpts),
}

pub fn user(cfg: UserOpts) {
    let conn = utils::db::establish_connection();

    match cfg.cmd {
        Some(cmd) => match cmd {
            UserSubcommand::List => list::user_list(conn),
            UserSubcommand::Add(opts) => add::user_add(opts, conn),
            UserSubcommand::Edit(opts) => edit::user_edit(opts, conn),
            UserSubcommand::Delete(opts) => delete::user_delete(opts, conn),
        },
        None => list::user_list(conn),
    }
}
