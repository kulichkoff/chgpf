use std::process::exit;

use chgpf::config_model::Config;
use chgpf::{change_profile, get_config_path, get_profile_argument};

fn main() {
    env_logger::init();
    log::info!("Application started");

    let input_profile = get_profile_argument();

    let config_path = get_config_path();
    let config = match Config::from_file(config_path) {
        Ok(conf) => conf,
        Err(err) => {
            log::error!("Failed to load config: {}", err);
            exit(1);
        }
    };

    let profile = match config.profiles.get(&input_profile) {
        Some(prof) => prof,
        None => {
            log::error!("Profile {} not found", &input_profile);
            exit(1);
        },
    };

    change_profile(profile);

    println!("Changed git profile: {}", &profile.email);
}
