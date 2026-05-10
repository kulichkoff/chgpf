use std::process::{Command, ExitStatus};
use thiserror::Error;

use crate::config::Profile;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("failed access to git executable: {source}")]
    Spawn {
        #[source]
        source: std::io::Error,
    },

    #[error("git exited ({status}): {stderr}")]
    Failed { status: ExitStatus, stderr: String },

    #[error("git produced invalid UTF-8")]
    InvalidUtf8,
}

pub struct Config;

impl Config {
    pub fn email() -> Result<String, GitError> {
        let output = Command::new("git")
            .args(["config", "--global", "user.email"])
            .output()
            .map_err(|e| GitError::Spawn { source: e })?;

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).map_err(|_| GitError::InvalidUtf8)?;
            return Err(GitError::Failed {
                status: output.status,
                stderr,
            });
        }

        let stdout = String::from_utf8(output.stdout).map_err(|_| GitError::InvalidUtf8)?;

        Ok(stdout)
    }

    /// Calls git to globally set config "user.email"
    pub fn set_email(email: &str) -> Result<(), GitError> {
        let output = Command::new("git")
            .args(["config", "--global", "user.email", email])
            .output()
            .map_err(|e| GitError::Spawn { source: e })?;

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).map_err(|_| GitError::InvalidUtf8)?;
            return Err(GitError::Failed {
                status: output.status,
                stderr,
            });
        }

        Ok(())
    }

    pub fn name() -> Result<String, GitError> {
        let output = Command::new("git")
            .args(["config", "--global", "user.name"])
            .output()
            .map_err(|e| GitError::Spawn { source: e })?;

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).map_err(|_| GitError::InvalidUtf8)?;
            return Err(GitError::Failed {
                status: output.status,
                stderr,
            });
        }

        let stdout = String::from_utf8(output.stdout).map_err(|_| GitError::InvalidUtf8)?;

        Ok(stdout)
    }

    /// Calls git to globally set config "user.name"
    pub fn set_name(name: &str) -> Result<(), GitError> {
        let output = Command::new("git")
            .args(["config", "--global", "user.name", name])
            .output()
            .map_err(|e| GitError::Spawn { source: e })?;

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).map_err(|_| GitError::InvalidUtf8)?;
            return Err(GitError::Failed {
                status: output.status,
                stderr,
            });
        }

        Ok(())
    }

    /// Calls git to globally set all the fields of Profile struct
    pub fn set_profile(profile: &Profile) -> Result<(), GitError> {
        Self::set_email(&profile.email).and(Self::set_name(&profile.name))
    }
}
