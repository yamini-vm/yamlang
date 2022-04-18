use crate::node::ExpressionNode;

pub struct StringNode {
    pub(crate) value: String,
}

impl ExpressionNode for StringNode {
    fn print_expression(&self, indent: usize) {
        println!("{}StringNode", "  ".repeat(indent));
        println!("{}value: {}", "  ".repeat(indent + 1), self.value);
    }

    fn compile_expression(&self, program_str: &mut String) {
        program_str.push_str(&format!("LOAD \"{}\"\n", self.value));

    }
}