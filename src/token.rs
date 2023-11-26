#[derive(PartialEq, Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub literal: String, //TODO: use &'static str?
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Illegal,
    EOF,

    Identifier,
    Integer,

    // math operators
    Assign,
    Plus,
    Minus,
    //comparisons TODO: rename
    GT,
    LT,
    EQ,
    NE,
    GE,
    LE,

    Comma,
    Semicolon,
    Bang,
    Asterisk,
    Slash,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

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
            TokenType::EOF => write!(f, "EOF"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Integer => write!(f, "Integer"),
            TokenType::Assign => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::GT => write!(f, ">"),
            TokenType::LT => write!(f, "<"),
            TokenType::EQ => write!(f, "=="),
            TokenType::NE => write!(f, "!="),
            TokenType::GE => write!(f, ">="),
            TokenType::LE => write!(f, "<="),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Lparen => write!(f, "("),
            TokenType::Rparen => write!(f, ")"),
            TokenType::Lbrace => write!(f, "{{"),
            TokenType::Rbrace => write!(f, "}}"),
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