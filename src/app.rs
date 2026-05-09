use std::env;
use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    config_home().join("chgpf")
}

#[cfg(not(target_os = "macos"))]
fn config_home() -> PathBuf {
    use std::str::FromStr;

    env::var_os("XDG_CONFIG_HOME").map_or_else(
        || {
            let home = env::home_dir().unwrap();
            home.join(".config")
        },
        |var| PathBuf::from_str(var.to_str().unwrap()).unwrap(),
    )
}

#[cfg(target_os = "macos")]
fn config_home() -> PathBuf {
    let home = env::home_dir().unwrap();
    home.join(".config")
}
