use std::{collections::HashMap, path::Path};

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
    pub fn from_file<P>(p: P)
    where
        P: AsRef<Path>,
    {
    }
}
