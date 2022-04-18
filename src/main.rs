use std::env;
use std::io::Write;

use yamlang::lexer::Lexer;
use yamlang::parser::Parser;
use yamlang::compiler::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filepath>", args[0]);
        return;
    }
    let file_path = &args[1];

    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize(&file_path);

    let mut parser = Parser::new(tokens);
    let ast_root = parser.parse();

    let compiler = Compiler::new();
    let asm = compiler.compile(ast_root);

    let mut asm_file = std::fs::File::create(format!("{}.yas", file_path)).unwrap();
    asm_file.write_all(asm.as_bytes()).unwrap();
}