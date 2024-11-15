mod lexer;
mod parser;
mod codegen;
mod error;

use lexer::tokenize;
use parser::parse;
use codegen::generate_bytecode;

fn main() {
    let input = "(1 + 2)^3 * 4 / -5 + 6 % (7 + 8)";
    println!("Input:\n{}\n", input);
    let lexer_result = tokenize(input);
    if let Ok(tokens) = &lexer_result {
        println!("Tokens:\n{:?}\n", tokens);
    }
    let ast_result = parse(lexer_result);
    match ast_result {
        Err(e) => println!("{:?}", e),
        Ok(ast) => {
            println!("AST:\n{:?}\n", ast);
            let bytecode = generate_bytecode(ast);
            println!("Bytecode: \n{:?}\n", bytecode);
        }
    }
}
