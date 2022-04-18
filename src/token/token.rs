#[derive(Debug)]
pub enum Token {
    // Keywords
    PRINT,
    VAR,

    // Operators
    LPAREN,
    RPAREN,
    ASSIGN,

    // Literals
    STRING(String),
    NUM(String),
    IDENTIFIER(String),

    // Meta
    EOF,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::PRINT, Token::PRINT) => true,
            (Token::VAR, Token::VAR) => true,
            (Token::LPAREN, Token::LPAREN) => true,
            (Token::RPAREN, Token::RPAREN) => true,
            (Token::ASSIGN, Token::ASSIGN) => true,
            (Token::STRING(_), Token::STRING(_)) => true,
            (Token::NUM(_), Token::NUM(_)) => true,
            (Token::IDENTIFIER(_), Token::IDENTIFIER(_)) => true,
            (Token::EOF, Token::EOF) => true,
            _ => false,
        }
    }
}

impl Token {
    pub fn value(&self) -> String {
        match self {
            Token::PRINT => "print".to_string(),
            Token::VAR => "var".to_string(),
            Token::LPAREN => "(".to_string(),
            Token::RPAREN => ")".to_string(),
            Token::ASSIGN => "=".to_string(),
            Token::STRING(ref s) => s.clone(),
            Token::NUM(ref s) => s.clone(),
            Token::IDENTIFIER(ref s) => s.clone(),
            Token::EOF => "EOF".to_string(),
        }
    }

    pub fn from(value: &str) -> Token {
        match value {
            "print" => Token::PRINT,
            "var" => Token::VAR,
            _ => panic!("Unknown token {}", value),
        }
    }
}