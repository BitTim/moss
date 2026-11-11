use crate::{Error, Result, account::private_fs};

use portablemc::msa::{Auth, Database, DeviceCodeFlow};
use std::path::PathBuf;
use uuid::Uuid;

pub struct AccountsConfig {
    pub msa_client_id: Uuid,
    pub auth_db_path: PathBuf,
}

#[derive(Debug)]
pub struct DeviceCodePrompt {
    pub user_code: String,
    pub verification_uri: String,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub struct Account {
    pub uuid: Uuid,
    pub username: String,
    pub xuid: String,
}

impl From<portablemc::msa::Account> for Account {
    fn from(value: portablemc::msa::Account) -> Self {
        Self {
            uuid: value.uuid(),
            username: value.username().into(),
            xuid: value.xuid().into(),
        }
    }
}

impl DeviceCodePrompt {
    fn from_flow(flow: &DeviceCodeFlow) -> Self {
        Self {
            user_code: flow.user_code().to_owned(),
            verification_uri: flow.verification_uri().to_owned(),
            message: flow.message().to_owned(),
        }
    }
}

pub struct Accounts {
    msa_client_id: Uuid,
    db: Database,
}

impl Accounts {
    pub fn new(config: AccountsConfig) -> Result<Self> {
        private_fs::ensure_private_db_file(&config.auth_db_path)?;
        let db = Database::new(config.auth_db_path);

        Ok(Self {
            msa_client_id: config.msa_client_id,
            db,
        })
    }

    pub fn add(&self, on_prompt: impl FnOnce(&DeviceCodePrompt)) -> Result<String> {
        let auth = Auth::new(&self.msa_client_id.to_string());
        let flow = auth.request_device_code()?;

        on_prompt(&DeviceCodePrompt::from_flow(&flow));

        let account = flow.wait()?;
        let username = account.username().to_owned();

        self.db.store(account)?;
        Ok(username)
    }

    pub fn list(&self) -> Result<impl Iterator<Item = Account>> {
        Ok(self.db.load_iter()?.map(Account::from))
    }

    pub fn refresh(&self, target: &str) -> Result<Option<Account>> {
        let mut account = if let Ok(uuid) = Uuid::parse_str(target) {
            self.db
                .load_from_uuid(uuid)?
                .ok_or(Error::NoAccountWithUuidFound { uuid })?
        } else {
            self.db
                .load_from_username(target)?
                .ok_or(Error::NoAccountWithUsernameFound {
                    username: target.into(),
                })?
        };

        let prev_acc = Account::from(account.to_owned());
        account.request_refresh()?;
        let new_acc = Account::from(account.to_owned());

        Ok(if prev_acc != new_acc {
            Some(new_acc)
        } else {
            None
        })
    }

    pub fn remove(&self, target: &str) -> Result<()> {
        if let Ok(uuid) = Uuid::parse_str(target) {
            self.db.remove_from_uuid(uuid)?;
        } else {
            self.db.remove_from_username(target)?;
        }

        Ok(())
    }
}
