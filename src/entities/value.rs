use std::fmt::Display;

use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Primitive(Primitive),
    List(List),
    Object,
}

impl Value {
    pub fn to_list(self) -> Result<List, RuntimeError> {
        match self {
            Value::List(list) => Ok(list),
            _ => Err(RuntimeError::TypeMismatch {
                expected: Datatype::Identifier,
                found: self.as_type(),
            }),
        }
    }

    pub fn to_primitive(self) -> Result<Primitive, RuntimeError> {
        match self {
            Value::Primitive(primitive) => Ok(primitive),
            _ => Err(RuntimeError::TypeMismatch {
                expected: Datatype::Primitive,
                found: self.as_type(),
            }),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
