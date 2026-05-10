use anyhow::{Context, Result};
use bpaf::*;

use chgpf::config::{ConfigError, Profiles};
use chgpf::git;

#[derive(Debug, Clone)]
enum Command {
    Switch { profile: String },
    List,
    Init { force: bool },
}

impl Command {
    fn execute(&self) -> Result<()> {
        match self {
            Command::Switch { profile } => switch_profile(profile),
            Command::List => list_profiles(),
            Command::Init { force } => init_config(*force),
        }
    }
}

fn switch_cmd() -> impl Parser<Command> {
    positional::<String>("PROFILE").map(|profile| Command::Switch { profile })
}

fn list_cmd() -> impl Parser<Command> {
    pure(Command::List).to_options().command("list")
}

fn init_cmd() -> impl Parser<Command> {
    let force = long("force")
        .short('f')
        .help("Overwrite existing configuration")
        .switch();
    construct!(Command::Init { force })
        .to_options()
        .command("init")
}

fn parser() -> OptionParser<Command> {
    construct!([list_cmd(), init_cmd(), switch_cmd(),])
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

fn init_config(forced: bool) -> Result<()> {
    let git_profile = git::Config::profile()?;
    let profiles = Profiles::from(git_profile);
    profiles.save(forced).inspect_err(|e| {
        if let ConfigError::AlreadyExists { path: _ } = e {
            eprintln!(
                "If you want to rewrite existing profile, pass --force option calling chgpf init"
            );
        }
    })?;

    Ok(())
}

fn main() {
    let cmd = parser().run();

    if let Err(err) = cmd.execute() {
        eprintln!("Error: {err:#}");
        std::process::exit(1);
    }
}
