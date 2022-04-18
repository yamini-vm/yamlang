use crate::node::Node;
use crate::node::StatementNode;
use crate::node::ExpressionNode;

pub struct VarNode {
    pub(crate) identifier: Box<dyn ExpressionNode>,
    pub(crate) expression: Box<dyn ExpressionNode>,
}

impl StatementNode for VarNode {
    fn print_statement(&self, indent: usize) {
        println!("{}VarNode", "  ".repeat(indent));
        self.identifier.print_expression(indent + 2);
        println!("{}Expression: ", "  ".repeat(indent + 1));
        self.expression.print(indent + 2);
    }

    fn compile_statement(&self, program_str:&mut String) {
        self.expression.compile(program_str);
        program_str.push_str("POP ");
        self.identifier.compile_expression(program_str);
    }
}