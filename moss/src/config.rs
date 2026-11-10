use std::{
    fs, io,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize, Default)]
struct RawConfig {
    data_path: Option<PathBuf>,
}

pub(crate) struct Config {
    pub(crate) data_path: PathBuf,
}

impl RawConfig {
    fn load(path: &Path) -> anyhow::Result<Self> {
        match fs::read_to_string(path) {
            Ok(text) => toml::from_str(&text)
                .with_context(|| format!("invalid config file {}", path.display())),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e).with_context(|| format!("could not read {}", path.display())),
        }
    }
}

impl Config {
    pub(crate) fn load(config_path: &Path, default_data_path: &Path) -> anyhow::Result<Self> {
        let raw = RawConfig::load(config_path)?;
        Ok(Self {
            data_path: raw.data_path.unwrap_or(default_data_path.to_owned()),
        })
    }
}
