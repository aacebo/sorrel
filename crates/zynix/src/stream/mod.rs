mod buffer;
mod limit;
mod parse;

pub use buffer::*;
pub use limit::*;
pub use parse::*;

use std::str::FromStr;

use crate::{DelimSpan, ParseError, Span, Token};

/// An immutable collection of tokens.
/// On nightly, stays as `External` (native `proc_macro::TokenStream`) when running inside
/// a proc-macro invocation, normalizing to `Internal` (Vec<Token>) only when random access
/// is needed. On stable, always `Internal`.
#[derive(Clone)]
pub enum TokenStream {
    #[cfg(nightly)]
    External(proc_macro::TokenStream),
    Internal(Vec<Token>),
}

impl TokenStream {
    pub fn new() -> Self {
        Self::Internal(vec![])
    }

    /// Normalize to Internal, returning a mutable reference to the vec.
    pub fn normalize(&mut self) -> &mut Vec<Token> {
        #[cfg(nightly)]
        if let Self::External(ts) = self {
            let tokens: Vec<Token> = ts.clone().into_iter().map(Token::from).collect();
            *self = Self::Internal(tokens);
        }

        match self {
            #[cfg(nightly)]
            Self::External(_) => unreachable!(),
            Self::Internal(v) => v,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            #[cfg(nightly)]
            Self::External(ts) => ts.clone().into_iter().next().is_none(),
            Self::Internal(v) => v.is_empty(),
        }
    }

    pub fn len(&mut self) -> usize {
        self.normalize().len()
    }

    pub fn get(&mut self, index: usize) -> Option<&Token> {
        self.normalize().get(index)
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &Token> {
        self.normalize().iter()
    }

    pub fn first(&mut self) -> Span {
        self.normalize()
            .first()
            .map(|v| v.span())
            .unwrap_or_default()
    }

    pub fn last(&mut self) -> Span {
        self.normalize()
            .last()
            .map(|v| v.span())
            .unwrap_or_default()
    }

    pub fn span(&mut self) -> Span {
        self.first().join(self.last())
    }

    pub fn delim(&mut self) -> DelimSpan {
        DelimSpan::new(self.first(), self.last())
    }

    pub fn extend_one(&mut self, token: Token) {
        self.normalize().push(token);
    }

    pub fn parse(&mut self) -> ParseStream<'_> {
        ParseStream::new(self)
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::Internal(vec![])
    }
}

impl Extend<Token> for TokenStream {
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        self.normalize().extend(iter);
    }
}

impl std::ops::Deref for TokenStream {
    type Target = [Token];

    fn deref(&self) -> &[Token] {
        match self {
            #[cfg(nightly)]
            Self::External(_) => &[],
            Self::Internal(v) => v.as_slice(),
        }
    }
}

impl From<proc_macro2::TokenStream> for TokenStream {
    fn from(stream: proc_macro2::TokenStream) -> Self {
        #[cfg(nightly)]
        if proc_macro::is_available() {
            return Self::External(stream.into());
        }

        Self::Internal(stream.into_iter().map(Token::from).collect())
    }
}

impl From<TokenStream> for proc_macro2::TokenStream {
    fn from(mut stream: TokenStream) -> Self {
        #[cfg(nightly)]
        if let TokenStream::External(ts) = stream {
            return proc_macro2::TokenStream::from(ts);
        }

        stream
            .normalize()
            .iter()
            .cloned()
            .map(proc_macro2::TokenTree::from)
            .collect()
    }
}

#[cfg(nightly)]
impl From<proc_macro::TokenStream> for TokenStream {
    fn from(stream: proc_macro::TokenStream) -> Self {
        Self::External(stream)
    }
}

#[cfg(nightly)]
impl From<TokenStream> for proc_macro::TokenStream {
    fn from(stream: TokenStream) -> Self {
        match stream {
            TokenStream::External(ts) => ts,
            TokenStream::Internal(v) => v.into_iter().map(proc_macro::TokenTree::from).collect(),
        }
    }
}

impl From<&[Token]> for TokenStream {
    fn from(value: &[Token]) -> Self {
        Self::Internal(value.to_vec())
    }
}

impl From<Vec<Token>> for TokenStream {
    fn from(value: Vec<Token>) -> Self {
        Self::Internal(value)
    }
}

impl From<TokenStream> for Vec<Token> {
    fn from(value: TokenStream) -> Self {
        match value {
            #[cfg(nightly)]
            TokenStream::External(ts) => ts.into_iter().map(Token::from).collect(),
            TokenStream::Internal(v) => v,
        }
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self::Internal(iter.into_iter().collect())
    }
}

impl FromIterator<Self> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        Self::Internal(iter.into_iter().flat_map(|s| s.into_iter()).collect())
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = Box<dyn Iterator<Item = Token>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            #[cfg(nightly)]
            Self::External(ts) => Box::new(ts.into_iter().map(Token::from)),
            Self::Internal(v) => Box::new(v.into_iter()),
        }
    }
}

impl FromStr for TokenStream {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pm2 = proc_macro2::TokenStream::from_str(s)?;

        #[cfg(nightly)]
        if proc_macro::is_available() {
            return Ok(Self::External(pm2.into()));
        }

        Ok(Self::Internal(pm2.into_iter().map(Token::from).collect()))
    }
}

impl std::fmt::Debug for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(nightly)]
            Self::External(ts) => write!(f, "TokenStream::External({})", ts),
            Self::Internal(v) => write!(f, "TokenStream::Internal({:?})", v),
        }
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(nightly)]
            Self::External(ts) => write!(f, "{}", ts),
            Self::Internal(v) => {
                for token in v.iter() {
                    write!(f, "{}", token)?;
                }

                Ok(())
            }
        }
    }
}
