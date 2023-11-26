use crate::token::{lookup_identifier, Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    curr_position: usize,
    next_position: usize,
    processed_char: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let lexer: Lexer = Lexer {
            input: input.chars().collect(),
            curr_position: 0,
            next_position: 1,
            processed_char: Default::default(),
        };
        lexer
    }

    fn read_char(&mut self) {
        if self.curr_position == self.input.len() {
            self.processed_char = '\0';
        } else {
            self.processed_char = self.input[self.curr_position];
            self.curr_position = self.next_position;
            self.next_position += 1;
        }
    }

    fn peek_char(&self) -> char {
        if self.curr_position == self.input.len() {
            '\0'
        } else {
            self.input[self.curr_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();
        self.skip_whitespace();
        let token = match self.processed_char {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token { t_type: TokenType::Equal, literal: "==".to_string() }
                } else {
                    Token { t_type: TokenType::Assign, literal: self.processed_char.to_string() }
                }
            }
            '+' => Token { t_type: TokenType::Plus, literal: self.processed_char.to_string() },
            '-' => Token { t_type: TokenType::Minus, literal: self.processed_char.to_string() },
            '/' => Token { t_type: TokenType::Slash, literal: self.processed_char.to_string() },
            '*' => Token { t_type: TokenType::Asterisk, literal: self.processed_char.to_string() },
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token { t_type: TokenType::GreaterOrEqual, literal: ">=".to_string() }
                } else {
                    Token { t_type: TokenType::GreaterThan, literal: self.processed_char.to_string() }
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token { t_type: TokenType::LessOrEqual, literal: "<=".to_string() }
                } else {
                    Token { t_type: TokenType::LessThan, literal: self.processed_char.to_string() }
                }
            }
            '(' => Token { t_type: TokenType::OpenParenthesis, literal: self.processed_char.to_string() },
            ')' => Token { t_type: TokenType::CloseParenthesis, literal: self.processed_char.to_string() },
            '{' => Token { t_type: TokenType::OpenBrace, literal: self.processed_char.to_string() },
            '}' => Token { t_type: TokenType::CloseBrace, literal: self.processed_char.to_string() },
            ',' => Token { t_type: TokenType::Comma, literal: self.processed_char.to_string() },
            ';' => Token { t_type: TokenType::Semicolon, literal: self.processed_char.to_string() },
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token { t_type: TokenType::NotEqual, literal: "!=".to_string() }
                } else {
                    Token { t_type: TokenType::Bang, literal: self.processed_char.to_string() }
                }
            }
            '\0' => Token { t_type: TokenType::EndOfFile, literal: "".to_string() },
            _ => {
                return if Lexer::is_letter(self.processed_char) {
                    let literal: String = self.read_identifier();
                    let t_type: TokenType = lookup_identifier(&literal);
                    Token { t_type, literal }
                } else if Lexer::is_digit(self.processed_char) {
                    let literal: String = self.read_number();
                    let t_type: TokenType = if literal.contains(".") {
                        TokenType::Double
                    } else {
                        TokenType::Integer
                    };
                    Token { t_type, literal }
                } else {
                    Token { t_type: TokenType::Illegal, literal: self.processed_char.to_string() }
                };
            }
        };
        token
    }

    fn skip_whitespace(&mut self) {
        while self.processed_char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        identifier.push(self.processed_char);
        while Lexer::is_letter(self.peek_char()) {
            self.read_char();
            identifier.push(self.processed_char);
        }
        identifier
    }

    //TODO:rename
    //TODO: extract '.' and '_'
    fn is_digit(ch: char) -> bool {
        ch.is_numeric() || ch == '.' || ch == '_'
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        number.push(self.processed_char);
        while Lexer::is_digit(self.peek_char()) {
            self.read_char();
            number.push(self.processed_char);
        }
        // if number.ends_with(".") || number.ends_with("_") {
        //TODO: verify pattern -> slice of chars vs array?
        if number.ends_with(['.', '_']) {
            //TODO: handle panic and add unit test
            panic!("invalid input {number}");
        }
        number
    }
}

//////////////////// Tests //////////////////////

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_next_token_basic_input() {
        let input: &str = "=+-<>(){},;!/*";

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
                t_type: TokenType::LessThan,
                literal: "<".to_string(),
            },
            Token {
                t_type: TokenType::GreaterThan,
                literal: ">".to_string(),
            },
            Token {
                t_type: TokenType::OpenParenthesis,
                literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::CloseParenthesis,
                literal: ")".to_string(),
            },
            Token {
                t_type: TokenType::OpenBrace,
                literal: "{".to_string(),
            },
            Token {
                t_type: TokenType::CloseBrace,
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
                t_type: TokenType::Bang,
                literal: "!".to_string(),
            },
            Token {
                t_type: TokenType::Slash,
                literal: "/".to_string(),
            },
            Token {
                t_type: TokenType::Asterisk,
                literal: "*".to_string(),
            },
            Token {
                t_type: TokenType::EndOfFile,
                literal: "".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);
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

    #[test]
    fn test_next_token_simple_code() {
        //TODO: handle return keyword in func
        let input: &str = r#"
        var first_num = 3_000_000;
        var second_num = 5.1;
        var add = func(x, y) {
           return x + y;
        };
        var result = add(first_num, second_num);
        "#;

        let expected: Vec<Token> = vec![
            //first line
            Token {
                t_type: TokenType::Variable,
                literal: "var".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "first_num".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Integer,
                literal: "3_000_000".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //second line
            Token {
                t_type: TokenType::Variable,
                literal: "var".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "second_num".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Double,
                literal: "5.1".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //third line
            Token {
                t_type: TokenType::Variable,
                literal: "var".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "add".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Function,
                literal: "func".to_string(),
            },
            Token {
                t_type: TokenType::OpenParenthesis,
                literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "y".to_string(),
            },
            Token {
                t_type: TokenType::CloseParenthesis,
                literal: ")".to_string(),
            },
            Token {
                t_type: TokenType::OpenBrace,
                literal: "{".to_string(),
            },
            //fourth line
            Token {
                t_type: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::Plus,
                literal: "+".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "y".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //fifth line
            Token {
                t_type: TokenType::CloseBrace,
                literal: "}".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //sixth line
            Token {
                t_type: TokenType::Variable,
                literal: "var".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "result".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "add".to_string(),
            },
            Token {
                t_type: TokenType::OpenParenthesis,
                literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "first_num".to_string(),
            },
            Token {
                t_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "second_num".to_string(),
            },
            Token {
                t_type: TokenType::CloseParenthesis,
                literal: ")".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);
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

    #[test]
    fn test_next_token_conditionals() {
        let input: &str = r#"
        var a = 4;
        var b = 7;
        if (a == b) {
            return true;
        } else {
            return false;
        }
        "#;

        let expected: Vec<Token> = vec![
            Token {
                t_type: TokenType::Variable,
                literal: "var".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "a".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Integer,
                literal: "4".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //second line
            Token {
                t_type: TokenType::Variable,
                literal: "var".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "b".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Integer,
                literal: "7".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //third line
            Token {
                t_type: TokenType::If,
                literal: "if".to_string(),
            },
            Token {
                t_type: TokenType::OpenParenthesis,
                literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "a".to_string(),
            },
            Token {
                t_type: TokenType::Equal,
                literal: "==".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "b".to_string(),
            },
            Token {
                t_type: TokenType::CloseParenthesis,
                literal: ")".to_string(),
            },
            Token {
                t_type: TokenType::OpenBrace,
                literal: "{".to_string(),
            },
            //fourth line
            Token {
                t_type: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                t_type: TokenType::True,
                literal: "true".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //fifth line
            Token {
                t_type: TokenType::CloseBrace,
                literal: "}".to_string(),
            },
            Token {
                t_type: TokenType::Else,
                literal: "else".to_string(),
            },
            Token {
                t_type: TokenType::OpenBrace,
                literal: "{".to_string(),
            },
            //sixth line
            Token {
                t_type: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                t_type: TokenType::False,
                literal: "false".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //seventh line
            Token {
                t_type: TokenType::CloseBrace,
                literal: "}".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);
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

    #[test]
    fn test_next_token_comparisons() {
        let input: &str = r#"
        x == y;
        x != y;
        x >= y;
        x <= y;
        "#;

        let expected: Vec<Token> = vec![
            //first line
            Token {
                t_type: TokenType::Identifier,
                literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::Equal,
                literal: "==".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "y".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //second line
            Token {
                t_type: TokenType::Identifier,
                literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::NotEqual,
                literal: "!=".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "y".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //third line
            Token {
                t_type: TokenType::Identifier,
                literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::GreaterOrEqual,
                literal: ">=".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "y".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            //fourth line
            Token {
                t_type: TokenType::Identifier,
                literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::LessOrEqual,
                literal: "<=".to_string(),
            },
            Token {
                t_type: TokenType::Identifier,
                literal: "y".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);
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