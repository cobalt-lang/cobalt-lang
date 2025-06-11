use phf::phf_map;

use super::tokens::{Token, TokenType};

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "let" => TokenType::Let,
    "const" => TokenType::Const,
    "fn" => TokenType::Fn,
    "return" => TokenType::Return,
    "true" => TokenType::True,
    "false" => TokenType::False,
    "if" => TokenType::If,
    "else" => TokenType::Else,
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
    ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_'
}

fn is_skippable(ch: char) -> bool {
    ch == '\r' || ch == '\n' || ch == '\t' || ch == ' '
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn is_alphanumeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
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

            '{' => {
                tokens.push(Token { value: ch.to_string(), r#type: TokenType::OpenBrace });
                l.read();
            }

            '}' => {
                tokens.push(Token { value: ch.to_string(), r#type: TokenType::CloseBrace });
                l.read();
            }

            '+' | '-' | '/' | '*' | '%' => {
                tokens.push(Token { value: ch.to_string(), r#type: TokenType::BinaryOperator });
                l.read();
            }

            '#' => {
                while l.peek() != '\n' && l.peek() != '\0' {
                    l.read();
                }
                if l.peek() == '\n' {
                    l.read(); // skip past it so we don't need to handle it later
                }
            }

            '=' => {
                l.read();
                if l.peek() == '=' {
                    tokens.push(Token { value: "==".to_string(), r#type: TokenType::BinaryOperator });
                    l.read();
                } else {
                    tokens.push(Token { value: ch.to_string(), r#type: TokenType::Equals });
                }
            }

            '>' | '<'  => {
                l.read();
                if l.peek() == '=' {
                    let value = ch.to_string() + "=";
                    tokens.push(Token { value, r#type: TokenType::BinaryOperator });
                    l.read();
                } else {
                    tokens.push(Token { value: ch.to_string(), r#type: TokenType::BinaryOperator });
                }
            }

            '!' => {
                l.read();
                if l.peek() == '=' {
                    tokens.push(Token { value: "!=".to_string(), r#type: TokenType::BinaryOperator});
                    l.read();
                } else {
                    tokens.push(Token { value: ch.to_string(), r#type: TokenType::Not });
                }
            }

            ':' => {
                tokens.push(Token { value: ch.to_string(), r#type: TokenType::Colon });
                l.read();
            }

            ch if is_skippable(ch) => {
                l.read(); // just skip it
            }

            ch if is_digit(ch) => {
                let mut num: String = "".to_string();

                while is_digit(l.peek()) {
                    num.push(l.read());
                }

                tokens.push(Token { value: num, r#type: TokenType::Number });
            }

            // IDENTIFIERS AND KEYWORDS HANDLED IN THIS BLOCK!!!
            ch if is_alpha(ch) => {
                let mut ident: String = "".to_string();

                while is_alphanumeric(l.peek()) {
                    ident.push(l.read());
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