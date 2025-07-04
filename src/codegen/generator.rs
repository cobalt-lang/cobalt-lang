// convert AST into bytecode
// have a sum of vars (to use STORE 0, then STORE 1, etc.)
// a hashmap that maps variable names to their memory stack numbers

use std::collections::HashMap;
use std::process;
use crate::interpreter::constants;
use crate::parser::ast;

pub struct Codegen {
    bytecode: Vec<u8>,
    scopes: Vec<HashMap<String, Variable>>,
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
            scopes: vec![HashMap::new()],
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
        let current_scope = self.scopes.last_mut().unwrap();
        
        // make sure the variable doesn't exist first (takes up unnecessary memory)
        if current_scope.contains_key(ident) {
            eprintln!("Generator Error: Variable '{}' already exists in the current scope.", ident);
            process::exit(1);
        }
        
        current_scope.insert(ident.clone(), Variable { 
            constant,
            id: self.next_var_id
        });
        
        self.next_var_id += 1;
    }

    fn get_var(&mut self, ident: &String) -> &Variable {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(ident) {
                return var;
            }
        }
        
        eprintln!("Generator Error: Variable '{}' does not exist in any accessible scope.", ident);
        process::exit(1);
    }

    fn generate_operator(&mut self, operator: &str) {
        match operator {
            "+" | "+=" => {
                self.bytecode.push(constants::ADD);
            }
            "-" | "-=" => {
                self.bytecode.push(constants::SUB);
            }
            "*" | "*=" => {
                self.bytecode.push(constants::MUL);
            }
            "/" | "/=" => {
                self.bytecode.push(constants::DIV);
            }
            "%" | "%=" => {
                self.bytecode.push(constants::MOD);
            }
            "=" => { /* do nothing */ }
            "==" => {
                self.bytecode.push(constants::EQ);
            }
            "!=" => {
                self.bytecode.push(constants::NEQ);
            }
            ">" => {
                self.bytecode.push(constants::GT);
            }
            "<" => {
                self.bytecode.push(constants::LT);
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

    fn generate_logical_expr(&mut self, logical_expr: &ast::LogicalExpr) {
        // generate the left side, will either push true or false to the stack once evaluated
        self.generate_expr(&logical_expr.left);
        
        match logical_expr.operator.as_str() {
            "||" => {
                // if the left side is true, skip evaluating the right and keep true on the stack
                self.bytecode.push(constants::JMP_IF_TRUE_PEEK);
                let left_pos = self.bytecode.len();
                self.bytecode.extend(self.emit_u64(0)); // placeholder bytes
                
                // generate the right side
                self.generate_expr(&logical_expr.right);
                // the right side is generated and evaluated, its result stays on the stack.
                
                let after_right_pos = self.bytecode.len();
                self.patch_jump(left_pos, after_right_pos);
            }
            "&&" => {
                // if the left side is false, skip evaluating the right and keep false on the stack
                self.bytecode.push(constants::JMP_IF_FALSE_PEEK);
                let left_pos = self.bytecode.len();
                self.bytecode.extend(self.emit_u64(0)); // placeholder bytes
                
                // pop the value from above
                self.bytecode.push(constants::POP);
                // generate the right side
                self.generate_expr(&logical_expr.right);
                // the right side is generated and evaluated, its result stays on the stack.

                let after_right_pos = self.bytecode.len();
                self.patch_jump(left_pos, after_right_pos);
            }
            _ => {
                eprintln!("Generator Error: Invalid logical operator: {}", logical_expr.operator);
                process::exit(1);
            }
        }
    }

    fn generate_assignment_expr(&mut self, assignmentexpr: &ast::AssignmentExpr) {
        // make sure the assignee is an identifier (only one currently supported)

        let assignee = &*assignmentexpr.assignee;
        let ident: String = match assignee {
            ast::Expr::Identifier(identifier) => { (*identifier.symbol).to_string() },
            _ => {
                eprintln!("Generator Error: The left hand side of the assignment expression was not an identifier.");
                process::exit(1);
            }
        };

        // make sure the variable that the assignee is referring to exists
        let var = self.get_var(&ident);
        let var_id = var.id as u64;

        // make sure that the variable is not constant

        if var.constant {
            eprintln!("Generator Error: Attempted to assign to constant '{}', which is immutable.", ident);
            process::exit(1);
        }

        // if the assignment operator is not =, also push the value of the assignee before the assignment
        if assignmentexpr.operator != "=" {
            self.generate_expr(assignee);
        }

        self.generate_expr(&assignmentexpr.value);
        self.generate_operator(&assignmentexpr.operator);
        self.bytecode.push(constants::STORE);
        self.bytecode.extend(self.emit_u64(var_id));
    }

    fn generate_expr(&mut self, expr: &ast::Expr) {
        match expr {
            ast::Expr::Binary(binary_expr) => self.generate_binary_expr(binary_expr),
            ast::Expr::Identifier(identifier) => {
                // check if a variable exists and get its id if it does
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
                // push bytecode depending on the operator

                match unary_expr.operator.as_str() {
                    "+" => {
                        // generate the expression normally
                        self.generate_expr(&unary_expr.value);
                    }
                    "-" => {
                        // generate the expression
                        self.generate_expr(&unary_expr.value);
                        // add neg opcode to make the result a negative value
                        self.bytecode.push(constants::NEG);
                    }
                    "!" => {
                        // generate the expression
                        self.generate_expr(&unary_expr.value);
                        // add not opcode
                        self.bytecode.push(constants::NOT)
                    }
                    _ => {
                        eprintln!("Generator Error: Unexpected operator for unary expression.\nAllowed operators are: +, -\nOperator used was: {}", unary_expr.operator);
                        process::exit(1);
                    }
                }
            }
            ast::Expr::LogicalExpr(logical_expr) => self.generate_logical_expr(logical_expr),
            ast::Expr::AssignmentExpr(assignment_expr) => self.generate_assignment_expr(assignment_expr),
            ast::Expr::BooleanLiteral(literal) => {
                self.bytecode.push(constants::PUSH_BOOL);
                if literal.value {
                    self.bytecode.push(1);
                } else {
                    self.bytecode.push(0);
                }
            }
        }
    }

    // this function needs to exist because the if statement does not know where to jump because that area is not yet generated, once it is, we can patch the placeholder with the real
    fn patch_jump(&mut self, pos: usize, target: usize) {
        let target_bytes = target.to_le_bytes();
        self.bytecode[pos..(pos + 8)].copy_from_slice(&target_bytes);
    }

    fn generate_if_stmt(&mut self, if_stmt: &ast::IfStatement) {
        // generate condition
        self.generate_expr(&if_stmt.test);

        // emit the jmpiffalse opcode
        self.bytecode.push(constants::JMP_IF_FALSE);
        let jmp_if_false_pos = self.bytecode.len(); // address of the jump, either to be after the if statement or to jump towards the next alternate condition
        self.bytecode.extend(self.emit_u64(0)); // placeholder bytes

        // generate code for if statement body
        self.generate_stmt(&if_stmt.body);

        if let Some(alternate) = &if_stmt.alternate {
            self.bytecode.push(constants::JMP);
            let jmp_over_else_pos = self.bytecode.len();
            self.bytecode.extend(self.emit_u64(0));

            // patch the jmpiffalse from earlier to here (start of the alternate block)
            let alternate_start = self.bytecode.len();
            self.patch_jump(jmp_if_false_pos, alternate_start);

            // generate the alternate's stmt
            self.generate_stmt(alternate);

            // patch jmp from earlier to after the else block
            let after_else = self.bytecode.len();
            self.patch_jump(jmp_over_else_pos, after_else);
        } else {
            // this occurs when no alternate is given, it patches jmpiffalse to after the body of the if stmt
            let after_body = self.bytecode.len();
            self.patch_jump(jmp_if_false_pos, after_body);
        }
    }

    fn generate_block_stmt(&mut self, block_stmt: &ast::BlockStatement) {
        self.scopes.push(HashMap::new()); // make a new scope
        
        // generate code for each statement in the block
        for stmt in &block_stmt.body {
            self.generate_stmt(stmt);
        }
        
        self.scopes.pop(); // exit the scope
    }

    fn generate_stmt(&mut self, stmt: &ast::Stmt) {
        match stmt {
            ast::Stmt::VariableDeclaration(vardecl) => self.generate_vardecl_stmt(vardecl),
            ast::Stmt::IfStatement(if_stmt) => self.generate_if_stmt(if_stmt),
            ast::Stmt::BlockStatement(block_stmt) => self.generate_block_stmt(block_stmt),
            ast::Stmt::Expr(expr) => self.generate_expr(expr),
            ast::Stmt::Program(_) => {
                eprintln!("Generator Error: There is a program within the program, this is not allowed!");
                process::exit(1);
            }
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
    pub fn generate(&mut self, ast: Vec<ast::Stmt>) -> &Vec<u8> {
        // push the magic number
        self.bytecode.extend(constants::MAGIC_NUMBER_U8);

        // iterate through statements and expressions and turn them into operations
        for stmt in ast {
            self.generate_stmt(&stmt);
        }

        self.bytecode.push(constants::HALT);
        &self.bytecode
    }
} 