use crate::token::{Token, TokenType};

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
            '=' => Token::new(TokenType::Assign, self.curr_char.to_string()),
            '+' => Token::new(TokenType::Plus, self.curr_char.to_string()),
            '-' => Token::new(TokenType::Minus, self.curr_char.to_string()),
            '(' => Token::new(TokenType::Lparen, self.curr_char.to_string()),
            ')' => Token::new(TokenType::Rparen, self.curr_char.to_string()),
            '{' => Token::new(TokenType::Lbrace, self.curr_char.to_string()),
            '}' => Token::new(TokenType::Rbrace, self.curr_char.to_string()),
            ',' => Token::new(TokenType::Comma, self.curr_char.to_string()),
            ';' => Token::new(TokenType::Semicolon, self.curr_char.to_string()),
            '\0' => Token::new(TokenType::EOF, "".to_string()),
            _ => Token::new(TokenType::Illegal, self.curr_char.to_string()),
        };
        token
    }
}

//////////////////// Tests //////////////////////

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input: &str = "=+-(){},;";

        let expected: Vec<Token> = vec![
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(), //TODO: check if String::from should be used instead
            },
            Token {
                t_type: TokenType::Plus,
                literal: "+".to_string(),
            },
            Token {
                t_type: TokenType::Minus,
                literal: "-".to_string(),
            },
            Token {
                t_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            Token {
                t_type: TokenType::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                t_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                t_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::EOF,
                literal: "".to_string(),
            },
        ];

        let mut lexer: Lexer = Lexer::new(input);

        for (_, expected_token) in expected.into_iter().enumerate() {
            let received_token = lexer.next_token();
            assert_eq!(expected_token.t_type, received_token.t_type,
                       "tests[(idx)] - token type is wrong, expected={}, got={}",
                       expected_token.t_type, received_token.t_type
            );
            assert_eq!(expected_token.literal, received_token.literal,
                       "tests[(idx)] - token literal is wrong, expected={}, got={}",
                       expected_token.literal, received_token.literal
            );
        }
    }
}