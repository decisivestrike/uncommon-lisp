use std::fmt::Display;

use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Primitive {
    pub fn to_value(self) -> Value {
        Value::Primitive(self)
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Primitive::Nil => true,
            _ => false,
        }
    }

    pub fn as_number(&self) -> Result<f64, RuntimeError> {
        match self {
            Primitive::Number(number) => Ok(*number),
            Primitive::String(str) => str.parse().map_err(|_| RuntimeError::TypeMismatch {
                expected: Datatype::Number,
                found: Datatype::String,
            }),
            Primitive::Bool(_) => todo!(),
            Primitive::Nil => todo!(),
        }
    }

    pub fn as_string(&self) -> String {
        self.to_string()
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Primitive::Number(number) => *number > 0.0,
            Primitive::String(string) => string.len() > 0,
            Primitive::Bool(bool) => *bool,
            Primitive::Nil => false,
        }
    }
}

impl ToEntity for Primitive {
    fn to_entity(self) -> Entity {
        Entity::Value(Value::Primitive(self))
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Primitive::Number(number) => number.to_string(),
            Primitive::String(string) => string.to_owned(),
            Primitive::Bool(bool) => bool.to_string(),
            Primitive::Nil => "nil".to_string(),
        };

        write!(f, "{}", output)
    }
}
