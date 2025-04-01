use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct List(pub VecDeque<Entity>);

impl List {
    pub fn new() -> Self {
        List(VecDeque::new())
    }

    pub fn from<C, I>(iterable: C) -> Self
    where
        C: IntoIterator<Item = I>,
        I: ToEntity,
    {
        List(iterable.into_iter().map(|e| e.to_entity()).collect())
    }

    pub fn get(&mut self) -> Entity {
        self.pop_front().unwrap()
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
