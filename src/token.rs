#[derive(PartialEq, Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub literal: String, //TODO: use &'static str?
}

impl Token {
    pub fn new(t_type: TokenType, literal: String) -> Token {
        Token { t_type, literal }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Illegal,
    EOF,

    Identifier,
    Integer,

    Assign,
    Plus,
    Minus,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Variable,
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
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Lparen => write!(f, "("),
            TokenType::Rparen => write!(f, ")"),
            TokenType::Lbrace => write!(f, "{{"),
            TokenType::Rbrace => write!(f, "}}"),
            TokenType::Function => write!(f, "Function"),
            TokenType::Variable => write!(f, "Variable"),
        }
    }
}

pub fn lookup_identifier(identifier: &String) -> TokenType {
    //TODO use static str instead of str?
    match identifier.as_str() {
        "func" => TokenType::Function,
        "var" => TokenType::Variable,
        _ => TokenType::Identifier
    }
}