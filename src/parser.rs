use crate::lexer::Token;

#[derive(Debug)]
pub enum Error {
    LexingError(String),
    ParsingError(String),
}

#[derive(Debug)]
pub enum Ast {
    Value(f64),
    Neg(Box<Ast>),
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mult(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Mod(Box<Ast>, Box<Ast>),
    Exp(Box<Ast>, Box<Ast>),
}

#[derive(Debug)]
struct TokenStack {
    tokens: Vec<Token>,
}

impl TokenStack {
    fn new(mut tokens: Vec<Token>) -> TokenStack {
        tokens.push(Token::EOF);
        TokenStack {
            tokens: tokens.into_iter().rev().collect(),
        }
    }

    fn is_empty(&self) -> bool {
        self.tokens.is_empty() || *self.next_token() == Token::EOF
    }

    fn pop_token(&mut self) -> Token {
        self.tokens
            .pop()
            .unwrap_or_else(|| panic!("`pop_token` called on an empty stack"))
    }

    fn skip(&mut self) -> &mut TokenStack {
        self.tokens.pop();
        self
    }

    fn next_token(&self) -> &Token {
        self.tokens
            .last()
            .unwrap_or_else(|| panic!("`pop_token` called on an empty stack"))
    }

    fn next_matches(&self, tokens: &[Token]) -> bool {
        !self.is_empty() && matches!(self.next_token(), token if tokens.contains(token))
    }

    fn expect(&mut self, tokens: &[Token], message: String) -> Result<Token, Error> {
        if !self.next_matches(tokens) {
            return Err(Error::ParsingError(message));
        }
        Ok(self.pop_token())
    }
}

pub type ParseResult = Result<Ast, Error>;

macro_rules! chain_binary {
    ($func_name:ident, $next_func:ident, $tokens:expr, $constructor:expr) => {
        fn $func_name(state: &mut TokenStack) -> ParseResult {
            let mut expr = $next_func(state)?;
            // println!("Entered macro func: {}", stringify!($func_name));
            while state.next_matches($tokens) {
                let op = state.pop_token();
                let right = $next_func(state)?;
                expr = $constructor(op, Box::new(expr), Box::new(right));
            }
            Ok(expr)
        }
    };
}

fn primary(state: &mut TokenStack) -> ParseResult {
    match state.pop_token() {
        Token::Numeral(num) => Ok(Ast::Value(num)),
        Token::LParen => group(state),
        t => Err(Error::ParsingError(format!("No rule found for {:?}", t))),
    }
}

fn negation(state: &mut TokenStack) -> ParseResult {
    match state.next_token() {
        Token::Minus => Ok(Ast::Neg(Box::new(negation(state.skip())?))),
        _ => primary(state),
    }
}

chain_binary!(
    exponentiation,
    negation,
    &[Token::Caret],
    |_, left, right| { Ast::Exp(left, right) }
);

chain_binary!(
    factor,
    exponentiation,
    &[Token::Star, Token::Slash, Token::Percent],
    |op, left, right| {
        match op {
            Token::Star => Ast::Mult(left, right),
            Token::Slash => Ast::Div(left, right),
            Token::Percent => Ast::Mod(left, right),
            _ => unreachable!(),
        }
    }
);

chain_binary!(
    term,
    factor,
    &[Token::Plus, Token::Minus],
    |op, left, right| {
        match op {
            Token::Plus => Ast::Add(left, right),
            Token::Minus => Ast::Sub(left, right),
            _ => unreachable!(),
        }
    }
);

fn group(state: &mut TokenStack) -> ParseResult {
    let expr = expression(state)?;
    state.expect(&[Token::RParen], "Expected `)`".to_string())?;
    Ok(expr)
}

fn expression(state: &mut TokenStack) -> ParseResult {
    term(state)
}

pub fn parse(lexing_result: Result<Vec<Token>, regex::Error>) -> ParseResult {
    match lexing_result {
        Err(e) => Err(Error::LexingError(format!("{:?}", e))),
        Ok(tokens) => {
            let mut state = TokenStack::new(tokens);
            expression(&mut state)
        }
    }
}
