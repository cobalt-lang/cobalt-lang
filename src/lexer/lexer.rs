use std::collections::HashMap;

use super::tokens::{Token, TokenType};

pub struct Lexer {
    pub src: Vec<char>,
    pos: usize,
    keywords: HashMap<String, TokenType>
}

impl Lexer {
    pub fn new(src: Vec<char>) -> Lexer {
        let mut keywords: HashMap<String, TokenType> = Default::default();

        keywords.insert("let".to_string(), TokenType::Let);
        keywords.insert("const".to_string(), TokenType::Const);
        keywords.insert("fn".to_string(), TokenType::Fn);
        keywords.insert("return".to_string(), TokenType::Return);

        Self {
            src,
            pos: 0,
            keywords
        }
    }

    pub fn peek(&self) -> char {
        if self.pos >= self.src.len() {
            // sentinel value
            return '\0'
        }
        self.src[self.pos]
    }

    pub fn read(&mut self) -> char {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    pub fn lex(&mut self) -> Vec<Token> {
        lex_fn(self)
    }
}

// INTERNAL FUNCTIONS BELOW

fn is_alpha(ch: char) -> bool {
    (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
}

fn is_skippable(ch: char) -> bool {
    ch == '\r' || ch == '\n' || ch == '\t' || ch == ' '
}

fn isint(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

// LEXER FUNCTION BELOW (wrapped by the lex method in the Lexer struct)

fn lex_fn(l: &mut Lexer) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    while l.pos < l.src.len() {
        let ch = l.peek();

        // end when sentinel character appears
        if ch == '\0' {
            break;
        }

        match ch {
            '(' => {
                tokens.push(Token { value: ch.to_string(), r#type: TokenType::OpenParen });
                l.read();
            }

            ')' => {
                tokens.push(Token { value: ch.to_string(), r#type: TokenType::CloseParen });
                l.read();
            }

            '+' | '-' | '/' | '*' | '%' => {
                tokens.push(Token { value: ch.to_string(), r#type: TokenType::BinaryOperator });
                l.read();
            }

            '=' => {
                tokens.push(Token { value: ch.to_string(), r#type: TokenType::Equals });
                l.read();
            }

            ch if is_skippable(ch) => {
                l.read(); // just skip it
            }

            ch if isint(ch) => {
                let mut num: String = "".to_string();

                while isint(l.peek()) {
                    let next_digit = l.read().to_string();
                    num += next_digit.as_str();
                }

                tokens.push(Token { value: num, r#type: TokenType::Number });
            }

            // IDENTIFIERS AND KEYWORDS HANDLED IN THIS BLOCK!!!
            ch if is_alpha(ch) => {
                let mut ident: String = "".to_string();

                while is_alpha(l.peek()) {
                    let next_char = l.read().to_string();
                    ident += next_char.as_str();
                }

                if let Some(keyword_type) = l.keywords.get(&ident) {
                // get the tokentype from the keywords
                tokens.push(Token { 
                    value: ident, 
                    r#type: keyword_type.clone() 
                });
                } else {
                    // default to identifier
                    tokens.push(Token { 
                        value: ident, 
                        r#type: TokenType::Identifier 
                    });
                }
            }

            _ => {
                panic!("LEXER ERROR: Unrecognized character in source: {}", ch);
            }
        }
    }

    tokens.push(Token { value: "EOF".to_string(), r#type: TokenType::EOF });
    tokens
}