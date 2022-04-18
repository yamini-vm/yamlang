pub trait Node {
    fn print(&self, indent: usize);

    fn compile(&self, program_str: &mut String);
}