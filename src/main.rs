use std::path::PathBuf;
use std::process::Command;

use dirs::home_dir;

mod config_model;
use config_model::{Config, Profile};

fn main() {
    let input_profile = "home";

    let config_path = get_config_path();
    let config = Config::from_file(config_path).unwrap();

    let profile = config
        .profiles
        .get(input_profile)
        .expect("failed to find profile");

    change_profile(profile);

    println!("Changed git profile: {}", &profile.email);
}

fn get_config_path() -> PathBuf {
    let home = home_dir().expect("failed to get your home directory");

    home.join(".config").join("gprof").join("profiles")
}

fn change_profile(profile: &Profile) {
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
