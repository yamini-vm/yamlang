use crate::node::Node;
use crate::node::StatementNode;
use crate::node::ExpressionNode;

pub struct PrintNode {
    pub(crate) expression: Box<dyn ExpressionNode>,
}

impl StatementNode for PrintNode {
    fn print_statement(&self, indent: usize) {
        println!("{}PrintNode", "  ".repeat(indent));
        self.expression.print(indent + 1);
    }

    fn compile_statement(&self, program_str:&mut String) {
        self.expression.compile(program_str);
        program_str.push_str("SHOW\n");
    }
}