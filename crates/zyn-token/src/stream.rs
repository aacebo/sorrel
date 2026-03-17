use crate::{Iter, Token, TokenBuffer};

pub trait Stream {
    fn stream(&self) -> TokenStream;
}

#[derive(Debug, Default, Clone)]
pub struct TokenStream(TokenBuffer);

impl std::ops::Deref for TokenStream {
    type Target = TokenBuffer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TokenBuffer> for TokenStream {
    fn from(value: TokenBuffer) -> Self {
        Self(value)
    }
}

impl From<TokenStream> for TokenBuffer {
    fn from(value: TokenStream) -> Self {
        value.0
    }
}

impl From<proc_macro2::TokenStream> for TokenStream {
    fn from(stream: proc_macro2::TokenStream) -> Self {
        Self(stream.into_iter().map(Token::from).collect())
    }
}

impl From<TokenStream> for proc_macro2::TokenStream {
    fn from(stream: TokenStream) -> Self {
        stream
            .into_iter()
            .map(proc_macro2::TokenTree::from)
            .collect()
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromIterator<Self> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        Self(iter.into_iter().flatten().collect())
    }
}

impl IntoIterator for TokenStream {
    type IntoIter = Iter;
    type Item = Token;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
