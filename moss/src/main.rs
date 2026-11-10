use std::path::PathBuf;

use clap::Parser;

use crate::{
    cli::{Args, Commands},
    paths::Paths,
};
use mosslib::account::{Accounts, AccountsConfig};

mod cli;
mod commands;
mod config;
mod constants;
mod paths;

pub(crate) use config::Config;

fn init_accounts(accounts_file: PathBuf) -> anyhow::Result<Accounts> {
    Ok(Accounts::new(AccountsConfig {
        msa_client_id: constants::MSA_CLIENT_ID.into(),
        auth_db_path: accounts_file,
    })?)
}

fn main() -> anyhow::Result<()> {
    let paths = Paths::new(None)?;
    let config = Config::load(&paths.config_file, &paths.local_data)?;
    let store_path = config.data_path.join("store");
    let instance_path = config.data_path.join("instance");

    let cli = Args::parse();

    match cli.cmd {
        Commands::Account(cmd) => {
            let accounts = init_accounts(paths.accounts_file())?;
            commands::account::run(cmd, accounts)?;
        }
    }

    Ok(())
}
