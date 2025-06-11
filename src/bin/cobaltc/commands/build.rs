use argh::FromArgs;

use std::path::PathBuf;
use std::{env, process};

use cobalt_lang::lexer::lexer;
use cobalt_lang::parser::parser;
use cobalt_lang::codegen::generator;
use cobalt_lang::utils::files_u8;

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "build")]
/// Compile a .cb file into .cbx (bytecode).
pub struct Build {
    #[argh(positional)]
    /// the name of the file to be compiled
    pub file: String,
    #[argh(option, short = 'o')]
    /// the name of the output file
    pub output: Option<String>,
    #[argh(switch)]
    /// whether to enable or disable debug mode, which provides detailed information
    pub debug: bool
}

pub fn run(args: Build) {
    let file_path: PathBuf = env::current_dir().unwrap_or_else(|_| {
        eprintln!("Error getting current directory");
        std::process::exit(1);
    });

    let file_content = std::fs::read_to_string(file_path.join(&args.file)).unwrap_or_else(|e| { 
        eprintln!("Error: Could not open file: {}", e); 
        std::process::exit(1); 
    });

    let output_file_name: String = if args.output.is_none() {
        files_u8::get_file_name_without_extension(&args.file).unwrap_or_else(|| {
            eprintln!("Error: Failed to extract file name from file positional argument. Try specifying the -o/--output flag.");
            process::exit(1);
        })
    } else {
        args.output.unwrap()
    };


    let mut lexer_ = lexer::Lexer::new(file_content.chars().collect());
    let tokens = lexer_.lex();

    if args.debug {
        for token in &tokens {
            println!("TokenType: {:?}, Value: {}", token.r#type, token.value);
        }
    }

    let mut parser_ = parser::Parser::new(tokens);
    let ast = parser_.produce_ast();

    if args.debug {
        for node in &ast.body {
            println!("{:#?}", node);
        }
    }

    let mut codegen_ = generator::Codegen::new();
    let bytecode = codegen_.generate(ast.body, "global");

    if args.debug {
        println!("{:?}", bytecode);
    }

    files_u8::write_vec_to_file(file_path.join(output_file_name + ".cbx").to_str().expect("Error: Failed to convert working directory into a string."), bytecode);
}