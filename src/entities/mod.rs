use std::{
    collections::VecDeque,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{errors::RuntimeError, scope::Scope};

pub use expression::Expression;
pub use identifier::Identifier;
pub use list::List;
pub use primitive::Primitive;
use traits::AsType;
pub use value::Value;

mod expression;
mod identifier;
mod list;
mod primitive;
pub mod traits;
mod value;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Entity {
    Value(Value),
    Identifier(Identifier),
    Expression(Expression),
}

impl Entity {
    pub fn to_value(self, scope: &mut Scope) -> Result<Value, RuntimeError> {
        match self {
            Entity::Value(v) => Ok(v),
            Entity::Identifier(id) => Ok(id.value_from(scope)),
            Entity::Expression(e) => e.execute(scope),
        }
    }

    pub fn to_id(self) -> Result<Identifier, RuntimeError> {
        match self {
            Entity::Identifier(id) => Ok(id),
            _ => Err(RuntimeError::TypeMismatch {
                expected: Datatype::Identifier,
                found: self.as_type(),
            }),
        }
    }

    pub fn to_expression(self) -> Result<Expression, RuntimeError> {
        match self {
            Entity::Expression(e) => Ok(e),
            _ => Err(RuntimeError::TypeMismatch {
                expected: Datatype::Expression,
                found: self.as_type(),
            }),
        }
    }

    // pub fn into(self) -> Result<Self, RuntimeError> {
    //     if self.as_type() != Self
    // }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Datatype {
    Number,
    String,
    Bool,
    Nil,

    List,
    Object,

    Identifier,
    Expression,

    Primitive,
    Any, // Special
}
