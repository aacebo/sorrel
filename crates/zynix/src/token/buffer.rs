use crate::TokenTree;

use super::TokenStream;

/// A mutable collection of tokens
#[derive(Debug, Default, Clone)]
pub struct Buffer(Vec<TokenTree>);

impl Buffer {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, token: TokenTree) {
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

impl From<Vec<TokenTree>> for Buffer {
    fn from(value: Vec<TokenTree>) -> Self {
        Self(value)
    }
}

impl From<&[TokenTree]> for Buffer {
    fn from(value: &[TokenTree]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Buffer> for Vec<TokenTree> {
    fn from(value: Buffer) -> Self {
        value.0
    }
}

impl FromIterator<TokenTree> for Buffer {
    fn from_iter<T: IntoIterator<Item = TokenTree>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Buffer {
    type Item = TokenTree;
    type IntoIter = std::vec::IntoIter<TokenTree>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<TokenTree> for Buffer {
    fn extend<T: IntoIterator<Item = TokenTree>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'a> Extend<&'a TokenTree> for Buffer {
    fn extend<T: IntoIterator<Item = &'a TokenTree>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().cloned());
    }
}
