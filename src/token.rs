use std::{collections::VecDeque, fmt::Display};

use crate::{errors::RuntimeError, executer::execute, extractor::Extractable, scope::Scope};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Token {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,

    List(VecDeque<Token>),
    // Object(VecDeque<(Token, Token)>),
    Identifier(String),
    Expression(VecDeque<Token>),
}

impl Token {
    // Expression / Id -> Num / Str / Bool / Nil / List
    pub fn to_value(self, scope: &mut Scope) -> Result<Token, RuntimeError> {
        Ok(match self {
            Token::Identifier(_) => scope.get_variable(&self.to_string()),
            Token::Expression(_) => execute(self, scope)?,
            _ => self,
        })
    }

    pub fn as_type(&self) -> &str {
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
    }

    pub fn evaluate<T: Extractable>(mut self, scope: &mut Scope) -> Result<T, RuntimeError> {
        if matches!(self, Token::Identifier(_) | Token::Expression(_)) {
            self = self.to_value(scope)?;
        }

        T::extract(self, scope)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Expression(tokens) => format!(
                "({})",
                tokens
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Self::Number(v) => v.to_string(),
            Self::String(v) | Self::Identifier(v) => v.to_string(),
            Self::Bool(v) => v.to_string(),
            Self::Nil => "nil".to_string(),
            Self::List(items) => {
                let mut result = Vec::new();

                for item in items.into_iter() {
                    result.push(item.to_string());
                }

                result.join(" ").to_string()
            }
        };

        write!(f, "{}", output)
    }
}
