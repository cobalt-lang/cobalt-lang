use std::env::consts::{OS, ARCH};
use colored::Colorize;

pub fn version() {
    let app_name = "cobalt";
    let rest = format!("{}/{} v0.1.0", OS, ARCH);

    let version_message = format!(
        "{} {}",
        app_name.blue(),
        rest.bold(),
    );

    println!("{}", version_message);
}