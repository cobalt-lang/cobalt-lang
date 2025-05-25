use argh::FromArgs;

use std::env::consts::{OS, ARCH};
use colored::Colorize;

#[derive(FromArgs, Debug)]
/// Displays the current cobalt version
#[argh(subcommand, name = "version")]
pub struct Version {}

pub fn run() {
    let version = env!("CARGO_PKG_VERSION");

    let app_name = "cobaltc";
    let rest = format!("{}/{} v{}", OS, ARCH, version);

    let version_message = format!(
        "{} {}",
        app_name.blue(),
        rest.bold(),
    );

    println!("{}", version_message);
}