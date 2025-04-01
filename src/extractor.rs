use crate::{
    entities::{Datatype, Entity, Primitive, ToEntity, Value},
    errors::RuntimeError,
    scope::Scope,
};

pub trait Extractable: Sized {
    fn extract(source: Entity, scope: &mut Scope) -> Result<Self, RuntimeError>;
}

impl Extractable for f64 {
    fn extract(source: Entity, scope: &mut Scope) -> Result<Self, RuntimeError> {
        match source {
            Entity::Value(value) => match value {
                Value::Primitive(primitive) => match primitive {
                    Primitive::Number(number) => Ok(number),
                    Primitive::String(string) => Ok(string.parse()),
                    Primitive::Bool(b) => Ok(b as i32 as f64),
                    Primitive::Nil => todo!(),
                },

                v => Err(RuntimeError::TypeMismatch {
                    expected: Datatype::Number,
                    found: v.as_type(),
                }),
            },

            Entity::Expression(e) => Self::extract(e.execute(scope)?.to_entity(), scope),
            Entity::Identifier(id) => Self::extract(scope.get_variable(&name).to_entity(), scope),
        }
    }
}

impl Extractable for String {
    fn extract(token: Entity, scope: &mut Scope) -> Result<Self, RuntimeError> {
        match token {
            Token::Identifier(name) => Self::extract(scope.get_variable(&name), scope),
            Token::Expression(_) => Self::extract(execute(token, scope)?, scope),
            _ => Ok(token.to_string()),
        }
    }
}

impl Extractable for bool {
    fn extract(token: Entity, scope: &mut Scope) -> Result<Self, RuntimeError> {
        match token {
            Token::Number(v) => Ok(v != 0.0),
            Token::String(v) => Ok(v.len() > 0),
            Token::Bool(value) => Ok(value),
            Token::Nil => Ok(false),
            Token::Identifier(name) => Self::extract(scope.get_variable(&name), scope),
            Token::Expression(_) => Self::extract(execute(token, scope)?, scope),
            _ => Err(RuntimeError::TypeMismatch {
                expected: ULispType::Number,
                found: token.as_type(),
            }),
        }
    }
}

pub fn evaluate<T: Extractable>(source: Entity, scope: &mut Scope) -> Result<T, RuntimeError> {
    T::extract(source, scope)
}
