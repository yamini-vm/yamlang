use crate::node::Node;
use crate::node::StatementNode;
use crate::node::ExpressionNode;

pub struct VarNode {
    pub(crate) identifier: String,
    pub(crate) expression: Box<dyn ExpressionNode>,
}

impl StatementNode for VarNode {
    fn print_statement(&self, indent: usize) {
        println!("{}VarNode", "  ".repeat(indent));
        println!("{}Identifier: {}", "  ".repeat(indent + 1), self.identifier);
        self.expression.print(indent + 1);
    }

    fn compile_statement(&self, program_str:&mut String) {
        self.expression.compile(program_str);
        program_str.push_str(&format!("POP &{}\n", self.identifier));
    }
}