use crate::{
    Token,
    ast::{Expression, FnParameter, Function, Literal, Program, Statement, Type},
    lexer::Lexer,
};

pub struct Parser<'a> {
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

    fn next_token(&mut self) {
        // TODO: I don't like the `.clone`
        self.token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.token != Token::EOF {
            let stmt = self.parse_statment();
            statements.push(stmt);
            self.next_token();
        }

        return Program { statements };
    }

    fn parse_statment(&mut self) -> Statement {
        match self.token {
            Token::Return => self.parse_return_statment(),
            Token::Int | Token::Void => self.parse_declaration(),
            _ => unimplemented!("Not Yet!: {:?}", self.token),
        }
    }

    // Parse declaration
    fn parse_declaration(&mut self) -> Statement {
        let ctype = self.token.clone();
        self.next_token();

        if let Token::Identifier(id) = self.token.clone() {
            self.next_token();
            match self.token {
                Token::Assignmen => unimplemented!("Var declaration"),
                Token::OpenParen => self.parse_function(ctype, id),
                _ => todo!("Habdle error"),
            }
        } else {
            todo!("Handle error no identifier");
        }
    }

    fn parse_function(&mut self, ctype: Token, name: String) -> Statement {
        Statement::Function(Function::new(
            Type::try_from(ctype).unwrap(),
            name,
            self.parse_fn_parameters(),
            self.parse_body(),
        ))
    }

    fn parse_fn_parameters(&mut self) -> Vec<FnParameter> {
        self.next_token(); // Consume the `(`
        while self.token != Token::CloseParen {
            self.next_token();
        }
        self.next_token(); // Consume the `)`
        Vec::new()
    }

    fn parse_body(&mut self) -> Vec<Statement> {
        if self.token != Token::OpenBrace {
            todo!("Handle no open brace");
        }
        self.next_token(); // Consume the '{'

        let mut statements = Vec::new();

        while self.token != Token::CloseBrace {
            let stmt = self.parse_statment();
            statements.push(stmt);
            self.next_token();
        }

        self.next_token(); // Consume the '}'

        statements
    }

    fn parse_return_statment(&mut self) -> Statement {
        // Consume `return`
        self.next_token();

        if let Token::IntegerLiteral(n) = self.token {
            // Consume `;`
            self.next_token();
            Statement::Return(Expression::Literal(Literal::Integer(n)))
        } else {
            todo!("Handle Errors: {:?}", self.token)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Function, Literal, Statement, Type};

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
            statements: vec![Statement::Function(Function::new(
                Type::Int,
                "main".to_string(),
                Vec::new(),
                vec![Statement::Return(Expression::Literal(Literal::Integer(2)))],
            ))],
        };

        assert_eq!(expected, parse_src(src));
    }
}
