use clap::Subcommand;
use mosslib::account::Accounts;

#[derive(Debug, Subcommand)]
pub(crate) enum AccountCommand {
    /// Log into and save an account
    #[command(alias = "login")]
    Add,
    /// List authenticated accounts
    List,
    /// Marks an account as the default. Only one account can be set as default
    Default,
    /// Log out of and remove an account
    Remove,
}

pub(crate) fn run(cmd: AccountCommand, accounts: Accounts) -> anyhow::Result<()> {
    match cmd {
        AccountCommand::Add => add(accounts),
        AccountCommand::List => list(accounts),
        _ => Ok(()),
    }
}

fn add(accounts: Accounts) -> anyhow::Result<()> {
    let username = accounts.add(|prompt| println!("{prompt:#?}"))?;
    println!("Successfully added {}", username);
    Ok(())
}

fn list(accounts: Accounts) -> anyhow::Result<()> {
    let accounts_iterator = accounts.list()?;
    let mut table_builder = tabled::builder::Builder::new();

    for acc in accounts_iterator {
        table_builder.push_record([acc.username, acc.uuid, acc.xuid]);
    }

    let mut table = table_builder.build();
    table.with(tabled::settings::Style::blank());

    println!("{}", table);
    Ok(())
}
