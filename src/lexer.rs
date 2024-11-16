/// Contains the lexing logic
use crate::error::Error;
use regex::Regex;

/// Pattern for matching words, can be changed if the language gets expanded
const PATTERN: &str = r"[()+\-*/^%]|(?:\d*\.\d+|\d+)";

/// All valid words of the language
#[derive(Debug, PartialEq)]
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

/// Match the input text for all words valid in the language and return a list of tokens
pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let re_res = Regex::new(PATTERN);
    match re_res {
        Err(e) => Err(Error::LexingError(format!("{:?}", e))),
        Ok(re) => {
            // Find all matching substrings and map them to their respective tokens
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

            // Remove all matches and leading and trailing whitespace from the input string
            let cleaned_input = re.replace_all(input, "").to_string();
            let cleaned_input = cleaned_input.trim();

            // Check if there are any leftover unmatched characters
            if !cleaned_input.is_empty() {
                return Err(Error::LexingError(format!(
                    "Unexpected characters: '{}'",
                    cleaned_input
                )));
            }

            Ok(tokens)
        }
    }
}
