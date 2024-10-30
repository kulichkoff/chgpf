use chgpf::config_model::Config;
use chgpf::{change_profile, get_config_path, get_profile_argument};

fn main() {
    let input_profile = get_profile_argument();

    let config_path = get_config_path();
    let config = Config::from_file(config_path).unwrap();

    let profile = config
        .profiles
        .get(&input_profile)
        .expect("failed to find profile");

    change_profile(profile);

    println!("Changed git profile: {}", &profile.email);
}
