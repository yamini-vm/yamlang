use crate::node::ExpressionNode;
use crate::token::Token;
use crate::node::{Node, ProgramNode, StatementNode, PrintNode, StringNode};

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

    fn expression(&mut self) -> impl ExpressionNode {
        self.expect(Token::LPAREN);

        let string_node = StringNode {
            value: self.next().value(),
        };

        self.expect(Token::RPAREN);

        string_node
    }

    fn statement(&mut self) -> Box<dyn StatementNode> {
        if *self.peek() == Token::PRINT {
            self.next(); // Skip PRINT
            let print_node = PrintNode {
                expression: Box::new(self.expression()),
            };

            Box::new(print_node)
        } else {
            panic!("Unexpected token {:?}", self.peek());
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