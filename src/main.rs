// this file will have a purpose in the future (or be deleted)
// for now it's just for testing random stuff during development

mod lexer;
mod parser;
mod interpreter;

fn main() {
    let src = "(774 + 123) * 77 / 4 - (85 * 3)";
    let src_as_chars: Vec<char> = src.chars().collect();

    let mut lexer_ = lexer::lexer::Lexer::new(src_as_chars);
    let tokens: Vec<lexer::tokens::Token> = lexer_.lex();

    for token in &tokens {
        println!("Token Type: {:?}, Token Value: {}", token.r#type, token.value);
    }

    let mut parser_ = parser::parser::Parser::new(tokens);
    let ast = parser_.produce_ast();

    for stmt in &ast.body {
        println!("{:?}", stmt);
    }
}