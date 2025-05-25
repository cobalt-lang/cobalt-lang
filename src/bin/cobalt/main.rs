mod commands;

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// The virtual machine that interprets Cobalt bytecode (.cbx) files.
struct TopLevel {
    #[argh(subcommand)]
    nested: Command,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Command {
    Run(commands::run::Run),
    Version(commands::version::Version)
}

fn main() {
    let command: TopLevel = argh::from_env();

    match command.nested {
        Command::Run(run) => {
            commands::run::run(run);
        }
        Command::Version(_) => {
            commands::version::version();
        }
    }
}