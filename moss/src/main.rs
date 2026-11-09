use clap::Parser;

use crate::cli::{Cli, Commands::Account};
use mosslib::Result;
use mosslib::account::*;

mod cli;

fn main() -> Result<()> {
    let accounts = Accounts::new(AccountsConfig {
        msa_client_id: "".into(),
        auth_db_path: "./test-db.json".into(),
    })?;
    let cli = Cli::parse();

    match cli.cmd {
        Account => accounts.add(|prompt| println!("{prompt:#?}"))?,
    }

    Ok(())
}
