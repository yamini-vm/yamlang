use crate::token::Token;

pub struct Lexer {
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
        }
    }

    pub fn tokenize(&self, file_path: &str) -> Vec<Token> {
        Vec::new()
    }
}