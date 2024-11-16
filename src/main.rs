mod codegen;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod runtime;

use interpreter::Interpreter;
use std::io::{stdin, stdout, Write};

/// Run a REPL
fn main() {
    let mut line: String;
    let mut interpreter = Interpreter::new();

    loop {
        line = String::new();
        print!("> ");
        _ = stdout().flush();

        match stdin().read_line(&mut line) {
            Err(e) => {
                println!("{}", e);
            }
            Ok(_) => {
                interpreter.input = line;
                interpreter.run_and_print();
            }
        }
    }
}
