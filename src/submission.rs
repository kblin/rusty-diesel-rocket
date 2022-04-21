use structopt::StructOpt;

use crate::utils;

mod create;
mod delete;
mod edit;
mod list;

#[derive(Debug, StructOpt)]
pub struct SubmissionOpts {
    #[structopt(subcommand)]
    cmd: Option<SubmissionSubcommand>,
}

#[derive(Debug, StructOpt)]
enum SubmissionSubcommand {
    #[structopt(name = "list", about = "List submission")]
    List,
    #[structopt(name = "create", about = "Create a submission")]
    Create(create::CreateSubmissionOpts),
    #[structopt(name = "delete", about = "Delete a submission")]
    Delete(delete::DeleteSubmissionOpts),
    #[structopt(name = "edit", about = "Edit a submission")]
    Edit(edit::EditSubmissionOpts),
}

pub fn submission(cfg: SubmissionOpts) {
    let conn = utils::db::establish_connection();

    match cfg.cmd {
        Some(cmd) => match cmd {
            SubmissionSubcommand::List => list::submission_list(conn),
            SubmissionSubcommand::Create(opts) => create::submission_create(opts, conn),
            SubmissionSubcommand::Edit(opts) => edit::submission_edit(opts, conn),
            SubmissionSubcommand::Delete(opts) => delete::submission_delete(opts, conn),
        },
        None => list::user_list(conn),
    }
}
