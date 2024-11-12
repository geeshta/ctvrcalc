mod lexer;
mod parser;

fn main() {
    if let Ok(tokens) = lexer::tokenize("(12.6)/17 + .35^2") {
        for token in tokens {
            println!("{:?}", token)
        }
    }
}
