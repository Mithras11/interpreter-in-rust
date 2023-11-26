use std::io::{Stdin, Stdout, Write};
use crate::lexer::Lexer;
use crate::token::TokenType;

pub fn start(stdin: Stdin, mut stdout: Stdout) {
    loop {
        write!(stdout, ">> ").expect("unable to display prompt string '>>'");
        stdout.flush().expect("unable to flush stdout");

        let mut input = String::new();
        if let Err(e) = stdin.read_line(&mut input) {
            writeln!(stdout, "Error: {e}").expect("unable to display the error message");
            return;
        }

        let mut lexer = Lexer::new(input.as_str());
        loop {
            let token = lexer.next_token();
            if token.t_type == TokenType::EndOfFile {
                break;
            }
            writeln!(stdout, "{token:?}").expect("unable to display token");
        }
    }
}