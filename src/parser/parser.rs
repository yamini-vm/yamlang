use crate::node::ExpressionNode;
use crate::token::Token;
use crate::node::{Node, ProgramNode, StatementNode, PrintNode, StringNode, NumberNode, VarNode};
use crate::node::{IdentifierNode};

pub struct Parser {
    tokens: Vec<Token>,
    current_token_ptr: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current_token_ptr: 0,
        }
    }

    fn is_end(&self) -> bool {
        self.tokens[self.current_token_ptr] == Token::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current_token_ptr]
    }

    fn next(&mut self) -> &Token {
        let token = &self.tokens[self.current_token_ptr];
        self.current_token_ptr += 1;
        token
    }

    fn expect(&mut self, expected_token: Token) {
        if *self.peek() != expected_token {
            panic!("Expected token {:?} but got {:?}", expected_token, self.peek());
        }
        self.next();
    }

    fn expression(&mut self) -> Box<dyn ExpressionNode> {
        match *self.peek() {
            Token::STRING(ref s) => {
                let string_node = StringNode {
                    value: s.clone(),
                };
                Box::new(string_node)
            },
            Token::NUM(ref s) => {
                let number_node = NumberNode {
                    value: s.clone(),
                };
                Box::new(number_node)
            },
            Token::IDENTIFIER(ref s) => {
                let identifier_node = IdentifierNode {
                    name: s.clone(),
                };
                Box::new(identifier_node)
            },
            _ => {
                panic!("Expected STRING or NUM but got {:?}", self.peek());
            }
        }
    }

    fn statement(&mut self) -> Box<dyn StatementNode> {
        match &*self.peek() {
            Token::PRINT => {
                self.next(); // Skip PRINT
                self.expect(Token::LPAREN);

                let print_node = PrintNode {
                    expression: self.expression(),
                };
                self.next(); // Skip expression
                self.expect(Token::RPAREN);

                Box::new(print_node)
            },
            Token::VAR => {
                self.next(); // Skip VAR

                let identifier = self.next().value();
                self.expect(Token::ASSIGN);

                let var_node = VarNode {
                    identifier: identifier,
                    expression: self.expression(),
                };
                self.next(); // Skip expression

                Box::new(var_node)
            },
            Token::IDENTIFIER(ident) => {
                let identifier = ident.clone();
                self.next(); // Skip IDENTIFIER
                self.expect(Token::ASSIGN);

                let identifier_node =  VarNode {
                    identifier: identifier,
                    expression: self.expression(),
                };
                self.next(); // Skip expression

                Box::new(identifier_node)
            },
            _ => {
                panic!("Unexpected token {:?}", self.peek());
            }
        }
    }

    fn program(&mut self) -> ProgramNode {
        let mut statement_nodes = Vec::new();

        while !self.is_end() {
            statement_nodes.push(self.statement());
        }

        ProgramNode {
            function: "<main>".to_string(),
            statement_nodes,
        }
    }

    pub fn parse(&mut self) -> impl Node {
        let program_node = self.program();
        program_node
    }
}