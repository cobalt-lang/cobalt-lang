use std::path::PathBuf;
use std::{env, process};

use cobalt_lang::lexer::lexer;
use cobalt_lang::parser::parser;
use cobalt_lang::codegen::generator;
use cobalt_lang::utils::files_u8;

pub fn compile(args: Vec<String>) {
    // executable path, command arg, file name arg
    if args.len() < 3 {
        eprintln!("Error: Expected the name of the file! Usage: cobaltc compile <file name>");
        process::exit(1);
    }

    let file_path: PathBuf = env::current_dir().unwrap_or_else(|_| {
        eprintln!("Error getting current directory");
        std::process::exit(1);
    });

    let file_content = std::fs::read_to_string(file_path.join(args.get(2).unwrap())).unwrap_or_else(|e| { 
        eprintln!("Error: Could not open file: {}", e); 
        std::process::exit(1); 


    });

    let mut lexer_ = lexer::Lexer::new(file_content.chars().collect());
    let tokens = lexer_.lex();

    let mut parser_ = parser::Parser::new(tokens);
    let ast = parser_.produce_ast();

    let mut codegen_ = generator::Codegen::new();
    let bytecode = codegen_.generate(ast.body, "global");

    files_u8::write_vec_to_file(file_path.join("o.cbytes").to_str().expect("Error: Failed to convert working directory into a string."), bytecode);
}