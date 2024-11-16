//// Contains the interpreter that performs the entire compilation and runtime pipeline

use crate::codegen::generate_bytecode;
use crate::error::Error;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::runtime::{run_bytecode, run_bytecode_and_print};

pub struct Interpreter {
    pub input: String,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            input: String::new(),
        }
    }

    /// Silently run the compilation and runtime pipeline and return the calculated result
    /// or an error
    pub fn run(&self) -> Result<f64, Error> {
        tokenize(&self.input)
            .and_then(|tokens| parse(tokens))
            .map(|ast| generate_bytecode(ast))
            .and_then(|bytecode| run_bytecode(&bytecode))
    }

    /// Print each intermediate result along with the execution steps and the calculated result
    pub fn run_and_print(&self) -> () {
        let res = tokenize(&self.input)
            .and_then(|tokens| {
                println!("=== TOKENS ===\n{:?}\n", tokens);
                parse(tokens)
            })
            .map(|ast| {
                println!("=== AST ===\n{:?}\n", ast);
                generate_bytecode(ast)
            })
            .and_then(|bytecode| {
                println!("=== BYTECODE ===");
                for instruction in &bytecode {
                    println!("{:?}", instruction)
                }
                println!("\n=== EXECUTION ===");
                run_bytecode_and_print(&bytecode)
            });

        match res {
            Err(e) => println!("{:?}", e),
            Ok(f) => println!("=== RESULT ===\n{:?}\n", f),
        }
    }
}
