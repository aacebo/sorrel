use std::ops::{Deref, DerefMut};

use crate::{AsStream, ToStream, Token};

use super::Stream;

/// A mutable collection of tokens
#[derive(Debug, Default, Clone)]
pub struct Buffer(Stream);

impl Buffer {
    pub fn new() -> Self {
        Self(Stream::new())
    }

    pub fn push(&mut self, token: Token) {
        self.0.write(std::iter::once(token));
    }

    pub fn freeze(self) -> Stream {
        self.0
    }
}

impl Deref for Buffer {
    type Target = Stream;

    fn deref(&self) -> &Stream {
        &self.0
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Stream {
        &mut self.0
    }
}

impl From<Vec<Token>> for Buffer {
    fn from(value: Vec<Token>) -> Self {
        Self(Stream::from(value))
    }
}

impl From<&[Token]> for Buffer {
    fn from(value: &[Token]) -> Self {
        Self(Stream::from(value))
    }
}

impl From<Buffer> for Vec<Token> {
    fn from(value: Buffer) -> Self {
        value.freeze().into_iter().collect()
    }
}

impl FromIterator<Token> for Buffer {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self(Stream::from_iter(iter))
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
        self.0.write(iter);
    }
}

impl<'a> Extend<&'a Token> for Buffer {
    fn extend<T: IntoIterator<Item = &'a Token>>(&mut self, iter: T) {
        self.0.write(iter.into_iter().cloned());
    }
}

impl ToStream for Buffer {
    fn to_stream(self) -> Stream {
        self.0
    }
}

impl AsStream for Buffer {
    fn as_stream(&self) -> &Stream {
        &self.0
    }
}
