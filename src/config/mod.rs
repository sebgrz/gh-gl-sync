use std::fs::File;

use serde::Deserialize;
use serde_yaml;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub provider: Provider,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Provider {
    pub github: ProviderConfig,
    pub gitlab: ProviderConfig,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ProviderConfig {
    pub hostname: String,
    pub https: bool,
    pub username: String,
    pub token: String,
    pub private_ssh_key: String,
}

#[derive(Debug)]
pub struct ConfigError(String);

pub fn load(path: &str) -> Result<Config, ConfigError> {
    let file = File::open(path).map_err(|e| ConfigError(e.to_string()))?;
    let config: Config = serde_yaml::from_reader(file).map_err(|e| ConfigError(e.to_string()))?;

    Ok(config)
}
