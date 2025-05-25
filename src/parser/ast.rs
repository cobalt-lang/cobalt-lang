#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Program,
    VariableDeclaration,
    BinaryExpr,
    Identifier,
    NumericLiteral,
    AssignmentExpr,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Program(Program),
    VariableDeclaration(VariableDeclaration),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Identifier(Identifier),
    NumericLiteral(NumericLiteral),
    AssignmentExpr(AssignmentExpr),
}

// === AST Node Definitions ===

#[derive(Debug, Clone)]
pub struct Program {
    pub kind: NodeType, // Always NodeType::Program
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub kind: NodeType, // Always NodeType::VariableDeclaration
    pub identifier: String,
    pub constant: bool, // whether or not the variable is mutable
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub kind: NodeType, // Always NodeType::BinaryExpr
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: String, // You could also use an enum for operators
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub kind: NodeType, // Always NodeType::Identifier
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub kind: NodeType, // Always NodeType::NumericLiteral
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub kind: NodeType,
    pub assignee: Box<Expr>,
    pub value: Box<Expr>
}