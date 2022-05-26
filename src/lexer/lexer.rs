use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::token::Token;

pub struct Lexer {
    current_idx: usize,
    file_chars: Vec<char>,
}

fn read_file_line_by_line(filepath: &str) -> Result<Vec<char>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut file_chars = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                for ch in line.chars() {
                    file_chars.push(ch);
                }
            },
            Err(e) => println!("Error: {}", e),
        }
        file_chars.push('\n');
    }

    Ok(file_chars)
}

const KEYWORDS: [&'static str; 2] = [
    "print",
    "var",
];

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            current_idx: 0,
            file_chars: Vec::new(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_idx < self.file_chars.len() && [' ', '\n', '\t'].contains(&self.file_chars[self.current_idx]) {
            self.current_idx += 1;
        }
    }

    fn get_string(&mut self, start_char: char) -> String {
        let mut string = String::new();
        while self.file_chars[self.current_idx] != start_char {
            string.push(self.file_chars[self.current_idx]);
            self.current_idx += 1;
        }
        self.current_idx += 1;

        string
    }

    fn get_number(&mut self) -> String {
        let mut number = String::new();
        while self.file_chars[self.current_idx].is_numeric() {
            number.push(self.file_chars[self.current_idx]);
            self.current_idx += 1;
        }

        number
    }

    fn get_identifier_or_keyword(&mut self) -> Token {
        let mut identifier = String::new();
        while self.file_chars[self.current_idx].is_alphanumeric() || self.file_chars[self.current_idx] == '_' {
            identifier.push(self.file_chars[self.current_idx]);
            self.current_idx += 1;
        }

        let token;
        if KEYWORDS.contains(&identifier.as_str()) {
            token = Token::from(identifier.as_str());
        } else {
            token = Token::IDENTIFIER(identifier);
        } 

        token
    }

    pub fn tokenize(&mut self, file_path: &str) -> Vec<Token> {
        self.file_chars = read_file_line_by_line(file_path).unwrap();
        let mut tokens = Vec::new();

        while self.current_idx < self.file_chars.len() {
            match self.file_chars[self.current_idx] {
                'p' => {
                    self.current_idx += 5;
                    tokens.push(Token::PRINT);
                },
                '\n' | ' ' | '\t' => {
                    self.skip_whitespace();
                },
                '(' => {
                    self.current_idx += 1;
                    tokens.push(Token::LPAREN);
                },
                ')' => {
                    self.current_idx += 1;
                    tokens.push(Token::RPAREN);
                },
                '=' => {
                    self.current_idx += 1;
                    tokens.push(Token::ASSIGN);
                },
                '+' => {
                    self.current_idx += 1;
                    tokens.push(Token::PLUS);
                },
                '-' => {
                    self.current_idx += 1;
                    tokens.push(Token::MINUS);
                },
                '*' => {
                    self.current_idx += 1;
                    tokens.push(Token::STAR);
                },
                '/' => {
                    self.current_idx += 1;
                    tokens.push(Token::SLASH);
                },
                '\"' | '\'' => {
                    self.current_idx += 1;
                    tokens.push(Token::STRING(self.get_string(self.file_chars[self.current_idx - 1])));
                },
                '0'..='9' => {
                    tokens.push(Token::NUM(self.get_number()))
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let token = self.get_identifier_or_keyword();
                    tokens.push(token);
                }
                _ => {
                    panic!("Illegal token {}", self.file_chars[self.current_idx]);
                }
            }
        }

        tokens.push(Token::EOF);

        tokens
    }
}