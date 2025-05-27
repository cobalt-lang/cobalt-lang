use crate::parser::ast;
use crate::lexer::tokens::{Token, TokenType};

use super::ast::VariableDeclaration;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Self {
            tokens,
            pos: 0
        }
    }

    fn not_eof(&self) -> bool {
        if let Some(current_token) = self.tokens.get(self.pos) {
            current_token.r#type != TokenType::EOF
        } else {
            false
        }
    }

    fn at(&self) -> Token {
        if let Some(current_token) = self.tokens.get(self.pos) {
            current_token.clone()
        } else {
            Token { value: "EOF".to_string(), r#type: TokenType::EOF }
        }
    }

    // Returns the current token (before it's eaten) and then eats it
    fn eat(&mut self) -> Token {
        if let Some(current_token) = self.tokens.get(self.pos) {
            self.pos += 1;
            current_token.clone()
        } else {
            Token { value: "EOF".to_string(), r#type: TokenType::EOF }
        }
    }

    // Eats the token and then makes sure that it matches the expected token type, also returns the current token (before it's eaten)
    fn expect(&mut self, r#type: TokenType, err: &'static str) -> Token {
        let prev = self.eat();

        if prev.r#type == TokenType::EOF || prev.r#type != r#type {
            panic!("Parser Error: {} - Expecting: {:#?}", err, r#type);
        }

        prev
    }

    pub fn produce_ast(&mut self) -> ast::Program {
        let mut program: ast::Program = ast::Program {
            kind: ast::NodeType::Program,
            body: vec![]
        };

        while self.not_eof() {
            program.body.push(self.parse_stmt());
        }

        program
    }

    fn parse_stmt(&mut self) -> ast::Stmt {
        let tk = self.at().r#type;

        match tk {
            TokenType::Let => {
                self.parse_variable_stmt(false)
            }

            TokenType::Const => {
                self.parse_variable_stmt(true)
            }

            _ => {
                ast::Stmt::Expr(self.parse_expr())
            }
        }
    }

    fn parse_expr(&mut self) -> ast::Expr {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> ast::Expr {
        let left = self.parse_additive_expr();

        if self.at().r#type == TokenType::Equals {
            self.eat(); // advance past the equals sign to get the value of the assignment expr
            let value = self.parse_assignment_expr();
            return ast::Expr::AssignmentExpr(ast::AssignmentExpr {
                kind: ast::NodeType::AssignmentExpr,
                assignee: Box::new(left),
                value: Box::new(value)
            })
        }

        left
    }

    fn parse_additive_expr(&mut self) -> ast::Expr {
        let mut left = self.parse_multiplicative_expr();

        while matches!(self.at().value.as_str(), "+" | "-") {
            let operator = self.eat().value;
            let right = self.parse_multiplicative_expr();
            
            left = ast::Expr::Binary(ast::BinaryExpr {
                kind: ast::NodeType::BinaryExpr,
                left: Box::new(left),
                right: Box::new(right),
                operator
            });
        }

        left
    }

    fn parse_multiplicative_expr(&mut self) -> ast::Expr {
        let mut left = self.parse_primary_expr();

        while matches!(self.at().value.as_str(), "*" | "/" | "%") {
            let operator = self.eat().value;
            let right = self.parse_primary_expr();

            left = ast::Expr::Binary(ast::BinaryExpr {
                kind: ast::NodeType::BinaryExpr,
                left: Box::new(left),
                right: Box::new(right),
                operator
            });
        }

        left
    }

    fn parse_primary_expr(&mut self) -> ast::Expr {
        let tk = self.at().r#type;

        match tk {
            TokenType::Identifier => {
                ast::Expr::Identifier(ast::Identifier { kind: ast::NodeType::Identifier, symbol: self.eat().value })
            }

            TokenType::Number => {
                ast::Expr::NumericLiteral(ast::NumericLiteral { kind: ast::NodeType::NumericLiteral, value: self.eat().value.parse::<i64>().expect("Parser Error: Failed to parse numeric literal.") })
            }

            TokenType::OpenParen => {
                self.eat(); // eat the opening parenthesis
                let value = self.parse_expr();
                self.expect(TokenType::CloseParen, "Unexpected token found inside parenthesised expression, expected closing parenthesis.");
                value
            }

            _ => {
                panic!("Parser Error: Unexpected token found during parsing: {:?}", self.at());
            }
        }
    }

    fn parse_variable_stmt(&mut self, constant: bool) -> ast::Stmt {
        self.eat(); // eat the let keyword
        let ident = self.expect(TokenType::Identifier, "The variable you want to declare must have a proper name!");
        self.expect(TokenType::Equals, "Expected equals sign after identifier.");
        let value = self.parse_expr();

        ast::Stmt::VariableDeclaration(VariableDeclaration {
            kind: ast::NodeType::VariableDeclaration,
            identifier: ident.value,
            constant,
            value
        })
    }
}

