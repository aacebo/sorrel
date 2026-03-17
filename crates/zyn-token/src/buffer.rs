use crate::Token;

#[derive(Debug, Default, Clone)]
pub struct TokenBuffer(Vec<Token>);

impl TokenBuffer {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        self.0.get(index)
    }

    pub fn push(&mut self, token: Token) {
        self.0.push(token);
    }
}

impl From<Vec<Token>> for TokenBuffer {
    fn from(value: Vec<Token>) -> Self {
        Self(value)
    }
}

impl From<&[Token]> for TokenBuffer {
    fn from(value: &[Token]) -> Self {
        Self(value.to_vec())
    }
}

impl From<TokenBuffer> for Vec<Token> {
    fn from(value: TokenBuffer) -> Self {
        value.0
    }
}

impl FromIterator<Token> for TokenBuffer {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for TokenBuffer {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<Token> for TokenBuffer {
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'a> Extend<&'a Token> for TokenBuffer {
    fn extend<T: IntoIterator<Item = &'a Token>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().cloned());
    }
}
