use std::fmt::Display;

use crate::{errors::RuntimeError, executer::execute};

#[derive(Clone, Debug, PartialEq)]
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
    pub fn value(self) -> Result<String, RuntimeError> {
        match self {
            Self::Expression(_) => execute(self)?.value(),
            Self::Number(value) => Ok(value.to_string()),
            Self::String(value) | Self::Identifier(value) => Ok(value),
            Self::Bool(value) => Ok(value.to_string()),
            Self::Nil => Ok("Nil".to_string()),
            Self::List(items) => {
                let mut result = Vec::new();

                for item in items.into_iter() {
                    result.push(item.value()?);
                }

                Ok(result.join(" ").to_string())
            }
            Self::Object(fields) => {
                let mut result = Vec::new();

                for (key, value) in fields {
                    result.push(format!("{}:{}\n", key.value()?, value.value()?));
                }

                Ok(result.join(" ").to_string())
            }
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
