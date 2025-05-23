// convert AST into bytecode
// have a sum of vars (to use STORE 0, then STORE 1, etc.)
// a hashmap that maps variable names to their memory stack numbers

use std::collections::HashMap;
use crate::interpreter::constants;
use crate::parser::ast;

pub struct Codegen {
    bytecode: Vec<u8>,
    variables: HashMap<String, usize>,
    next_var_id: usize, // used to map variable names (in AST) to their IDs (in bytecode, which doesn't support string names)
    labels: HashMap<String, usize>, // used to map functions to their IP (instruction pointer), aka the byte they start at
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            variables: HashMap::new(),
            next_var_id: 0,
            labels: HashMap::new()
        }
    }

    fn emit_u64(&self, value: u64) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    // emits it in the form PUSH_STR expects, which is <the amount of bytes that are string> <the string bytes>
    fn emit_str(&self, value: &str) -> Vec<u8> {
        let bytes = value.as_bytes();
        let length = bytes.len();

        assert!(length <= u8::MAX as usize, "String too long to encode length in one byte");

        let mut result = Vec::with_capacity(1 + length);
        result.push(length as u8);
        result.extend_from_slice(bytes);

        result
    }

    fn set_var(&mut self, ident: String) {
        self.variables.insert(ident, self.next_var_id);
        self.next_var_id += 1;
    }

    fn generate_operator(&mut self, operator: &str) {
        match operator {
            "+" => {
                self.bytecode.push(constants::ADD);
            }
            "-" => {
                self.bytecode.push(constants::SUB);
            }
            "*" => {
                self.bytecode.push(constants::MUL);
            }
            "/" => {
                self.bytecode.push(constants::DIV);
            }
            "%" => {
                self.bytecode.push(constants::MOD)
            }
            _ => {
                // Handle other operators or throw an error if the operator is not supported
                panic!("Generator Error: Invalid operator: {}", operator);
            }
        }
    }

    fn generate_binary_expr(&mut self, binaryexpr: &ast::BinaryExpr) {
        self.generate_expr(&binaryexpr.left);
        self.generate_expr(&binaryexpr.right);
        self.generate_operator(&binaryexpr.operator);
    }

    fn generate_expr(&mut self, expr: &ast::Expr) {
        match expr {
            ast::Expr::Binary(binary_expr) => {
                self.generate_binary_expr(binary_expr);
            }
            ast::Expr::Identifier(identifier) => {
                // check if variable exists + get its id if it does
                let val = self.variables.get(&identifier.symbol).expect(&format!("Generator Error: Variable \"{}\" doesn't exist!", identifier.symbol));
                let val_u64: u64 = *val as u64;
                
                self.bytecode.push(constants::LOAD);
                self.bytecode.extend(self.emit_u64(val_u64));
            }
            ast::Expr::NumericLiteral(literal) => {
                let val_u64: u64 = literal.value as u64;

                self.bytecode.push(constants::PUSH_INT);
                self.bytecode.extend(self.emit_u64(val_u64));
            }
        }
    }

    /// Generate a bytecode array (that can be written to bytecode files and interpreted) based off the parser's produced AST.
    /// 
    /// `ast`: A vector of statements
    /// 
    /// `scope`: The scope to push variables in (used to set the scope to "local" when generating function bodies recursively.) 
    pub fn generate(&mut self, ast: Vec<ast::Stmt>, scope: &str) -> Vec<u8> {
        if scope != "global" && scope != "local" { panic!("Generator: generate function was given an incorrect scope! Only options are global or local, the scope you gave was {}.", scope)}

        // push magic number
        self.bytecode.extend(constants::MAGIC_NUMBER_U8);

        // iterate through statements and expressions and turn them into operations
        for stmt in ast {
            match stmt {
                ast::Stmt::Program(program) => {
                    self.generate(program.body, scope);
                }
                ast::Stmt::Expr(expr) => {
                    match expr {
                        ast::Expr::Binary(binary_expr) => {
                            self.generate_binary_expr(&binary_expr);
                        }
                        _ => todo!()
                    }
                }
            }
        }

        self.bytecode.push(constants::HALT);
        self.bytecode.clone()
    }
} 