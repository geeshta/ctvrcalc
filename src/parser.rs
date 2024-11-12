#[derive(Debug)]
pub enum Ast {
    Value(f64),
    Neg(Box<Ast>),
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mult(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Pow(Box<Ast>, Box<Ast>),
    Mod(Box<Ast>, Box<Ast>),
}
