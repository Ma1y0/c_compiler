#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Int,
    Void,
    Return,

    // Identifiers (e.g., 'main')
    Identifier(String),

    // Literals
    IntegerLiteral(u64),
    CharacterLiteral(char), // 'x'

    // Punctuators
    OpenParen,  // (
    CloseParen, // )
    OpenBrace,  // {
    CloseBrace, // }
    Semicolon,  // ;
    Colon,      // :

    // Operators
    Plus,    // +
    Minus,   // -
    Asterix, // *
    Slash,   // /

    // Utils
    Error(String),
    EOF,
}
