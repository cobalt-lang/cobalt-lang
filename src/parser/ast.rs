#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Program, // the node that contains the AST
    VariableDeclaration, // let x = 42, const x = 42 for immutable vars
    IfStatement, // if true {} else if x {} else {}, it checks the condition and if evaluated to true executes the statement following it.
    BlockStatement, // { body }, blocks have their own scope
    BinaryExpr, // an expression which has a left and right hand side seperated by an operator that determines the operation
    Identifier, // also a type of literal
    NumericLiteral, // 123
    FloatLiteral, // 123.0
    BooleanLiteral, // true / false
    StringLiteral, // "content here"
    AssignmentExpr, // x = 42
    UnaryExpr, // -42, !true
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Program(Program),
    VariableDeclaration(VariableDeclaration),
    IfStatement(IfStatement),
    BlockStatement(BlockStatement),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Identifier(Identifier),
    NumericLiteral(NumericLiteral),
    BooleanLiteral(BooleanLiteral),
    AssignmentExpr(AssignmentExpr),
    UnaryExpr(UnaryExpr),
}

// === AST Node Definitions ===

#[derive(Debug, Clone)]
pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub kind: NodeType,
    pub identifier: String,
    pub constant: bool,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub kind: NodeType,
    pub test: Expr,
    pub alternate: Option<Box<Stmt>>,
    pub body: Box<Stmt> // either a single statement or a block statement are most common
}

// { code in here }
#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub kind: NodeType,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub kind: NodeType,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: String,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub kind: NodeType,
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub kind: NodeType,
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub kind: NodeType,
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub kind: NodeType,
    pub assignee: Box<Expr>,
    pub operator: String, // =, +=, -=, /=, *=, %=
    pub value: Box<Expr>
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub kind: NodeType,
    pub operator: String, // +, -, ! (for if statements)
    pub value: Box<Expr>
}
