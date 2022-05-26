use crate::node::ExpressionNode;

pub struct BinaryOpNode {
    pub(crate) left: Box<dyn ExpressionNode>,
    pub(crate) right: Box<dyn ExpressionNode>,
    pub(crate) op: String,
}

impl ExpressionNode for BinaryOpNode {
    fn print_expression(&self, indent: usize) {
        println!("{}BinaryOpNode", "  ".repeat(indent));
        println!("{}left:", "  ".repeat(indent + 1));
        self.left.print_expression(indent + 2);
        println!("{}right:", "  ".repeat(indent + 1));
        self.right.print_expression(indent + 2);
        println!("{}op: {}", "  ".repeat(indent + 1), self.op);
    }

    fn compile_expression(&self, program_str: &mut String) {
        self.left.compile_expression(program_str);
        self.right.compile_expression(program_str);
        program_str.push_str(&format!("LOAD {}\n", self.op));
    }
}