use std::path::PathBuf;

use anyhow::Context;

use crate::constants;

pub(crate) struct Paths {
    pub(crate) config_file: PathBuf,
    pub(crate) local_data: PathBuf,
}

impl Paths {
    pub fn new(config_override: Option<PathBuf>) -> anyhow::Result<Self> {
        let local_data = dirs::data_local_dir()
            .context("could not determine the local data directory")?
            .join(constants::APP_NAME);

        let config_file = match config_override {
            Some(p) => p,
            None => dirs::config_dir()
                .context("could not determine the local data directory")?
                .join(constants::APP_NAME)
                .join("config.toml"),
        };

        Ok(Self {
            config_file,
            local_data,
        })
    }

    pub fn accounts_file(&self) -> PathBuf {
        self.local_data.join("accounts.json")
    }
}
