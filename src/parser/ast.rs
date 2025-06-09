#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Program,
    VariableDeclaration,
    IfStatement,
    BlockStatement,
    BinaryExpr,
    Identifier,
    NumericLiteral,
    BooleanLiteral,
    AssignmentExpr,
    UnaryExpr,
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
    pub value: Box<Expr>
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub kind: NodeType,
    pub operator: String, // +, -, ! (for if statements)
    pub value: Box<Expr>
}
