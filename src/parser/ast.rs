use crate::{Token, parser::error::ParserError};

/// The top node of every AST. Represents each c file
#[derive(Debug, PartialEq)]
pub struct TranslationUnit {
    pub declarations: Vec<Declaration>,
}

/// C data types
#[derive(Debug, PartialEq)]
pub enum CType {
    Int,
    Void,
}

impl TryFrom<&Token> for CType {
    type Error = ParserError;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::Int => Ok(CType::Int),
            Token::Void => Ok(CType::Void),
            _ => Err(ParserError::Expected("type token", value.clone())),
        }
    }
}

/// C expression
#[derive(Debug, PartialEq)]
pub enum Expression {
    IntegerLiteral(i64),
    Identifier(String),
    BinaryOp(Box<BinaryOp>),
    Unary(Box<Unary>),
}

#[derive(Debug, PartialEq)]
pub enum Unary {
    Positive(Expression),
    Negative(Expression),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add(Expression, Expression),
    Subtract(Expression, Expression),
    Multiply(Expression, Expression),
    Divide(Expression, Expression),
    Equals(Expression, Expression),
    NotEquals(Expression, Expression),
    Assign(Expression, Expression),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declaration(VariableDeclaration),
    ExpressionStatment(Expression), // e.g., `5 + 6;`, `my_func();`
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    ctype: CType,
    name: String,
    initilizer: Option<Expression>,
}

impl VariableDeclaration {
    pub fn new(ctype: CType, name: String, initilizer: Option<Expression>) -> Self {
        Self {
            ctype,
            name,
            initilizer,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    return_type: CType,
    name: String,
    parameters: Vec<FnParameter>,
    body: Option<Vec<Statement>>,
}

impl FunctionDeclaration {
    pub fn new(
        return_type: CType,
        name: String,
        parameters: Vec<FnParameter>,
        body: Option<Vec<Statement>>,
    ) -> Self {
        Self {
            return_type,
            name,
            parameters,
            body,
        }
    }
}

/// `ctype` is void and `name` is empty if function doesn't take any parameters
#[derive(Debug, PartialEq)]
pub struct FnParameter {
    ctype: CType,
    name: String,
}

impl FnParameter {
    pub fn new(ctype: CType, name: String) -> Self {
        Self { ctype, name }
    }
}
