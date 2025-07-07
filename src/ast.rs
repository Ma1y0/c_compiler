use crate::Token;

/// Node of the AST `Program` should always be at the top
#[derive(Debug, PartialEq)]
pub enum Node {
    Program(Vec<Statement>),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

/// C function
#[derive(Debug, PartialEq)]
pub struct Function {
    return_type: Type,
    name: String,
    parameters: Vec<FnParameter>,
    body: Vec<Statement>,
}

impl Function {
    pub fn new(
        return_type: Type,
        name: String,
        parameters: Vec<FnParameter>,
        body: Vec<Statement>,
    ) -> Self {
        Self {
            return_type,
            name,
            parameters,
            body,
        }
    }
}

/// Function parameters
#[derive(Debug, PartialEq)]
pub struct FnParameter {
    param_type: Type,
    name: String,
}

/// C types
#[derive(Debug, PartialEq)]
pub enum Type {
    Void,
    Int,
}

impl TryFrom<Token> for Type {
    type Error = &'static str;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Int => Ok(Type::Int),
            Token::Void => Ok(Type::Void),
            _ => Err("Invalid type"),
        }
    }
}

/// Statement
#[derive(Debug, PartialEq)]
pub enum Statement {
    Function(Function),
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
}
