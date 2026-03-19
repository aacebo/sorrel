use std::str::FromStr;

use super::fallback;
use crate::{DelimSpan, ParseError, ParseStream, Span, Token};

#[derive(Clone)]
pub enum TokenStream {
    Compiler(proc_macro::TokenStream),
    Fallback(fallback::TokenStream),
}

impl TokenStream {
    pub fn new() -> Self {
        Self::Fallback(fallback::TokenStream::new())
    }

    /// Normalize to Fallback, returning a mutable reference to the vec.
    pub fn normalize(&mut self) -> &mut Vec<Token> {
        if let Self::Compiler(ts) = self {
            let tokens: Vec<Token> = ts.clone().into_iter().map(Token::from).collect();
            *self = Self::Fallback(fallback::TokenStream(tokens));
        }

        match self {
            Self::Compiler(_) => unreachable!(),
            Self::Fallback(v) => v.inner_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Compiler(ts) => ts.clone().into_iter().next().is_none(),
            Self::Fallback(v) => v.is_empty(),
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
        Self::Fallback(fallback::TokenStream::new())
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
            Self::Compiler(_) => &[],
            Self::Fallback(v) => v,
        }
    }
}

impl From<proc_macro2::TokenStream> for TokenStream {
    fn from(stream: proc_macro2::TokenStream) -> Self {
        if proc_macro::is_available() {
            return Self::Compiler(stream.into());
        }

        Self::Fallback(stream.into_iter().map(Token::from).collect())
    }
}

impl From<TokenStream> for proc_macro2::TokenStream {
    fn from(mut stream: TokenStream) -> Self {
        if let TokenStream::Compiler(ts) = stream {
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

impl From<proc_macro::TokenStream> for TokenStream {
    fn from(stream: proc_macro::TokenStream) -> Self {
        Self::Compiler(stream)
    }
}

impl From<TokenStream> for proc_macro::TokenStream {
    fn from(stream: TokenStream) -> Self {
        match stream {
            TokenStream::Compiler(ts) => ts,
            TokenStream::Fallback(v) => v.into_iter().map(proc_macro::TokenTree::from).collect(),
        }
    }
}

impl From<&[Token]> for TokenStream {
    fn from(value: &[Token]) -> Self {
        Self::Fallback(fallback::TokenStream(value.to_vec()))
    }
}

impl From<Vec<Token>> for TokenStream {
    fn from(value: Vec<Token>) -> Self {
        Self::Fallback(fallback::TokenStream(value))
    }
}

impl From<TokenStream> for Vec<Token> {
    fn from(value: TokenStream) -> Self {
        match value {
            TokenStream::Compiler(ts) => ts.into_iter().map(Token::from).collect(),
            TokenStream::Fallback(v) => v.into_iter().collect(),
        }
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self::Fallback(iter.into_iter().collect())
    }
}

impl FromIterator<Self> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        Self::Fallback(iter.into_iter().flat_map(|s| s.into_iter()).collect())
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = Box<dyn Iterator<Item = Token>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Compiler(ts) => Box::new(ts.into_iter().map(Token::from)),
            Self::Fallback(v) => Box::new(v.into_iter()),
        }
    }
}

impl FromStr for TokenStream {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pm2 = proc_macro2::TokenStream::from_str(s)?;

        if proc_macro::is_available() {
            return Ok(Self::Compiler(pm2.into()));
        }

        Ok(Self::Fallback(pm2.into_iter().map(Token::from).collect()))
    }
}

impl std::fmt::Debug for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler(ts) => write!(f, "TokenStream::Compiler({})", ts),
            Self::Fallback(v) => write!(f, "TokenStream::Fallback({:?})", v),
        }
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler(ts) => write!(f, "{}", ts),
            Self::Fallback(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(nightly)]
impl proc_macro::ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut proc_macro::TokenStream) {
        match self {
            Self::Compiler(ts) => tokens.extend(ts.clone()),
            Self::Fallback(_) => {
                let pm: proc_macro::TokenStream = self.clone().into();
                tokens.extend(pm);
            }
        }
    }
}
