#[derive(Debug)]
pub enum Token {
    // Keywords
    PRINT,

    // Operators
    LPAREN,
    RPAREN,

    // Literals
    STRING(String),
}