//// Stack-based runtime

use crate::codegen::Instruction;
use crate::error::Error;

/// Public interface for silently running the bytecode
pub fn run_bytecode(bytecode: &[Instruction]) -> Result<f64, Error> {
    let mut runtime = Runtime::new();
    runtime.run(bytecode)
}

/// Public interface for running the bytecode and printing stack states and instructions
pub fn run_bytecode_and_print(bytecode: &[Instruction]) -> Result<f64, Error> {
    let mut runtime = Runtime::new();
    runtime.debug = true;
    runtime.run(bytecode)
}

/// The stack-operating runtime
#[derive(Debug)]
struct Runtime {
    stack: Vec<f64>,
    debug: bool,
}

impl Runtime {
    fn new() -> Runtime {
        Runtime {
            stack: Vec::new(),
            debug: false,
        }
    }

    /// Pop the top value from the stack. Should never be called on an empty stack.
    /// That would indicate an error in the parsing or codegen logic
    fn pop(&mut self) -> f64 {
        self.stack
            .pop()
            .unwrap_or_else(|| panic!("`pop` called on an empty runtime stack"))
    }

    /// Execute all instructions
    fn run(&mut self, bytecode: &[Instruction]) -> Result<f64, Error> {
        for instr in bytecode {
            self.run_instruction(instr)?
        }
        Ok(self.pop())
    }

    /// Execute a single instruction.
    /// Either push a value to a stack, or pop one or two values, evaluet the operation
    /// and push the result back to the top
    fn run_instruction(&mut self, instr: &Instruction) -> Result<(), Error> {
        match instr {
            Instruction::PUSH(num) => self.stack.push(*num),
            Instruction::NEG => {
                let value = self.pop();
                self.stack.push(-value)
            }
            Instruction::POW => {
                let (exp, base) = (self.pop(), self.pop());
                self.stack.push(base.powf(exp))
            }
            Instruction::MULT => {
                let (right, left) = (self.pop(), self.pop());
                self.stack.push(left * right)
            }
            Instruction::DIV => {
                let (right, left) = (self.pop(), self.pop());
                if right == 0.0 {
                    return Err(Error::RuntimeError(format!("Division by zero")));
                }
                self.stack.push(left / right)
            }
            Instruction::MOD => {
                let (right, left) = (self.pop(), self.pop());
                self.stack.push(left.rem_euclid(right))
            }
            Instruction::ADD => {
                let (right, left) = (self.pop(), self.pop());
                self.stack.push(left + right)
            }
            Instruction::SUB => {
                let (right, left) = (self.pop(), self.pop());
                self.stack.push(left - right)
            }
        }
        if self.debug {
            println!("{:?}", instr);
            println!("{:?}", self.stack);
        }
        Ok(())
    }
}
