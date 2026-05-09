use crate::app;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub email: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles(HashMap<String, Profile>);

impl Profiles {
    pub fn from_file<P: AsRef<Path>>(p: P) -> Result<Profiles, String> {
        let conf_toml = fs::read_to_string(p).map_err(|e| e.to_string())?;
        let config: Profiles = toml::from_str(&conf_toml).map_err(|e| e.to_string())?;
        Ok(config)
    }

    /// Loads Profiles map from config file.
    /// It uses app's config_dir() and "profiles" directory appended.
    ///
    /// If you want to parse Profiles from other config file, use Profiles::from_ffle().
    pub fn from_configured() -> Result<Profiles, String> {
        let profiles_file_path = profiles_path();
        Self::from_file(profiles_file_path)
    }
}

impl Deref for Profiles {
    type Target = HashMap<String, Profile>;

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
