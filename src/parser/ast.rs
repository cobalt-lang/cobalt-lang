#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Program,
    BinaryExpr,
    Identifier,
    NumericLiteral,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Program(Program),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Identifier(Identifier),
    NumericLiteral(NumericLiteral),
}

// === AST Node Definitions ===

#[derive(Debug, Clone)]
pub struct Program {
    pub kind: NodeType, // Always NodeType::Program
    pub body: Vec<Stmt>,
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
