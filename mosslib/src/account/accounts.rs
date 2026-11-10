use crate::{Result, account::private_fs};

use portablemc::msa::{Auth, Database, DeviceCodeFlow};
use std::path::PathBuf;

pub struct AccountsConfig {
    pub msa_client_id: String,
    pub auth_db_path: PathBuf,
}

#[derive(Debug)]
pub struct DeviceCodePrompt {
    pub user_code: String,
    pub verification_uri: String,
    pub message: String,
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
    msa_client_id: String,
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
        let auth = Auth::new(&self.msa_client_id);
        let flow = auth.request_device_code()?;

        on_prompt(&DeviceCodePrompt::from_flow(&flow));

        let account = flow.wait()?;
        let username = account.username().to_owned();

        self.db.store(account)?;
        Ok(username)
    }
}
