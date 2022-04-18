use crate::node::Node;

pub trait ExpressionNode {
    fn print_expression(&self, indent: usize);

    fn compile_expression(&self, program_str: &mut String);
}

impl Node for dyn ExpressionNode {
    fn print(&self, indent: usize) {
        self.print_expression(indent);
    }

    fn compile(&self, program_str: &mut String) {
        self.compile_expression(program_str);
    }
}