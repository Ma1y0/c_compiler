#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Int,
    Void,
    Return,

    // Identifiers (e.g., 'main')
    Identifier(String),

    // Literals
    IntegerLiteral(i64),
    CharacterLiteral(char), // 'x'

    // Punctuation
    OpenParen,  // (
    CloseParen, // )
    OpenBrace,  // {
    CloseBrace, // }
    Semicolon,  // ;
    Colon,      // :

    // Operators
    Plus,      // +
    Minus,     // -
    Asterix,   // *
    Slash,     // /
    Assignmen, // =
    Equal,     // ==

    // Utils
    Error(String),
    EOF,
}
