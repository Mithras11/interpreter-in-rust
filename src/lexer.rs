use crate::token::{Token, TokenKind};

struct Lexer {
    input: Vec<char>,
    curr_position: usize,
    next_position: usize,
    curr_char: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let lexer: Lexer = Lexer {
            input: input.chars().collect(),
            curr_position: 0,
            next_position: 1,
            curr_char: Default::default(),
        };
        lexer
    }

    fn read_char(&mut self) {
        if self.curr_position == self.input.len() {
            self.curr_char = '\0';
        } else {
            self.curr_char = self.input[self.curr_position];
            self.curr_position = self.next_position;
            self.next_position += 1;
        }
    }

    fn next_token(&mut self) -> Token {
        self.read_char();
        let token = match self.curr_char {
            '=' => Token::new(TokenKind::Assign, self.curr_char),
            '+' => Token::new(TokenKind::Plus, self.curr_char),
            '-' => Token::new(TokenKind::Minus, self.curr_char),
            '(' => Token::new(TokenKind::Lparen, self.curr_char),
            ')' => Token::new(TokenKind::Rparen, self.curr_char),
            '{' => Token::new(TokenKind::Lbrace, self.curr_char),
            '}' => Token::new(TokenKind::Rbrace, self.curr_char),
            ',' => Token::new(TokenKind::Comma, self.curr_char),
            ';' => Token::new(TokenKind::Semicolon, self.curr_char),
            '\0' => Token::new(TokenKind::EOF, self.curr_char),
            _ => Token::new(TokenKind::Illegal, self.curr_char),
        };
        token
    }
}

//////////////////// Tests //////////////////////

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_next_token() {
        let input: &str = "=+-(){},;";

        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(), //TODO: check if String::from shou;d be used instead
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::Minus,
                literal: "-".to_string(),
            },
            Token {
                kind: TokenKind::Lparen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Rparen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::EOF,
                literal: "\0".to_string(),
            },
        ];

        let mut lexer: Lexer = Lexer::new(input);

        for (_, expected_token) in expected.into_iter().enumerate() {
            let received_token = lexer.next_token();
            assert_eq!(expected_token.kind, received_token.kind,
                       "tests[(idx)] - token type is wrong, expected={}, got={}",
                       expected_token.kind, received_token.kind
            );
            assert_eq!(expected_token.literal, received_token.literal,
                       "tests[(idx)] - token literal is wrong, expected={}, got={}",
                       expected_token.literal, received_token.literal
            );
        }
    }
}