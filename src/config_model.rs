use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(flatten)]
    pub profiles: HashMap<String, Profile>,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub email: String,
    pub name: String,
}

impl Config {
    pub fn from_file<P>(p: P) -> Result<Config, String>
    where
        P: AsRef<Path>,
    {
        // TODO more info on errors
        let conf_toml = fs::read_to_string(p).map_err(|_| "failed to read config")?;
        let config: Config = toml::from_str(&conf_toml).map_err(|_| "failed to parse config")?;
        Ok(config)
    }
}
