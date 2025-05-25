mod commands;

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// The compiler that converts Cobalt (.cb) files into Cobalt bytecode (.cbx) files.
struct TopLevel {
    #[argh(subcommand)]
    nested: Command,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Command {
    Build(commands::build::Build),
    Version(commands::version::Version)
}


fn main() {
    let command: TopLevel = argh::from_env();

    match command.nested {
        Command::Build(build) => {
            commands::build::run(build);
        }
        Command::Version(_) => {
            commands::version::run();
        }
    }
}