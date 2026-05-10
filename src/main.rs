use bpaf::*;
use std::process::exit;

use chgpf::config::Profiles;
use chgpf::git;

#[derive(Debug, Clone)]
enum Command {
    Switch { profile: String },
    List,
}

fn switch_cmd() -> impl Parser<Command> {
    positional::<String>("PROFILE").map(|profile| Command::Switch { profile })
}

fn list_cmd() -> impl Parser<Command> {
    pure(Command::List).to_options().command("list")
}

fn parser() -> OptionParser<Command> {
    construct!([list_cmd(), switch_cmd(),])
        .to_options()
        .descr("Switch Git profiles")
}

fn main() {
    let cmd = parser().run();

    match cmd {
        Command::List => {
            // TODO: handle errors better
            let profiles = Profiles::from_configured().unwrap();
            let mut first = true;
            for (profile_name, profile) in profiles.iter() {
                if !first {
                    println!("---------")
                }
                first = false;
                println!("[{}]", profile_name);
                let profile_str = toml::to_string_pretty(profile).unwrap();
                print!("{}", profile_str);
            }
        },
        Command::Switch { profile } => {
            let profiles = match Profiles::from_configured() {
                Ok(conf) => conf,
                Err(_) => {
                    exit(1);
                }
            };

            let profile = match profiles.get(&profile) {
                Some(prof) => prof,
                None => {
                    exit(1);
                }
            };

            if git::Config::set_profile(profile).is_err() {
                exit(1)
            }

            println!("Switched: {}", &profile.email);
        }
    };
}
