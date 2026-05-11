use assert_cmd::Command;
use std::{fs, path::PathBuf};

use tempfile::{TempDir, tempdir};

pub struct TestEnv {
    // becomes $HOME
    pub home: TempDir,
    // $HOME/.config-ish, becomes $XDG_CONFIG_HOME
    pub xdg: PathBuf,

    // becomes $GIT_CONFIG_GLOBAL
    git_config: PathBuf,
}

impl TestEnv {
    pub fn new() -> Self {
        let dir = tempdir().unwrap();

        let home = dir.path();
        let xdg = home.join(".testconfig");
        let git_config = home.join(".gitconfig");

        let git_config_contents = "[user]
email = danchick03@gmail.com
name = Daniel Kulichkov";
        fs::write(&git_config, git_config_contents).unwrap();

        Self {
            home: dir,
            xdg,
            git_config,
        }
    }

    pub fn cmd(&self) -> Command {
        let mut c = Command::cargo_bin("chgpf").unwrap();
        c.env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap())
            .env("HOME", self.home.path())
            .env("XDG_CONFIG_HOME", &self.xdg)
            .env("GIT_CONFIG_GLOBAL", &self.git_config);
        c
    }

    pub fn write_profiles(&self, toml: &str) {
        fs::write(&self.git_config, toml).unwrap();
    }

    pub fn git_email(&self) -> String {
        let config_contents = fs::read_to_string(&self.git_config).unwrap();

        for line in config_contents.split('\n') {
            let trimmed = line.trim();
            if trimmed.starts_with("email = ") {
                let email = trimmed.strip_prefix("email = ").unwrap();
                return String::from(email);
            }
        }

        String::new()
    }
}
