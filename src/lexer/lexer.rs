use phf::phf_map;

use super::tokens::{Token, TokenType};

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "let" => TokenType::Let,
    "const" => TokenType::Const,
    "fn" => TokenType::Fn,
    "return" => TokenType::Return
};

pub struct Lexer {
    pub src: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(src: Vec<char>) -> Lexer {
        Self {
            src,
            pos: 0
        }
    }

    fn peek(&self) -> char {
        if self.pos >= self.src.len() {
            // sentinel value
            return '\0'
        }
        self.src[self.pos]
    }

    fn read(&mut self) -> char {
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
    ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
}

fn is_skippable(ch: char) -> bool {
    ch == '\r' || ch == '\n' || ch == '\t' || ch == ' '
}

fn isint(ch: char) -> bool {
    ('0'..='9').contains(&ch)
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
            ':' => {
                tokens.push(Token { value: ch.to_string(), r#type: TokenType::Colon });
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

                if let Some(&keyword_type) = KEYWORDS.get(ident.as_str()) {
                // get the tokentype from the keywords
                tokens.push(Token { 
                    value: ident, 
                    r#type: keyword_type
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