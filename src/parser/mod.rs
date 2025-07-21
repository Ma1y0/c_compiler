mod ast;
mod error;
mod precedence;

use crate::{
    Token,
    lexer::Lexer,
    parser::{
        ast::{
            BinaryOp, CType, Declaration, Expression, FnParameter, FunctionDeclaration, Statement,
            TranslationUnit, Unary, VariableDeclaration,
        },
        error::{ParserError, ParserResult},
        precedence::Precedence,
    },
};

/// Public API for parsing source to `TranslationUnit`
pub fn parse(src: &str) -> ParserResult<TranslationUnit> {
    let mut lexer = Lexer::new(src);
    let mut parser = Parser::new(&mut lexer);

    parser.parse()
}

struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        // TODO: Can this be done without the `.clone()`?
        self.token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> ParserResult<TranslationUnit> {
        let mut declarations = Vec::new();

        while self.token != Token::EOF {
            declarations.push(self.parse_declaration()?);
        }

        Ok(TranslationUnit { declarations })
    }

    fn parse_declaration(&mut self) -> ParserResult<Declaration> {
        // TODO: This code expects type to be only one token, but in c there can be multi token types (e.g. `unsigned int x = 5;`)
        // Convert the current token to a `CType`
        let ctype = CType::try_from(&self.token)?;
        self.next_token(); // Consume the type
        let identifier = match &self.token {
            Token::Identifier(id) => id.clone(),
            _ => return Err(ParserError::Expected("identifier", self.token.clone())),
        };
        self.next_token(); // Consume the identifier

        match self.token {
            Token::Assignment | Token::Semicolon => {
                self.parse_variable_declaration(ctype, identifier)
            }
            Token::OpenParen => self.parse_function_declaration(ctype, identifier),
            _ => Err(ParserError::Expected("declaration", self.token.clone())),
        }
    }

    fn parse_function_declaration(
        &mut self,
        ctype: CType,
        name: String,
    ) -> ParserResult<Declaration> {
        let parameters = self.parse_function_args()?;
        let body = self.parse_body()?;

        Ok(Declaration::Function(FunctionDeclaration::new(
            ctype,
            name,
            parameters,
            Some(body),
        )))
    }

    fn parse_function_args(&mut self) -> ParserResult<Vec<FnParameter>> {
        self.next_token(); // Consume the '('
        // Consume the args
        while self.token != Token::CloseParen {
            self.next_token();
        }
        self.next_token(); // Consume the ')'

        // TODO: Parse function parameters
        Ok(vec![FnParameter::new(CType::Void, String::new())])
    }

    fn parse_body(&mut self) -> ParserResult<Vec<Statement>> {
        self.next_token(); // Consume the '{'
        let mut statements = Vec::new();
        while self.token != Token::CloseBrace {
            statements.push(self.parse_statement()?);
        }
        self.next_token(); // Consume the '}'

        Ok(statements)
    }

    fn parse_variable_declaration(
        &mut self,
        ctype: CType,
        name: String,
    ) -> ParserResult<Declaration> {
        match self.token {
            Token::Semicolon => Ok(Declaration::Variable(VariableDeclaration::new(
                ctype, name, None,
            ))),
            Token::Assignment => {
                self.next_token(); // Consume the `=`
                Ok(Declaration::Variable(VariableDeclaration::new(
                    ctype,
                    name,
                    Some(self.parse_expresssion(Precedence::Lowest)?),
                )))
            }
            _ => Err(ParserError::Expected(
                "assignmen | semicolon",
                self.token.clone(),
            )),
        }
    }

    fn parse_expresssion(&mut self, precedence: Precedence) -> ParserResult<Expression> {
        let mut left = self.parse_prefix_expression()?;

        // Parse infix operators while they have higher precedence
        while self.token != Token::Semicolon
            && self.token != Token::CloseParen
            && self.token != Token::CloseBrace
            && precedence < Precedence::from(&self.token)
        {
            left = self.parse_infix_expression(left)?;
        }

        dbg!(&self.token);
        if self.token == Token::Semicolon {
            self.next_token();
        }

        Ok(left)
    }

    fn parse_prefix_expression(&mut self) -> ParserResult<Expression> {
        match &self.token {
            Token::IntegerLiteral(value) => {
                let value = *value;
                self.next_token();
                Ok(Expression::IntegerLiteral(value))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.next_token();
                Ok(Expression::Identifier(name))
            }
            Token::Plus => {
                self.next_token(); // Consume '+'
                let expr = self.parse_expresssion(Precedence::Unary)?;
                Ok(Expression::Unary(Box::new(Unary::Positive(expr))))
            }
            Token::Minus => {
                self.next_token(); // Consume '-'
                let expr = self.parse_expresssion(Precedence::Unary)?;
                Ok(Expression::Unary(Box::new(Unary::Negative(expr))))
            }
            Token::OpenParen => {
                self.next_token(); // Consume '('
                let expr = self.parse_expresssion(Precedence::Lowest)?;
                if self.token != Token::CloseParen {
                    return Err(ParserError::Expected(
                        "closing parenthesis",
                        self.token.clone(),
                    ));
                }
                self.next_token(); // Consume ')'
                Ok(expr)
            }
            _ => Err(ParserError::Expected("expression", self.token.clone())),
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> ParserResult<Expression> {
        let operator = self.token.clone();
        let precedence = Precedence::from(&operator);
        self.next_token(); // Consume the operator

        let right = self.parse_expresssion(precedence)?;

        let binary_op = match operator {
            Token::Plus => BinaryOp::Add(left, right),
            Token::Minus => BinaryOp::Subtract(left, right),
            Token::Asterix => BinaryOp::Multiply(left, right),
            Token::Slash => BinaryOp::Divide(left, right),
            Token::Equal => BinaryOp::Equals(left, right),
            Token::Assignment => BinaryOp::Assign(left, right),
            _ => return Err(ParserError::Expected("binary operator", operator)),
        };

        Ok(Expression::BinaryOp(Box::new(binary_op)))
    }

    fn parse_statement(&mut self) -> ParserResult<Statement> {
        let statement = match self.token {
            Token::Return => self.parse_return_statement()?,
            Token::Int => unimplemented!("Var statement"),
            _ => unimplemented!("Statement"),
        };

        Ok(statement)
    }

    fn parse_return_statement(&mut self) -> ParserResult<Statement> {
        self.next_token(); // Consume the `return` keyword
        let expression = self.parse_expresssion(Precedence::Lowest)?;

        Ok(Statement::Return(expression))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{
        CType, Declaration, Expression, FnParameter, FunctionDeclaration, Statement,
        VariableDeclaration,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parser_parse_simple_main_function() {
        let src = "int main(void) { return 0; }";
        let ast = parse(src).unwrap();

        let expected = TranslationUnit {
            declarations: vec![Declaration::Function(FunctionDeclaration::new(
                CType::Int,
                "main".to_string(),
                vec![FnParameter::new(CType::Void, "".to_string())],
                Some(vec![Statement::Return(Expression::IntegerLiteral(0))]),
            ))],
        };

        // tests
        assert_eq!(expected, ast);
    }

    #[test]
    fn test_parser_parse_global_variable() {
        let src = "int a = 5;\nvoid main(void) {return 0;}";
        let ast = parse(src).unwrap();

        let expected = TranslationUnit {
            declarations: vec![
                Declaration::Variable(VariableDeclaration::new(
                    CType::Int,
                    "a".to_string(),
                    Some(Expression::IntegerLiteral(5)),
                )),
                Declaration::Function(FunctionDeclaration::new(
                    CType::Void,
                    "main".to_string(),
                    vec![FnParameter::new(CType::Void, "".to_string())],
                    Some(vec![Statement::Return(Expression::IntegerLiteral(0))]),
                )),
            ],
        };

        // tests
        assert_eq!(expected, ast);
    }

    #[test]
    fn test_parser_parse_prefix_expression() {
        let src = "int a = +5; int b = -6;";
        let ast = parse(src).unwrap();

        let expected = TranslationUnit {
            declarations: vec![
                Declaration::Variable(VariableDeclaration::new(
                    CType::Int,
                    "a".to_string(),
                    Some(Expression::Unary(Box::new(Unary::Positive(
                        Expression::IntegerLiteral(5),
                    )))),
                )),
                Declaration::Variable(VariableDeclaration::new(
                    CType::Int,
                    "b".to_string(),
                    Some(Expression::Unary(Box::new(Unary::Negative(
                        Expression::IntegerLiteral(6),
                    )))),
                )),
            ],
        };

        assert_eq!(expected, ast);
    }

    #[test]
    fn test_parser_parse_binary_expression() {
        let src = "int a = 2 + 3 * 4;";
        let ast = parse(src).unwrap();

        // Should parse as 2 + (3 * 4) due to precedence
        let expected = TranslationUnit {
            declarations: vec![Declaration::Variable(VariableDeclaration::new(
                CType::Int,
                "a".to_string(),
                Some(Expression::BinaryOp(Box::new(BinaryOp::Add(
                    Expression::IntegerLiteral(2),
                    Expression::BinaryOp(Box::new(BinaryOp::Multiply(
                        Expression::IntegerLiteral(3),
                        Expression::IntegerLiteral(4),
                    ))),
                )))),
            ))],
        };

        assert_eq!(expected, ast);
    }

    #[test]
    fn test_parser_parse_parenthesized_expression() {
        let src = "int a = (2 + 3) * 4;";
        let ast = parse(src).unwrap();

        // Should parse as (2 + 3) * 4
        let expected = TranslationUnit {
            declarations: vec![Declaration::Variable(VariableDeclaration::new(
                CType::Int,
                "a".to_string(),
                Some(Expression::BinaryOp(Box::new(BinaryOp::Multiply(
                    Expression::BinaryOp(Box::new(BinaryOp::Add(
                        Expression::IntegerLiteral(2),
                        Expression::IntegerLiteral(3),
                    ))),
                    Expression::IntegerLiteral(4),
                )))),
            ))],
        };

        assert_eq!(expected, ast);
    }

    #[test]
    fn test_parser_complex_expression() {
        let src = "int result = -5 + 3 * 2 - 1;";
        let ast = parse(src).unwrap();

        // Should parse as (-5) + (3 * 2) - 1
        let expected = TranslationUnit {
            declarations: vec![Declaration::Variable(VariableDeclaration::new(
                CType::Int,
                "result".to_string(),
                Some(Expression::BinaryOp(Box::new(BinaryOp::Subtract(
                    Expression::BinaryOp(Box::new(BinaryOp::Add(
                        Expression::Unary(Box::new(Unary::Negative(Expression::IntegerLiteral(5)))),
                        Expression::BinaryOp(Box::new(BinaryOp::Multiply(
                            Expression::IntegerLiteral(3),
                            Expression::IntegerLiteral(2),
                        ))),
                    ))),
                    Expression::IntegerLiteral(1),
                )))),
            ))],
        };

        assert_eq!(expected, ast);
    }
}
