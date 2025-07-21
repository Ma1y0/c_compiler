use crate::Token;

/// REF: https://cppreference.com/w/c/language/operator_precedence.html
// Higher number is beater
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Precedence {
    Lowest = 0,
    Assignment = 1,     // = += -= etc.
    Equality = 2,       // == !=
    Relational = 3,     // < > <= >=
    Additive = 4,       // + -
    Multiplicative = 5, // * / %
    Unary = 6,          // + - ! ~ (prefix)
    Postfix = 7,        // [] () -> . (postfix)
}

impl From<&Token> for Precedence {
    /// Convert a token to its precedence level
    fn from(value: &Token) -> Self {
        match value {
            Token::Assignment => Precedence::Assignment,
            Token::Equal => Precedence::Equality,
            Token::Plus | Token::Minus => Precedence::Additive,
            Token::Asterix | Token::Slash => Precedence::Multiplicative,
            _ => Precedence::Lowest,
        }
    }
}

impl Precedence {
    pub fn is_right_associative(token: &Token) -> bool {
        matches!(token, Token::Assignment)
    }
}
