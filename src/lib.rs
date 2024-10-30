pub mod config_model;

use std::path::PathBuf;
use std::process::Command;

use config_model::Profile;

pub fn change_profile(profile: &Profile) {
    Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .arg(&profile.email)
        .status()
        .unwrap();

    Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .arg(&profile.name)
        .status()
        .unwrap();
}

pub fn get_config_path() -> PathBuf {
    let home = dirs::home_dir().expect("failed to get your home directory");
    home.join(".config").join("gprof").join("profiles")
}
