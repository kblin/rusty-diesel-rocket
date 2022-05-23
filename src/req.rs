use structopt::StructOpt;

use crate::utils;

mod approve;
mod create;
mod delete;
mod list;

#[derive(Debug, StructOpt)]
pub struct ReqOpts {
    #[structopt(subcommand)]
    cmd: Option<ReqSubcommand>,
}

#[derive(Debug, StructOpt)]
enum ReqSubcommand {
    #[structopt(name = "list", about = "List requests")]
    List,
    #[structopt(name = "create", about = "Create a request")]
    Create(create::ReqCreateOpts),
    #[structopt(name = "delete", about = "Delete a request")]
    Delete(delete::ReqDeleteOpts),
    #[structopt(name = "approve", about = "Approve a request")]
    Approve(approve::ReqApproveOpts),
}

pub fn req(cfg: ReqOpts) {
    let conn = utils::db::establish_connection();

    match cfg.cmd {
        Some(cmd) => match cmd {
            ReqSubcommand::List => list::req_list(conn),
            ReqSubcommand::Create(opts) => create::req_create(opts, conn),
            ReqSubcommand::Delete(opts) => delete::req_delete(opts, conn),
            ReqSubcommand::Approve(opts) => approve::req_approve(opts, conn),
        },
        None => list::req_list(conn),
    }
}
