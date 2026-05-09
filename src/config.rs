use std::path::PathBuf;

use crate::app;

pub fn profiles_path() -> PathBuf {
    app::config_dir().join("profiles")
}
