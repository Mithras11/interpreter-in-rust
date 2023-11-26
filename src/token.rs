#[derive(PartialEq, Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub literal: String, //TODO: use &'static str?
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Illegal,
    EndOfFile,

    Identifier,
    Integer,
    Double,

    // math operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    //comparisons TODO: rename
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterOrEqual,
    LessOrEqual,

    Comma,
    Semicolon,
    Bang,

    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,

    // keywords
    Function,
    Variable,
    True,
    False,
    If,
    Else,
    Return,
}

//TODO: use std::fmt{...}
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Illegal => write!(f, "Illegal"),
            TokenType::EndOfFile => write!(f, "EOF"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Integer => write!(f, "Integer"),
            TokenType::Double => write!(f, "Double"),
            TokenType::Assign => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::GreaterThan => write!(f, ">"),
            TokenType::LessThan => write!(f, "<"),
            TokenType::Equal => write!(f, "=="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::GreaterOrEqual => write!(f, ">="),
            TokenType::LessOrEqual => write!(f, "<="),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Bang => write!(f, "!"),
            TokenType::OpenParenthesis => write!(f, "("),
            TokenType::CloseParenthesis => write!(f, ")"),
            TokenType::OpenBrace => write!(f, "{{"),
            TokenType::CloseBrace => write!(f, "}}"),
            TokenType::Function => write!(f, "Function"),
            TokenType::Variable => write!(f, "Variable"),
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),
            TokenType::If => write!(f, "If"),
            TokenType::Else => write!(f, "Else"),
            TokenType::Return => write!(f, "Return"),
        }
    }
}

pub fn lookup_identifier(identifier: &String) -> TokenType {
    //TODO use static str instead of str?
    match identifier.as_str() {
        "func" => TokenType::Function,
        "var" => TokenType::Variable,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Identifier
    }
}