use std::fmt::Display;

use crate::{executer::execute, parser::ParseError};

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,

    List(Vec<Token>),
    Object(Vec<(Token, Token)>),

    Identifier(String),
    Expression(Vec<Token>),
}

impl Token {
    pub fn value(self) -> Result<String, ParseError> {
        match self {
            Self::Expression(_) => execute(self)?.value(),
            Self::Number(value) => Ok(value.to_string()),
            Self::String(value) | Self::Identifier(value) => Ok(value),
            Self::Bool(value) => Ok(value.to_string()),
            Self::Nil => Ok("Nil".to_string()),
            _ => todo!(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
