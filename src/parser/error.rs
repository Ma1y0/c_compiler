use std::fmt;

use crate::Token;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    Expected(&'static str, Token),
    Other(&'static str),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Other(msg) => write!(f, "ERROR: {}", msg),
            ParserError::Expected(exp, got) => write!(
                f,
                "ERROR: Unexpected error: expected '{}', but got '{:?}'",
                exp, got
            ),
        }
    }
}
