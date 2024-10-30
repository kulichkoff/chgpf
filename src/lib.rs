pub mod config_model;

use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

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
