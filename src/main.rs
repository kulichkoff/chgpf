use std::env;
use std::process::exit;

use chgpf::config_model::Config;
use chgpf::{change_profile, get_config_path, get_profile_argument};

fn main() {
    initializa_logger();
    log::info!("Application started");

    let input_profile = get_profile_argument();

    let config_path = get_config_path();
    let config = match Config::from_file(config_path) {
        Ok(conf) => conf,
        Err(err) => {
            log::error!("Failed to load config: {}", err);
            eprintln!("Failed to find load config. Ensure you have created it");
            exit(1);
        }
    };

    let profile = config
        .profiles
        .get(&input_profile)
        .expect("failed to find profile");

    change_profile(profile);

    println!("Changed git profile: {}", &profile.email);
}

fn initializa_logger() {
    let log_level = env::var("RUST_LOG").unwrap_or_default();
    
    if log_level.is_empty() {
        env_logger::Builder::new()
            .filter(None, log::LevelFilter::Off)
            .init();
    } else {
        env_logger::Builder::new()
            .filter(None, log::LevelFilter::Info)
            .init();
    }
}
