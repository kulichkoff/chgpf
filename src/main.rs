use std::process::exit;

use chgpf::config::profiles_path;
use chgpf::config::Config;
use chgpf::get_profile_argument;
use chgpf::git;

fn main() {
    let input_profile = get_profile_argument();

    let config_path = profiles_path();
    let config = match Config::from_file(config_path) {
        Ok(conf) => conf,
        Err(_) => {
            exit(1);
        }
    };

    let profile = match config.profiles.get(&input_profile) {
        Some(prof) => prof,
        None => {
            exit(1);
        },
    };

    if git::Config::set_profile(profile).is_err() {
        exit(1)
    }

    println!("Changed git profile: {}", &profile.email);
}
