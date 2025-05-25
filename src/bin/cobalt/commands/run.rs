use argh::FromArgs;

use std::path::PathBuf;
use std::env;

use cobalt_lang::interpreter::vm;
use cobalt_lang::utils::files_u8;

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "run")]
/// Run a .cbx (bytecode) file.
pub struct Run {
    #[argh(positional)]
    /// the name of the file to be interpreted
    pub file: String,
    #[argh(switch)]
    /// whether to enable or disable debug mode, which provides detailed information
    pub debug: bool
}

pub fn run(args: Run) {
    let file_path: PathBuf = env::current_dir().unwrap_or_else(|_| {
        eprintln!("Error: Could not find current directory");
        std::process::exit(1);
    }).join(args.file);

    let bytecode: Vec<u8> = files_u8::read_file_to_vec(file_path.to_str().expect("Error: Failed to convert working directory into a string."));
    let mut vm = vm::VM::new(bytecode, args.debug);
    vm.interpret();
}