mod commands;

use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        exit(0);
    }
    let cmd = args[1].as_str();
    
    match cmd {
        "version" => commands::version::version(),
        _ => {
            println!("Invalid command, printing help message:");

        }
    }
}