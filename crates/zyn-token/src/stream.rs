use std::str::FromStr;

use crate::{DelimSpan, Span, SpanError, Token, TokenBuffer};

pub trait Stream {
    fn stream(&self) -> TokenStream;
}

#[derive(Debug, Default, Clone)]
pub struct TokenStream(Vec<Token>);

impl TokenStream {
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

    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.0.iter()
    }

    pub fn first(&self) -> Span {
        self.0
            .first()
            .map(|v| v.span())
            .unwrap_or(Span::call_site())
    }

    pub fn last(&self) -> Span {
        self.0.last().map(|v| v.span()).unwrap_or(Span::call_site())
    }

    pub fn span(&self) -> Span {
        self.first().join(self.last()).unwrap_or(Span::call_site())
    }

    pub fn delim(&self) -> DelimSpan {
        DelimSpan::new(self.first(), self.last())
    }
}

impl From<TokenBuffer> for TokenStream {
    fn from(value: TokenBuffer) -> Self {
        Self(value.into_iter().collect())
    }
}

impl From<TokenStream> for TokenBuffer {
    fn from(value: TokenStream) -> Self {
        TokenBuffer::from(value.0)
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

impl From<&[Token]> for TokenStream {
    fn from(value: &[Token]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Vec<Token>> for TokenStream {
    fn from(value: Vec<Token>) -> Self {
        Self(value)
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
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for TokenStream {
    type Err = SpanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stream = proc_macro2::TokenStream::from_str(s)?;
        Ok(stream.into())
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.0.iter() {
            write!(f, "{}", token)?;
        }
        Ok(())
    }
}

impl Stream for proc_macro2::TokenStream {
    fn stream(&self) -> TokenStream {
        self.clone().into()
    }
}

impl Stream for TokenStream {
    fn stream(&self) -> TokenStream {
        self.clone()
    }
}

impl Stream for &str {
    fn stream(&self) -> TokenStream {
        TokenStream::from_str(self).unwrap_or_default()
    }
}
