use std::path::PathBuf;
use std::{env, process};

use cobalt_lang::interpreter::vm;
use cobalt_lang::utils::files_u8;

pub fn run(args: Vec<String>) {
    // executable path, command arg, file name arg
    if args.len() < 3 {
        eprintln!("Error: Expected the name of the file! Usage: cobalt run <file name>");
        process::exit(1);
    }

    let mut debug_mode: bool = false;

    if args.contains(&"--debug".to_string()) {
        debug_mode = true
    }

    let file_path: PathBuf = env::current_dir().unwrap_or_else(|_| {
        eprintln!("Error: Could not find current directory");
        std::process::exit(1);
    }).join(args.get(2).unwrap());

    let bytecode: Vec<u8> = files_u8::read_file_to_vec(file_path.to_str().expect("Error: Failed to convert working directory into a string."));
    let mut vm = vm::VM::new(bytecode, debug_mode);
    vm.interpret();
}