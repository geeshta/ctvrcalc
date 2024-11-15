use crate::parser::Ast;

#[derive(Debug)]
pub enum Instruction {
    PUSH(f64),
    NEG,
    ADD,
    SUB,
    MULT,
    DIV,
    MOD,
    POW
}

macro_rules! generate_binary {
    ($bytecode:expr, $left:expr, $right:expr, $instruction:expr) => {{
        extend_bytecode($bytecode, *$left);
        extend_bytecode($bytecode, *$right);
        $bytecode.push($instruction);
    }};
}

pub fn generate_bytecode(ast: Ast) -> Vec<Instruction> {
    let mut bytecode = Vec::new();
    extend_bytecode(&mut bytecode, ast);
    bytecode
}

fn extend_bytecode(bytecode: &mut Vec<Instruction>, ast: Ast) {
    match ast {
        Ast::Value(num) => bytecode.push(Instruction::PUSH(num)),
        Ast::Neg(right) => {
            extend_bytecode(bytecode, *right);
            bytecode.push(Instruction::NEG);
        }
        Ast::Add(left, right) => generate_binary!(bytecode, left, right, Instruction::ADD),
        Ast::Sub(left, right) => generate_binary!(bytecode, left, right, Instruction::SUB),
        Ast::Mult(left, right) => generate_binary!(bytecode, left, right, Instruction::MULT),
        Ast::Div(left, right) => generate_binary!(bytecode, left, right, Instruction::DIV),
        Ast::Mod(left, right) => generate_binary!(bytecode, left, right, Instruction::MOD),
        Ast::Pow(left, right) => generate_binary!(bytecode, left, right, Instruction::POW),
    }
}