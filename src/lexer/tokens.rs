#[derive(Debug, Clone, PartialEq, Copy)]
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
    If,
    Else,
    True,
    False,

    // Operators and Symbols
    Colon, // :  
    BinaryOperator, // + - * / %
    ComparisonOperator, // < > == <= >= !=
    Equals, // =
    OpenParen, // (
    CloseParen, // )
    OpenBrace, // {
    CloseBrace, // }
    EOF, // pushed at end of tokenization
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub r#type: TokenType
}