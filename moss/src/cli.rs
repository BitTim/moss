use clap::{Parser, Subcommand};

use crate::commands::account::AccountCommand;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = "A smart and storage optimizing Minecraft launcher"
)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Manage accounts
    #[command(subcommand)]
    Account(AccountCommand),
}
