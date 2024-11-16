//// Contains the parsing logic to parse the list of tokens into an AST
//// and also a mutable token stack for matching and lookahead
use crate::error::Error;
use crate::lexer::Token;
use std::fmt::{self, Debug};

/// All valid AST nodes for the lanugage
pub enum Ast {
    Value(f64),
    Neg(Box<Ast>),
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mult(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Mod(Box<Ast>, Box<Ast>),
    Pow(Box<Ast>, Box<Ast>),
}

pub type ParseResult = Result<Ast, Error>;

/// Parse the list of tokens into an AST
pub fn parse(tokens: Vec<Token>) -> ParseResult {
    let mut state = TokenStack::new(tokens);
    expression(&mut state)
}

/// Match a primary expression - a value or parenthesized expression
fn primary(state: &mut TokenStack) -> ParseResult {
    match state.pop_token() {
        Token::Numeral(num) => Ok(Ast::Value(num)),
        Token::LParen => group(state),
        t => Err(Error::ParsingError(format!("No rule found for {:?}", t))),
    }
}

/// Match a negation - unary minus
fn negation(state: &mut TokenStack) -> ParseResult {
    match state.next_token() {
        Token::Minus => Ok(Ast::Neg(Box::new(negation(state.skip())?))),
        _ => primary(state),
    }
}

/// Match exponantiation which is right assocative. 2^2^3 = 2^(2^3) so when encountering a caret,
/// recursively check right side for more exponentiations
fn exponentiation(state: &mut TokenStack) -> ParseResult {
    let mut left = negation(state)?;
    if state.next_matches(&[Token::Caret]) {
        let _ = state.pop_token();
        let right = exponentiation(state)?;
        left = Ast::Pow(Box::new(left), Box::new(right));
    }
    Ok(left)
}

/// Match multiplication, division and modulation which are left associative - 1 * 2 / 3 = (1 * 2) / 3
/// Chain these operations with a while loop linearly
fn factor(state: &mut TokenStack) -> ParseResult {
    let mut expr = exponentiation(state)?;
    while state.next_matches(&[Token::Star, Token::Slash, Token::Percent]) {
        let op = state.pop_token();
        let right = exponentiation(state)?;
        expr = match op {
            Token::Star => Ast::Mult(Box::new(expr), Box::new(right)),
            Token::Slash => Ast::Div(Box::new(expr), Box::new(right)),
            Token::Percent => Ast::Mod(Box::new(expr), Box::new(right)),
            _ => unreachable!(),
        };
    }
    Ok(expr)
}

/// Match left-associative addition and subtraction in the same way as factoring
fn term(state: &mut TokenStack) -> ParseResult {
    let mut expr = factor(state)?;
    while state.next_matches(&[Token::Plus, Token::Minus]) {
        let op = state.pop_token();
        let right = factor(state)?;
        expr = match op {
            Token::Plus => Ast::Add(Box::new(expr), Box::new(right)),
            Token::Minus => Ast::Sub(Box::new(expr), Box::new(right)),
            _ => unreachable!(),
        };
    }
    Ok(expr)
}

/// Match group enclosed in parentheses - the inside can be an arbitrary expression
fn group(state: &mut TokenStack) -> ParseResult {
    let expr = expression(state)?;
    state.expect(&[Token::RParen], "Expected `)`".to_string())?;
    Ok(expr)
}

/// Match any expression - this points to the lowest precedence rule. This used so that when more
/// rules are added, only this method has to be modified
fn expression(state: &mut TokenStack) -> ParseResult {
    term(state)
}

/// Type for a stack of tokens with interface for matching and lookahead
#[derive(Debug)]
struct TokenStack {
    tokens: Vec<Token>,
}

impl TokenStack {
    /// Initiate the stack. The tokens were parsed in order of appearence but Rust's Vec operates
    /// on the _last_ added element so we need to reverse them
    fn new(mut tokens: Vec<Token>) -> TokenStack {
        tokens.push(Token::EOF);
        TokenStack {
            tokens: tokens.into_iter().rev().collect(),
        }
    }

    fn is_empty(&self) -> bool {
        self.tokens.is_empty() || *self.next_token() == Token::EOF
    }

    /// Pop a token, should never be called on an empty stack
    /// That would mean an error in parsing logic
    fn pop_token(&mut self) -> Token {
        self.tokens
            .pop()
            .unwrap_or_else(|| panic!("`pop_token` called on an empty stack"))
    }

    /// Pop a token and return the stack back, used for skipping an already checked token and passing
    /// the stack to another rule in a consice way
    fn skip(&mut self) -> &mut TokenStack {
        self.pop_token();
        self
    }

    /// Peek on the next token, should never be used on an empty stack
    /// That would mean an error in parsing logic
    fn next_token(&self) -> &Token {
        self.tokens
            .last()
            .unwrap_or_else(|| panic!("`pop_token` called on an empty stack"))
    }

    /// Predicate to check if the next token matches any token from a list
    fn next_matches(&self, tokens: &[Token]) -> bool {
        !self.is_empty() && matches!(self.next_token(), token if tokens.contains(token))
    }

    /// Check wheter the next token is any from a list and return an erro if it is not
    fn expect(&mut self, tokens: &[Token], message: String) -> Result<Token, Error> {
        if !self.next_matches(tokens) {
            return Err(Error::ParsingError(message));
        }
        Ok(self.pop_token())
    }
}

/// Macro for printing a binary node - recursively print the left and right children along
/// with the current node name and indentation
macro_rules! print_binary {
    ($f:expr, $indent:expr, $depth:expr, $op:literal, $left:expr, $right:expr) => {{
        writeln!($f, "{}{}", $indent, $op)?;
        $left.print($f, $depth + 1)?;
        $right.print($f, $depth + 1)
    }};
}

/// Pretty print an AST increasing indentation with each level
impl Ast {
    fn print(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        let indent = "|  ".repeat(depth);
        match self {
            Ast::Value(v) => writeln!(f, "{}Value({})", indent, v),
            Ast::Neg(expr) => {
                writeln!(f, "{}Neg", indent)?;
                expr.print(f, depth + 1)
            }
            Ast::Add(left, right) => print_binary!(f, indent, depth, "Add", left, right),
            Ast::Sub(left, right) => print_binary!(f, indent, depth, "Sub", left, right),
            Ast::Mult(left, right) => print_binary!(f, indent, depth, "Mult", left, right),
            Ast::Div(left, right) => print_binary!(f, indent, depth, "Div", left, right),
            Ast::Mod(left, right) => print_binary!(f, indent, depth, "Mod", left, right),
            Ast::Pow(left, right) => print_binary!(f, indent, depth, "Pow", left, right),
        }
    }
}

impl Debug for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print(f, 0)
    }
}
