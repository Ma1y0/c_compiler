use std::{iter::Peekable, str::Chars};

use crate::Token;

pub struct Lexer<'a> {
    buffer: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            buffer: s.chars().peekable(),
        }
    }

    /// Lex all tokens
    pub fn lex_all(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token);
            if tokens.last() == Some(&Token::EOF) {
                break;
            }
        }
        tokens
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();

        let ch = match self.buffer.next() {
            Some(ch) => ch,
            None => return Token::EOF,
        };

        match ch {
            // Punctuation
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ';' => Token::Semicolon,

            // Operators
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterix,
            '/' => {
                // Check if it is a comment
                match self.buffer.peek() {
                    Some('/') => {
                        self.buffer.next(); // Consume the second '/'
                        self.consume_comment();

                        // Get the next token (recursive call)
                        self.next_token()
                    }
                    Some('*') => {
                        self.buffer.next(); // Consume '*'
                        match self.consume_multi_line_comment() {
                            Ok(()) => self.next_token(),
                            Err(err) => Token::Error(err),
                        }
                    }

                    _ => Token::Slash,
                }
            }

            // Keyword or identifier
            ch if ch.is_alphabetic() || ch == '_' => self.read_keyword_or_identifier(ch),

            // Integer literal
            ch if ch.is_ascii_digit() => self.read_integer_literal(ch),

            // Unexpected token
            ch => Token::Error(format!("Unexpected character: '{}'", ch)),
        }
    }

    fn read_keyword_or_identifier(&mut self, first: char) -> Token {
        let mut literal = String::new();
        literal.push(first);

        while let Some(&ch) = self.buffer.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                literal.push(self.buffer.next().unwrap());
            } else {
                break;
            }
        }

        match literal.as_str() {
            "int" => Token::Int,
            "void" => Token::Void,
            "return" => Token::Return,
            _ => Token::Identifier(literal),
        }
    }

    fn read_integer_literal(&mut self, first: char) -> Token {
        let mut literal = String::new();
        literal.push(first);

        while let Some(&ch) = self.buffer.peek() {
            if ch.is_ascii_digit() {
                literal.push(self.buffer.next().unwrap());
            } else {
                break;
            }
        }

        match literal.parse::<u64>() {
            Ok(num) => Token::IntegerLiteral(num),
            Err(e) => Token::Error(format!(
                "Invalid integer literal: '{}', with: {}",
                literal, e
            )),
        }
    }

    /// Consumes whitespace
    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.buffer.peek() {
            if ch.is_whitespace() {
                self.buffer.next();
            } else {
                break;
            }
        }
    }

    /// Consumes single line comment
    fn consume_comment(&mut self) {
        while let Some(ch) = self.buffer.peek() {
            if *ch == '\n' {
                break;
            }
            self.buffer.next();
        }
    }

    /// Consumes multi line comment
    fn consume_multi_line_comment(&mut self) -> Result<(), String> {
        while let Some(ch) = self.buffer.next() {
            if ch == '*' {
                if let Some('/') = self.buffer.peek() {
                    self.buffer.next(); // Consume the '/' as the end
                    return Ok(());
                }
            }
        }

        Err("Unterminated multi-line comment".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::Token;

    use super::*;

    #[test]
    fn test_lexer_simple_program_tokens() {
        let input = "int main(void) { return 2; }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Int);
        assert_eq!(lexer.next_token(), Token::Identifier("main".to_string()));
        assert_eq!(lexer.next_token(), Token::OpenParen);
        assert_eq!(lexer.next_token(), Token::Void);
        assert_eq!(lexer.next_token(), Token::CloseParen);
        assert_eq!(lexer.next_token(), Token::OpenBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::IntegerLiteral(2));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::CloseBrace);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_whitespace_and_newline() {
        let input = "int \n main ( void ) { \t return 2 ; }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Int);
        assert_eq!(lexer.next_token(), Token::Identifier("main".to_string()));
        assert_eq!(lexer.next_token(), Token::OpenParen);
        assert_eq!(lexer.next_token(), Token::Void);
        assert_eq!(lexer.next_token(), Token::CloseParen);
        assert_eq!(lexer.next_token(), Token::OpenBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::IntegerLiteral(2));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::CloseBrace);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_unexpected_character() {
        let input = "int $ main";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Int);
        assert_eq!(
            lexer.next_token(),
            Token::Error("Unexpected character: '$'".to_string())
        );
        assert_eq!(lexer.next_token(), Token::Identifier("main".to_string()));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_empty_input() {
        let input = "";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_comment() {
        let input = "// A comment \n int main ( void ) { \t return 2 ; }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Int);
        assert_eq!(lexer.next_token(), Token::Identifier("main".to_string()));
        assert_eq!(lexer.next_token(), Token::OpenParen);
        assert_eq!(lexer.next_token(), Token::Void);
        assert_eq!(lexer.next_token(), Token::CloseParen);
        assert_eq!(lexer.next_token(), Token::OpenBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::IntegerLiteral(2));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::CloseBrace);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_division() {
        let input = "int main(void) { return 2 / 5; }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Int);
        assert_eq!(lexer.next_token(), Token::Identifier("main".to_string()));
        assert_eq!(lexer.next_token(), Token::OpenParen);
        assert_eq!(lexer.next_token(), Token::Void);
        assert_eq!(lexer.next_token(), Token::CloseParen);
        assert_eq!(lexer.next_token(), Token::OpenBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::IntegerLiteral(2));
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::IntegerLiteral(5));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::CloseBrace);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_multi_line_comment() {
        let input = "/* This a a comment. Bla Bla \n Bla fwaiooja koawf \n lkdalw */ int main(void) { return 2; }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Int);
        assert_eq!(lexer.next_token(), Token::Identifier("main".to_string()));
        assert_eq!(lexer.next_token(), Token::OpenParen);
        assert_eq!(lexer.next_token(), Token::Void);
        assert_eq!(lexer.next_token(), Token::CloseParen);
        assert_eq!(lexer.next_token(), Token::OpenBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::IntegerLiteral(2));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::CloseBrace);
        assert_eq!(lexer.next_token(), Token::EOF);
    }
}
