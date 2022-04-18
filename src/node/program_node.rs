use crate::node::Node;
use crate::node::StatementNode;

pub struct ProgramNode {
    pub(crate) function: String,
    pub(crate) statement_nodes: Vec<Box<dyn StatementNode>>,
}

impl Node for ProgramNode {
    fn print(&self, indent: usize) {
        println!("{}ProgramNode", "  ".repeat(indent));
        println!("{}function: {}", "  ".repeat(indent + 1), self.function);
        println!("{}statement_nodes:", "  ".repeat(indent + 1));
        for statement_node in &self.statement_nodes {
            statement_node.print(indent + 2);
        }
    }

    fn compile(&self, program_str: &mut String) {
        for statement_node in &self.statement_nodes {
            statement_node.compile(program_str);
        }

        program_str.push_str(&format!("HALT\n"));
    }
}