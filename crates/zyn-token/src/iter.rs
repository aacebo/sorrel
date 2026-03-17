use crate::Token;

#[derive(Debug, Clone)]
pub struct Iter(std::vec::IntoIter<Token>);

impl From<std::vec::IntoIter<Token>> for Iter {
    fn from(value: std::vec::IntoIter<Token>) -> Self {
        Self(value)
    }
}

impl From<Vec<Token>> for Iter {
    fn from(value: Vec<Token>) -> Self {
        Self(value.into_iter())
    }
}

impl Iterator for Iter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
