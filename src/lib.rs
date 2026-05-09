pub mod app;
pub mod config;
pub mod git;

use std::env;
use std::process::exit;

pub fn get_profile_argument() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: chgpf [profile_name]");
        exit(1);
    }

    args[1].clone()
}
