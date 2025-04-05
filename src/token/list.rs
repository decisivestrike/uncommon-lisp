use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use super::Token;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct List(pub VecDeque<Token>);

impl List {
    // pub fn new() -> Self {
    //     List(VecDeque::new())
    // }

    pub fn from_iterable<T>(iterable: T) -> Self
    where
        T: IntoIterator<Item = Token>,
    {
        List(iterable.into_iter().collect())
    }

    // pub fn get(&mut self) -> Token {
    //     self.pop_front().unwrap()
    // }
}

impl Deref for List {
    type Target = VecDeque<Token>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
