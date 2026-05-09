use std::{
    io,
    process::{Command, ExitStatus},
};

use crate::config::Profile;

pub struct Config;

impl Config {
    /// Calls git to globally set config "user.email"
    pub fn set_email(email: &str) -> io::Result<ExitStatus> {
        Command::new("git")
            .args(["config", "--global", "user.email", email])
            .status()
    }

    /// Calls git to globally set config "user.name"
    pub fn set_name(name: &str) -> io::Result<ExitStatus> {
        Command::new("git")
            .args(["config", "--global", "user.name", name])
            .status()
    }

    /// Calls git to globally set all the fields of Profile struct
    pub fn set_profile(profile: &Profile) -> io::Result<ExitStatus> {
        Self::set_email(&profile.email).and(Self::set_name(&profile.name))
    }
}
