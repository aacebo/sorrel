use crate::TokenTree;

use super::TokenStream;

/// A mutable collection of tokens
#[derive(Debug, Default, Clone)]
pub struct TokenBuffer(Vec<TokenTree>);

impl TokenBuffer {
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

impl From<TokenStream> for TokenBuffer {
    fn from(value: TokenStream) -> Self {
        Self(value.into())
    }
}

impl From<TokenBuffer> for TokenStream {
    fn from(value: TokenBuffer) -> Self {
        value.freeze()
    }
}

impl From<Vec<TokenTree>> for TokenBuffer {
    fn from(value: Vec<TokenTree>) -> Self {
        Self(value)
    }
}

impl From<&[TokenTree]> for TokenBuffer {
    fn from(value: &[TokenTree]) -> Self {
        Self(value.to_vec())
    }
}

impl From<TokenBuffer> for Vec<TokenTree> {
    fn from(value: TokenBuffer) -> Self {
        value.0
    }
}

impl FromIterator<TokenTree> for TokenBuffer {
    fn from_iter<T: IntoIterator<Item = TokenTree>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for TokenBuffer {
    type Item = TokenTree;
    type IntoIter = std::vec::IntoIter<TokenTree>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<TokenTree> for TokenBuffer {
    fn extend<T: IntoIterator<Item = TokenTree>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'a> Extend<&'a TokenTree> for TokenBuffer {
    fn extend<T: IntoIterator<Item = &'a TokenTree>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().cloned());
    }
}
