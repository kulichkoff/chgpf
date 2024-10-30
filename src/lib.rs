pub mod config_model;

use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

use config_model::Profile;

pub fn change_profile(profile: &Profile) {
    if let Err(err) = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .arg(&profile.email)
        .status()
    {
        log::error!("Failed to execute git command: {}", err);
        exit(1);
    }

    if let Err(err) = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .arg(&profile.name)
        .status()
    {
        log::error!("Failed to execute git command: {}", err);
        exit(1);
    }
}

pub fn get_config_path() -> PathBuf {
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => {
            log::error!("Failed to find home directory");
            exit(1);
        }
    };
    home.join(".config").join("chgpf").join("profiles")
}

pub fn get_profile_argument() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: chgpf [profile_name]");
        exit(1);
    }

    args[1].clone()
}
