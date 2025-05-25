mod commands;

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// cbproj is a project manager used to manage projects that use the Cobalt programming language.
/// It uses 
struct TopLevel {
    #[argh(subcommand)]
    nested: Command,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Command {
    Init(commands::init::Init)
}


fn main() {
    let command: TopLevel = argh::from_env();

    match command.nested {
        Command::Init(_) => {
            commands::init::run();
        }
    }
}