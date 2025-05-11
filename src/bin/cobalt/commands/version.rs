use std::env::consts::{OS, ARCH};
use colored::Colorize;

pub fn version() {
    let version = env!("CARGO_PKG_VERSION");

    let app_name = "cobalt";
    let rest = format!("{}/{} v{}", OS, ARCH, version);

    let version_message = format!(
        "{} {}",
        app_name.blue(),
        rest.bold(),
    );

    println!("{}", version_message);
}