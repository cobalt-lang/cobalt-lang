#[derive(Debug, Clone)]
pub enum TokenType {
    // TYPES
    Number,
    Identifier, // mycoolvarorfunctionname
	String, // "my cool string"

    // KEYWORDS
    Let, // let myvar = 1 (mutable)
	Const, // const myvar = 1 (immutable)
	Fn, // fn mycoolfn(arg: string) {}
    Return, // return mycoolvalue

    // Operators and Symbols
    BinaryOperator, // + - * / %
    ComparisonOperator, // < > == <= >= !=
    Equals, // =
    OpenParen, // (
    CloseParen, // )
    EOF // pushed at end of tokenization
}

pub struct Token {
    pub value: String,
    pub r#type: TokenType
}