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

    // TYPES FOR VARIABLES (reserved because static typing is a strong feature of cobalt)
    // All types are 64-bit, 32-bit is a thing of the past
    TypeInt,
    TypeFloat,
    TypeBool,
    TypeStr,

    // Operators and Symbols
    Colon, // :  
    BinaryOperator, // + - * / % < > == <= >= !=
    ComparisonOperator, // || &&
    Equals, // =
    Not, // ! (when found alone by itself, meant for unary expressions)
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