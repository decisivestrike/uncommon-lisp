use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash, Default)]
pub struct Identifier(pub String);

// impl Identifier {
//     pub fn new(inner: &str) -> Self {
//         Self(inner.to_string())
//     }

//     pub fn value_from(&self) -> Token {
//         get_variable(&self.0)
//     }
// }

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

// impl TryFrom<Token> for Identifier {
//     type Error = RuntimeError;

//     fn try_from(source: Token) -> Result<Self, Self::Error> {
//         source.extract()
//     }
// }
