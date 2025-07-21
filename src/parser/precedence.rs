use crate::Token;

/// REF: https://cppreference.com/w/c/language/operator_precedence.html
/// Converts `Token` to precedence
fn get_precedence(token: &Token) -> Option<u8> {
    match *token {
        Token::Asterix | Token::Slash => Some(3),
        Token::Plus | Token::Minus => Some(4),
        _ => None,
    }
}
