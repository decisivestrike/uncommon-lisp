use std::fmt::Display;

use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Primitive(Primitive),
    List(List),
    Object,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
