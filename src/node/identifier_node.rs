use crate::node::ExpressionNode;

pub struct IdentifierNode {
    pub(crate) name: String,
}

impl ExpressionNode for IdentifierNode {
    fn print_expression(&self, indent: usize) {
        println!("{}IdentifierNode", "  ".repeat(indent));
        println!("{}name: {}", "  ".repeat(indent + 1), self.name);
    }

    fn compile_expression(&self, program_str: &mut String) {
        program_str.push_str(&format!("LOAD &{}\n", self.name));
    }
}