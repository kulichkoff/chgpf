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
        let conf_toml = fs::read_to_string(p).map_err(|e| e.to_string())?;
        let config: Config = toml::from_str(&conf_toml).map_err(|e| e.to_string())?;
        Ok(config)
    }
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
        let config = Config::from_file(temp_path).unwrap();

        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.profiles["home"].email, "home@example.com");
        assert_eq!(config.profiles["home"].name, "Home User");
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
        let result = Config::from_file(temp_path);

        assert!(result.is_err());
    }
}
