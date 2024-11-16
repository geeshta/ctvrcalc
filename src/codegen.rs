//// Logic for traversing and AST and generating a "bytecode" of instructions for the runtime
use crate::parser::Ast;

/// Valid instructions for the runtime ("bytecode" even though the instructions are not actual bytes)
#[derive(Debug)]
pub enum Instruction {
    PUSH(f64),
    NEG,
    ADD,
    SUB,
    MULT,
    DIV,
    MOD,
    POW,
}

/// Public interface for bytecode generation from an AST
pub fn generate_bytecode(ast: Ast) -> Vec<Instruction> {
    let mut bytecode = Vec::new();
    extend_bytecode(&mut bytecode, ast);
    bytecode
}

/// Recursive function that first visits the leaves of a node and appends the node instruction last
/// for operation on a stack-based runtime
fn extend_bytecode(bytecode: &mut Vec<Instruction>, ast: Ast) {
    match ast {
        Ast::Value(num) => {
            bytecode.push(Instruction::PUSH(num));
        }
        Ast::Neg(right) => {
            extend_bytecode(bytecode, *right);
            bytecode.push(Instruction::NEG);
        }
        Ast::Add(left, right) => {
            extend_bytecode(bytecode, *left);
            extend_bytecode(bytecode, *right);
            bytecode.push(Instruction::ADD);
        }
        Ast::Sub(left, right) => {
            extend_bytecode(bytecode, *left);
            extend_bytecode(bytecode, *right);
            bytecode.push(Instruction::SUB);
        }
        Ast::Mult(left, right) => {
            extend_bytecode(bytecode, *left);
            extend_bytecode(bytecode, *right);
            bytecode.push(Instruction::MULT);
        }
        Ast::Div(left, right) => {
            extend_bytecode(bytecode, *left);
            extend_bytecode(bytecode, *right);
            bytecode.push(Instruction::DIV);
        }
        Ast::Mod(left, right) => {
            extend_bytecode(bytecode, *left);
            extend_bytecode(bytecode, *right);
            bytecode.push(Instruction::MOD);
        }
        Ast::Pow(left, right) => {
            extend_bytecode(bytecode, *left);
            extend_bytecode(bytecode, *right);
            bytecode.push(Instruction::POW);
        }
    }
}
