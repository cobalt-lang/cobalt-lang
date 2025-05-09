mod commands;

use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        exit(0);
    }
    let cmd = args[1].as_str();

    match cmd {
        "init" => commands::init::init(),
        _ => {
            println!("Invalid command, printing help message:");

        }
    }
}