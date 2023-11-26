use crate::repl::start;

pub mod token;
pub mod lexer;
pub mod repl;

fn main() {
    println!("Welcome to the test repl.");
    println!("Enter your code here.");
    start(std::io::stdin(), std::io::stdout()); //TODO: use std::io::{...}?
}
