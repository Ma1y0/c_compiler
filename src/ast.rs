/// Node of the AST `Program` should always be at the top
#[derive(Debug, PartialEq)]
pub enum Node {
    Program(Vec<Statement>),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new(functions: Vec<Function>) -> Self {
        Self { functions }
    }
}

/// C function
#[derive(Debug, PartialEq)]
pub struct Function {
    return_type: Type,
    name: String,
    parameters: Vec<Parameter>,
    body: Vec<Statement>,
}

impl Function {
    pub fn new(
        return_type: Type,
        name: String,
        parameters: Vec<Parameter>,
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
pub struct Parameter {
    param_type: Type,
    name: String,
}

/// C types
#[derive(Debug, PartialEq)]
pub enum Type {
    Void,
    Int,
}

/// Statement
#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i64),
}
