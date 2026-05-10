use anyhow::{Context, Result};
use bpaf::*;

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

fn list_profiles() -> Result<()> {
    let profiles = Profiles::from_configured()?;
    let mut first = true;
    for (profile_name, profile) in profiles.iter() {
        if !first {
            println!("---------");
        }
        first = false;
        println!("[{}]", profile_name);
        let profile_str = toml::to_string_pretty(profile)?;
        print!("{}", profile_str);
    }
    Ok(())
}

fn switch_profile(profile_name: &str) -> Result<()> {
    let profiles = Profiles::from_configured()?;

    let profile = profiles
        .get(profile_name)
        .context("profile with this name does not exist")?;

    git::Config::set_profile(profile)?;

    println!("Switched: {}", &profile.email);

    Ok(())
}

fn main() {
    let cmd = parser().run();

    match cmd {
        Command::List => {
            if let Err(err) = list_profiles() {
                eprintln!("Error: {err:#}");
                std::process::exit(1);
            }
        }
        Command::Switch { profile } => {
            if let Err(err) = switch_profile(&profile) {
                eprintln!("Error: {err:#}");
                std::process::exit(1);
            }
        }
    };
}
