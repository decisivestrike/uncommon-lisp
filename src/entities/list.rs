use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct List(pub VecDeque<Entity>);

impl List {
    pub fn new() -> Self {
        List(VecDeque::new())
    }
}

impl Deref for List {
    type Target = VecDeque<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToEntity for List {
    fn to_entity(self) -> Entity {
        Entity::Value(Value::List(self))
    }
}
