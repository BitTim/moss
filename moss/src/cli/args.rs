use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = "A smart and storage optimizing Minecraft launcher"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Manage accounts
    Account,
}
