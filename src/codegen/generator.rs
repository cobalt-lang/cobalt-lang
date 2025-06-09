// convert AST into bytecode
// have a sum of vars (to use STORE 0, then STORE 1, etc.)
// a hashmap that maps variable names to their memory stack numbers

use std::collections::HashMap;
use std::process;
use crate::interpreter::constants;
use crate::parser::ast;

pub struct Codegen {
    bytecode: Vec<u8>,
    variables: HashMap<String, Variable>,
    next_var_id: usize, // used to map variable names (in AST) to their IDs (in bytecode, which doesn't support string names)
    labels: HashMap<String, usize>, // used to map functions to their IP (instruction pointer), aka the byte they start at
}

pub struct Variable {
    pub constant: bool,
    pub id: usize
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
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

        assert!(length <= u8::MAX as usize, "Generator Error: String too long to encode length in one byte");

        let mut result = Vec::with_capacity(1 + length);
        result.push(length as u8);
        result.extend_from_slice(bytes);

        result
    }

    fn set_var(&mut self, ident: &String, constant: bool) {
        // make sure the variable doesn't exist first (takes up unnecessary memory)
        if self.variables.contains_key(ident) {
            eprintln!("Generator Error: Attempted to declare variable {}, which already exists! Try reassigning the variable using {} = <insert value here>", ident, ident);
            process::exit(1);
        }

        self.variables.insert(ident.to_string(), Variable {
            constant,
            id: self.next_var_id
        });
        self.next_var_id += 1;
    }

    fn get_var(&mut self, ident: &String) -> &Variable {
        self.variables.get(ident).unwrap_or_else(|| {
            eprintln!("Generator Error: Could not find value {}, it does not exist in the current scope.", ident);
            process::exit(1);
        })
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

    fn generate_assignment_expr(&mut self, assignmentexpr: &ast::AssignmentExpr) {
        // make sure the assignee is an identifier (only one currently supported)
        let assignee = &*assignmentexpr.assignee;
        let ident: String;

        match assignee {
            ast::Expr::Identifier(identifier) => { ident = (*identifier.symbol).to_string() },
            _ => {
                eprintln!("Generator Error: The left hand side of the assignment expression was not an identifier.");
                process::exit(1);
            }
        }
        // make sure the variable that the assignee is referring to exists
        let var = self.get_var(&ident);
        let var_id = var.id as u64;

        // make sure that the variable is not constant

        if var.constant {
            eprintln!("Generator Error: Attempted to assign to constant '{}', which is immutable.", ident);
            process::exit(1);
        }

        self.generate_expr(&assignmentexpr.value);
        self.bytecode.push(constants::STORE);
        self.bytecode.extend(self.emit_u64(var_id));
    }

    fn generate_expr(&mut self, expr: &ast::Expr) {
        match expr {
            ast::Expr::Binary(binary_expr) => self.generate_binary_expr(binary_expr),
            ast::Expr::Identifier(identifier) => {
                // check if variable exists + get its id if it does
                let val = self.get_var(&identifier.symbol);
                let val_u64: u64 = val.id as u64;
                
                self.bytecode.push(constants::LOAD);
                self.bytecode.extend(self.emit_u64(val_u64));
            }
            ast::Expr::NumericLiteral(literal) => {
                let val_u64: u64 = literal.value as u64;

                self.bytecode.push(constants::PUSH_INT);
                self.bytecode.extend(self.emit_u64(val_u64));
            }
            ast::Expr::UnaryExpr(unary_expr) => {
                // push bytecode depending on operator
                // TODO: Add ! support when there is if statements

                match unary_expr.operator.as_str() {
                    "+" => {
                        // just generate the expression normally
                        self.generate_expr(&unary_expr.value);
                    }
                    "-" => {
                        // generate the expression
                        self.generate_expr(&unary_expr.value);
                        // add neg opcode to make the result a negative value
                        self.bytecode.push(constants::NEG);
                    }
                    _ => {
                        eprintln!("Generator Error: Unexpected operator for unary expression.\nAllowed operators are: +, -\nOperator used was: {}", unary_expr.operator);
                        process::exit(1);
                    }
                }
            }
            ast::Expr::AssignmentExpr(assignment_expr) => self.generate_assignment_expr(assignment_expr),
            ast::Expr::BooleanLiteral(literal) => {
                self.bytecode.push(constants::PUSH_INT);
                if literal.value {
                    self.bytecode.extend(self.emit_u64(1));
                } else {
                    self.bytecode.extend(self.emit_u64(0));
                }
            }
        }
    }

    fn generate_stmt(&mut self, stmt: &ast::Stmt) {
        match stmt {
            ast::Stmt::VariableDeclaration(vardecl) => self.generate_vardecl_stmt(vardecl),
            ast::Stmt::Expr(expr) => self.generate_expr(expr),
            ast::Stmt::Program(_) => {
                eprintln!("Generator Error: There is a program within the program, this is not allowed!");
                process::exit(1);
            }
            _ => todo!(),
        }
    }

    fn generate_vardecl_stmt(&mut self, vardecl: &ast::VariableDeclaration) {
        // PUSH_INT/PUSH_STR <whatever the variable value is>
        // STORE <next available ID>

        // generate the variable's value
        self.generate_expr(&vardecl.value);
        // set the variable in the bytecode - STORE <next available ID>
        self.bytecode.push(constants::STORE);
        self.bytecode.extend(self.emit_u64(self.next_var_id as u64));
        // set the variable in the generator so the ID isn't repeated
        self.set_var(&vardecl.identifier, vardecl.constant);
    }

    /// Generate a bytecode array (that can be written to bytecode files and interpreted) based off the parser's produced AST.
    /// 
    /// `ast`: A vector of statements
    /// 
    /// `scope`: The scope to push variables in (used to set the scope to "local" when generating function bodies recursively.) 
    pub fn generate(&mut self, ast: Vec<ast::Stmt>, scope: &str) -> &Vec<u8> {
        if scope != "global" && scope != "local" { panic!("Generator Error: Generate function was given an incorrect scope! Only options are global or local, the scope that was given was {}.", scope)}

        // push magic number
        self.bytecode.extend(constants::MAGIC_NUMBER_U8);

        // iterate through statements and expressions and turn them into operations
        for stmt in ast {
            self.generate_stmt(&stmt);
        }

        self.bytecode.push(constants::HALT);
        &self.bytecode
    }
} 