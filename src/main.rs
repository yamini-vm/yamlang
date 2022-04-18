use std::env;

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

    for token in tokens {
        println!("{:?}", token);
    }

    // let parser = Parser::new();
    // let ast_root = parser.parse(tokens);

    // let compiler = Compiler::new();
    // compiler.compile(ast_root);
}