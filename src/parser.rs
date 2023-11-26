use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::Token;

struct Parser {
    lexer: Lexer,
    curr_token: Token,
    next_token: Token,
}

impl Parser {
    fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            curr_token: Default::default(),
            next_token: Default::default(),
        };
        //TODO: refactor
        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn parse_program(&mut self)->Option<Program>{
        None
    }
}