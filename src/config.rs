use crate::app;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::{collections::BTreeMap, fs, path::Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file not found: {0}")]
    NotFound(String),

    #[error("invalid config format")]
    InvalidFormat,

    #[error("config file already exists: {path}")]
    AlreadyExists { path: PathBuf },

    #[error("io error")]
    Io(#[from] std::io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub email: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles(BTreeMap<String, Profile>);

impl Profiles {
    pub fn new() -> Profiles {
        let hash_map = BTreeMap::new();
        Profiles(hash_map)
    }
    pub fn from_file<P: AsRef<Path>>(p: P) -> Result<Profiles, ConfigError> {
        let conf_toml = fs::read_to_string(&p).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => {
                ConfigError::NotFound(String::from(p.as_ref().to_str().unwrap()))
            }
            _ => ConfigError::Io(e),
        })?;
        let config: Profiles =
            toml::from_str(&conf_toml).map_err(|_| ConfigError::InvalidFormat)?;
        Ok(config)
    }

    /// Loads Profiles map from config file.
    /// It uses app's config_dir() and "profiles" directory appended.
    ///
    /// If you want to parse Profiles from other config file, use Profiles::from_ffle().
    pub fn from_configured() -> Result<Profiles, ConfigError> {
        let profiles_file_path = profiles_path();
        Self::from_file(profiles_file_path)
    }

    /// # Errors
    /// It may throw ConfigError::AlreadyExists if profiles config file is located.
    /// To skip the existance check, pass rewrite as true. It is better to ask user if they
    /// really wish to rewrite the existing config.
    pub fn save(&self, rewrite: bool) -> Result<(), ConfigError> {
        let config_dir = app::config_dir();
        fs::create_dir_all(config_dir)?;

        let profiles_config_path = profiles_path();
        if !rewrite && fs::exists(&profiles_config_path)? {
            return Err(ConfigError::AlreadyExists {
                path: profiles_config_path,
            });
        }

        let profiles_str = toml::to_string_pretty(self).map_err(|_| ConfigError::InvalidFormat)?;
        fs::write(profiles_config_path, profiles_str)?;

        Ok(())
    }
}

impl Default for Profiles {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Profile> for Profiles {
    fn from(value: Profile) -> Self {
        let mut profiles = Profiles::new();
        profiles.insert(String::from("default"), value);
        profiles
    }
}

impl Deref for Profiles {
    type Target = BTreeMap<String, Profile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Profiles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn profiles_path() -> PathBuf {
    app::config_dir().join("profiles")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_parse() {
        let toml_data = r#"
            [home]
            email = "home@example.com"
            name = "Home User"
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_data.as_bytes()).unwrap();

        let temp_path = temp_file.path();
        let config = Profiles::from_file(temp_path).unwrap();

        assert_eq!(config.len(), 1);
        assert_eq!(config["home"].email, "home@example.com");
        assert_eq!(config["home"].name, "Home User");
    }

    #[test]
    fn test_invalid_config() {
        let toml_data = r#"
            [home
            email = "home@example.com"
            name = "Home User"
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_data.as_bytes()).unwrap();

        let temp_path = temp_file.path();
        let result = Profiles::from_file(temp_path);

        assert!(result.is_err());
    }
}
