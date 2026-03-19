use crate::Token;

use super::TokenStream;

/// A mutable collection of tokens
#[derive(Debug, Default, Clone)]
pub struct Buffer(Vec<Token>);

impl Buffer {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, token: Token) {
        self.0.push(token);
    }

    pub fn freeze(self) -> TokenStream {
        TokenStream::from(self.0)
    }
}

impl From<TokenStream> for Buffer {
    fn from(value: TokenStream) -> Self {
        Self(value.into())
    }
}

impl From<Buffer> for TokenStream {
    fn from(value: Buffer) -> Self {
        value.freeze()
    }
}

impl From<Vec<Token>> for Buffer {
    fn from(value: Vec<Token>) -> Self {
        Self(value)
    }
}

impl From<&[Token]> for Buffer {
    fn from(value: &[Token]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Buffer> for Vec<Token> {
    fn from(value: Buffer) -> Self {
        value.0
    }
}

impl FromIterator<Token> for Buffer {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Buffer {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<Token> for Buffer {
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'a> Extend<&'a Token> for Buffer {
    fn extend<T: IntoIterator<Item = &'a Token>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().cloned());
    }
}
