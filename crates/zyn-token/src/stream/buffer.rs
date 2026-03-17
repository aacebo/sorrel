use crate::{ToStream, Token};

use super::Stream;

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

    pub fn freeze(self) -> Stream {
        Stream::from(self.0)
    }
}

impl From<Stream> for Buffer {
    fn from(value: Stream) -> Self {
        Self(value.into_iter().collect())
    }
}

impl From<Buffer> for Stream {
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

impl ToStream for Buffer {
    fn to_stream(self) -> Stream {
        self.freeze()
    }
}
