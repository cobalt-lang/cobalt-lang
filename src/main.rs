// this file will have a purpose in the future (or be deleted)
// for now it's just for testing random stuff during development

mod lexer;

fn main() {
    let src = "let mycoolvar = 123";
    let src_as_chars: Vec<char> = src.chars().collect();

    let mut lexer_ = lexer::lexer::Lexer::new(src_as_chars);
    let tokens: Vec<lexer::tokens::Token> = lexer_.lex();

    for token in tokens {
        println!("Token Type: {:?}, Token Value: {}", token.r#type, token.value);
    }
}