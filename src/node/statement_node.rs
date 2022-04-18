use crate::node::Node;

pub trait StatementNode {
    fn print_statement(&self, indent: usize);

    fn compile_statement(&self, program_str: &mut String);
}

impl Node for dyn StatementNode {
    fn print(&self, indent: usize) {
        self.print_statement(indent);
    }

    fn compile(&self, program_str: &mut String) {
        self.compile_statement(program_str);
    }
}