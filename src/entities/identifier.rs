use crate::extractor::Extractable;

use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn new(inner: &str) -> Self {
        Self(inner.to_string())
    }

    pub fn value_from(&self, scope: &mut Scope) -> Value {
        scope.get_variable(self)
    }
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToEntity for Identifier {
    fn to_entity(self) -> Entity {
        Entity::Identifier(self)
    }
}

impl Extractable for Identifier {
    fn extract(token: Entity, scope: &mut Scope) -> Result<Self, RuntimeError> {
        match token {
            Entity::Identifier(i) => Ok(i),
            e => Err(RuntimeError::TypeMismatch {
                expected: Datatype::Identifier,
                found: e.as_type(),
            }),
        }
    }
}
