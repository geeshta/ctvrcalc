mod lexer;
mod parser;

use lexer::tokenize;
use parser::parse;

fn main() {
    let input = "(1 + 2)^3 * 4 / -5 + 6 % (7 + 8)";
    let lexer_result = tokenize(input);
    if let Ok(tokens) = &lexer_result {
        println!("{:?}", tokens);
    }
    let ast = parse(lexer_result);
    println!("{:?}", ast)
}
