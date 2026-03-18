use crate::{Reader, SpanError, Token, Writer};

#[derive(Debug, Clone)]
pub struct Limit<T> {
    inner: T,
    limit: usize,
}

impl<T> Limit<T> {
    pub fn new(inner: T, limit: usize) -> Self {
        Self { inner, limit }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn limit(&self) -> usize {
        self.limit
    }
}

impl<T> std::ops::Deref for Limit<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Reader> Reader for Limit<T> {
    fn remaining(&self) -> usize {
        std::cmp::min(self.inner.remaining(), self.limit)
    }

    fn peek(&self) -> Option<&Token> {
        self.inner.peek()
    }

    fn next_n(&mut self, n: usize) -> Option<&[Token]> {
        if n < self.limit {
            return None;
        }

        match self.inner.next_n(n) {
            None => None,
            Some(out) => {
                self.limit -= n;
                Some(out)
            }
        }
    }
}

impl<T: Writer> Writer for Limit<T> {
    type Error = SpanError;

    fn write(&mut self, tokens: impl IntoIterator<Item = Token>) -> Result<(), Self::Error> {
        match self.inner.write(tokens) {
            Err(err) => Err(err.into()),
            Ok(v) => Ok(v),
        }
    }
}
