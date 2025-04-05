use std::fmt::Display;

use crate::{errors::RuntimeError, extractor::Extractable, scope::get_variable};

pub use expression::Expression;
pub use identifier::Identifier;
pub use list::List;

mod expression;
mod identifier;
mod list;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,

    List(List),
    // Object(Hashmap<Token, Token>),
    Identifier(Identifier),
    Expression(Expression),
}

impl Token {
    pub fn into_value(self, maybe_prefix: Option<String>) -> Result<Token, RuntimeError> {
        Ok(match self {
            Token::Identifier(id) => {
                let variable_name = match maybe_prefix {
                    Some(prefix) => prefix + &id.0,
                    None => id.0,
                };
                get_variable(&variable_name)
            }
            Token::Expression(e) => e.execute(maybe_prefix)?,
            _ => self,
        })
    }

    pub fn as_type(&self) -> String {
        match self {
            Token::Number(_) => "number",
            Token::String(_) => "string",
            Token::Bool(_) => "bool",
            Token::Nil => "nil",
            Token::List(_) => "list",
            // ULispType::Object => "object",
            Token::Identifier(_) => "identifier",
            Token::Expression(_) => "expression",
        }
        .to_string()
    }

    pub fn extract<T: Extractable>(self, maybe_prefix: Option<String>) -> Result<T, RuntimeError> {
        // if matches!(self, Token::Identifier(_) | Token::Expression(_)) {
        //     self = self.into_value()?;
        // }

        T::extract(self, maybe_prefix)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Number(v) => v.to_string(),
            Self::String(v) => v.to_string(),
            Self::Identifier(v) => v.to_string(),
            Self::Bool(v) => v.to_string(),
            Self::Nil => "nil".to_string(),
            Self::List(list) => list.to_string(),
            Self::Expression(exp) => exp.to_string(),
        };

        write!(f, "{}", output)
    }
}
