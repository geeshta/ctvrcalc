use regex::Regex;

const PATTERN: &str = r"[()+\-*/^%]|(?:\d*\.\d+|\d+)";

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Percent,
    Numeral(f64),
    EOF,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, regex::Error> {
    let re = Regex::new(PATTERN)?;

    let tokens = re
        .find_iter(input)
        .map(|m| match m.as_str() {
            "(" => Token::LParen,
            ")" => Token::RParen,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Star,
            "/" => Token::Slash,
            "^" => Token::Caret,
            "%" => Token::Percent,
            numeral => {
                let normalized_input = if numeral.starts_with('.') {
                    format!("0{}", numeral)
                } else {
                    numeral.to_string()
                };

                Token::Numeral(normalized_input.parse::<f64>().unwrap())
            }
        })
        .collect();

    Ok(tokens)
}
