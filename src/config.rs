use std::fs::File;
use std::path::PathBuf;

use anyhow::Context;
use rustc_hash::FxHashMap;
use serde::Deserialize;

use crate::CONFIG;

#[derive(Debug, Deserialize)]
pub struct ConfigTls {
    pub cert: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigListen {
    pub ip: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tls: Option<ConfigTls>,
    pub listen: ConfigListen,
    pub ipset_reason: FxHashMap<String, String>
}

impl Config {
    pub fn global() -> &'static Config {
        CONFIG.get().expect("Config is not initialized")
    }
}

pub fn read_config() -> anyhow::Result<Config> {
    let file_path = PathBuf::from("./config.yaml");
    let f = File::open(file_path).with_context(|| "while reading config")?;
    let config: Config = serde_yaml::from_reader(f).with_context(|| "while reading config & deserializing")?;
    Ok(config)
}