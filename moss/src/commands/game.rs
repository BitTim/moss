use anyhow::Context;
use clap::{Subcommand, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum Loader {
    Vanilla,
}

impl From<Loader> for mosslib::game::Loader {
    fn from(value: Loader) -> Self {
        match value {
            Loader::Vanilla => Self::Vanilla,
        }
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum GameCommand {
    /// Search all available game versions online
    Search {
        /// The search query string
        query: String,
        /// Include snapshots in the list
        #[arg(long)]
        snapshot: bool,
        /// Limits the number of shown results
        #[arg(short = 'n', long, default_value_t = usize::MAX)]
        limit: usize,
        /// Filters the result by loader availability
        #[arg(short, long, default_value = "vanilla")]
        loader: Option<Loader>,
        /// Filters the result by game version
        #[arg(short, long)]
        game_version: Option<String>,
    },
    /// List all installed game versions present in the local store
    List,
}

pub(crate) fn run(cmd: GameCommand) -> anyhow::Result<()> {
    match cmd {
        GameCommand::Search {
            query,
            snapshot,
            limit,
            loader,
            game_version,
        } => search(&query, &loader),
        GameCommand::List => todo!(),
    }
}

fn search(query: &str, loader: &Option<Loader>) -> anyhow::Result<()> {
    let versions = mosslib::game::search(query, &loader.map(mosslib::game::Loader::from))
        .context("error during game search")?;

    if versions.is_empty() {
        println!("No matches found");
        return Ok(());
    }

    let mut table_builder = tabled::builder::Builder::new();
    for version in versions {
        table_builder.push_record([version.name]);
    }

    let mut table = table_builder.build();
    table.with(tabled::settings::Style::blank());
    println!("{}", table);

    Ok(())
}
