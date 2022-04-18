use crate::node::Node;

pub struct Compiler {

}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {

        }
    }

    pub fn compile(&self, ast_root: impl Node) -> String {
        let mut program_str = String::new();
        ast_root.compile(&mut program_str);

        program_str
    }
}