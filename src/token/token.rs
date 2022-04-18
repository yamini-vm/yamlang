#[derive(Debug)]
pub enum Token {
    // Keywords
    PRINT,

    // Operators
    LPAREN,
    RPAREN,

    // Literals
    STRING(String),
    NUM(String),

    // Meta
    EOF,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::PRINT, Token::PRINT) => true,
            (Token::LPAREN, Token::LPAREN) => true,
            (Token::RPAREN, Token::RPAREN) => true,
            (Token::STRING(ref s1), Token::STRING(ref s2)) => s1 == s2,
            (Token::EOF, Token::EOF) => true,
            (Token::NUM(ref s1), Token::NUM(ref s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl Token {
    pub fn value(&self) -> String {
        match self {
            Token::PRINT => "print".to_string(),
            Token::LPAREN => "(".to_string(),
            Token::RPAREN => ")".to_string(),
            Token::STRING(ref s) => s.clone(),
            Token::EOF => "EOF".to_string(),
            Token::NUM(ref s) => s.clone(),
        }
    }
}