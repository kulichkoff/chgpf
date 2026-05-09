pub mod app;
pub mod config;

use std::env;
use std::process::{exit, Command};

use config::Profile;

pub fn change_profile(profile: &Profile) {
    if let Err(_) = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .arg(&profile.email)
        .status()
    {
        exit(1);
    }

    if let Err(_) = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .arg(&profile.name)
        .status()
    {
        exit(1);
    }
}

pub fn get_profile_argument() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: chgpf [profile_name]");
        exit(1);
    }

    args[1].clone()
}
