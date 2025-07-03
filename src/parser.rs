use crate::{Token, ast::Program, lexer::Lexer};

struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let token = lexer.next_token();
        let peek_token = lexer.next_token();

        return Parser {
            lexer: lexer,
            token: token,
            peek_token: peek_token,
        };
    }

    fn parse_program(&mut self) -> Program {
        return Program {
            functions: Vec::new(),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expresssion, Function, Statement, Type};

    use super::*;

    fn parse_src(s: &str) -> Program {
        let mut lexer = Lexer::new(s);
        let mut parser = Parser::new(&mut lexer);
        parser.parse_program()
    }

    #[test]
    fn test_parser_parse_empty_main() {
        let src = "int main(void) { return 2;}";
        let expected = Program {
            functions: vec![Function::new(
                Type::Int,
                "name".to_string(),
                Vec::new(),
                vec![Statement::Return(Expresssion::IntegerLieral(2))],
            )],
        };

        assert_eq!(expected, parse_src(src));
    }
}
