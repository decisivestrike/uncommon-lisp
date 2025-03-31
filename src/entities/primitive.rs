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
}

impl ToEntity for Primitive {
    fn to_entity(self) -> Entity {
        Entity::Value(Value::Primitive(self))
    }
}
