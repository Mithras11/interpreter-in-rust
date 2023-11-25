#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String, //TODO: use &'static str?
}

impl Token {
    pub fn new(kind: TokenKind, ch: char) -> Token {
        Token { kind, literal: ch.to_string() }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
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
    Let,
}

//TODO: use std::fmt{...}
impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Integer => write!(f, "Integer"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Lparen => write!(f, "("),
            TokenKind::Rparen => write!(f, ")"),
            TokenKind::Lbrace => write!(f, "{{"),
            TokenKind::Rbrace => write!(f, "}}"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Let => write!(f, "Let"),
        }
    }
}