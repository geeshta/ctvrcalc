#[derive(Debug)]
pub enum Error {
    LexingError(String),
    ParsingError(String),
    RuntimeError(String)
}