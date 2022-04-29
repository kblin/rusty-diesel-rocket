use diesel::pg::PgConnection;

use exitcode;
use std::process;

use structopt::clap::arg_enum;
use structopt::StructOpt;

use crate::models::db::tokens::Token;
use crate::utils;

#[derive(Debug, StructOpt)]
pub struct TokenOpts {
    #[structopt(subcommand)]
    cmd: Option<TokenSubcommand>,
}

#[derive(Debug, StructOpt)]
enum TokenSubcommand {
    #[structopt(name = "list", about = "List tokens")]
    List(TokenListOpts),
}

#[derive(Debug, StructOpt)]
pub struct TokenListOpts {
    #[structopt(short, long, help = "Scope of the token", possible_values = &TokenListScope::variants(), case_insensitive = true, default_value = "All")]
    scope: TokenListScope,
}

structopt::clap::arg_enum! {
    #[derive(Debug)]
    enum TokenListScope {
        All,
        Activation,
    }
}

pub fn token(cfg: TokenOpts) {
    let conn = utils::db::establish_connection();

    match cfg.cmd {
        Some(cmd) => match cmd {
            TokenSubcommand::List(opts) => token_list(opts.scope, conn),
        },
        None => token_list(TokenListScope::All, conn),
    }
}

fn token_list(scope: TokenListScope, conn: PgConnection) {
    println!("hash\tuser_id\texpiry\tscope");

    let res = match scope {
        TokenListScope::All => Token::all(&conn),
        _ => Token::all_by_scope(scope.to_string(), &conn),
    };

    match res {
        Ok(results) => {
            for token in results {
                println!(
                    "{hash:?}\t{user_id}\t{expiry:?}\t{scope}",
                    hash = token.hash,
                    user_id = token.user_id,
                    expiry = token.expiry,
                    scope = token.scope,
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(exitcode::DATAERR);
        }
    }
}
