use clap::Parser;

use crate::{
    cli::{Cli, Commands::Account},
    paths::Paths,
};
use mosslib::account::{Accounts, AccountsConfig};

mod cli;
mod config;
mod constants;
mod paths;

pub(crate) use config::Config;

fn main() -> anyhow::Result<()> {
    let paths = Paths::new(None)?;
    let config = Config::load(&paths.config_file, &paths.local_data)?;
    let store_path = config.data_path.join("store");
    let instance_path = config.data_path.join("instance");

    let accounts = Accounts::new(AccountsConfig {
        msa_client_id: constants::MSA_CLIENT_ID.into(),
        auth_db_path: paths.accounts_file(),
    })?;
    let cli = Cli::parse();

    match cli.cmd {
        Account => accounts.add(|prompt| println!("{prompt:#?}"))?,
    }

    Ok(())
}
