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
    Default {
        /// Either a valid UUID or a username
        target: String,
    },
    /// Refreshes the authentication token and username of a user. This is also done automatically
    /// when launching an instance.
    Refresh {
        /// Either a valid UUID or a username
        target: String,
    },
    /// Log out of and remove an account
    Remove {
        /// Either a valid UUID or a username
        target: String,
    },
}

pub(crate) fn run(cmd: AccountCommand, accounts: Accounts) -> anyhow::Result<()> {
    match cmd {
        AccountCommand::Add => add(accounts),
        AccountCommand::List => list(accounts),
        AccountCommand::Default { target } => default(target),
        AccountCommand::Refresh { target } => refresh(accounts, target),
        AccountCommand::Remove { target } => remove(accounts, target),
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
        table_builder.push_record([acc.username, acc.uuid.to_string(), acc.xuid]);
    }

    let mut table = table_builder.build();
    table.with(tabled::settings::Style::blank());

    println!("{}", table);
    Ok(())
}

fn default(target: String) -> anyhow::Result<()> {
    Ok(())
}

fn refresh(accounts: Accounts, target: String) -> anyhow::Result<()> {
    let new_account = accounts.refresh(&target)?;

    println!("Successfully refreshed account {}", target);

    if let Some(account) = new_account {
        println!("Some account details have changed");
        println!("UUID: {}", account.uuid);
        println!("Username: {}", account.username);
    }

    Ok(())
}

fn remove(accounts: Accounts, target: String) -> anyhow::Result<()> {
    Ok(())
}
