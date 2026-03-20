use std::str::FromStr;

use super::fallback;
use crate::{ParseError, ParseStream, ToTokens, Token};

#[derive(Clone)]
pub enum TokenStream {
    Compiler(proc_macro::TokenStream),
    Fallback(fallback::TokenStream),
}

impl TokenStream {
    pub fn new() -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::TokenStream::new())
        } else {
            Self::Fallback(fallback::TokenStream::new())
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Compiler(ts) => ts.clone().into_iter().next().is_none(),
            Self::Fallback(v) => v.is_empty(),
        }
    }

    pub fn extend_one(&mut self, token: Token) {
        match self {
            Self::Compiler(v) => v.extend_one(proc_macro::TokenTree::from(token)),
            Self::Fallback(v) => v.extend_one(token),
        }
    }

    pub fn parse(&self) -> ParseStream<'_> {
        ParseStream::new(self)
    }

    pub fn to_vec(self) -> Vec<Token> {
        match self {
            Self::Compiler(v) => v.into_iter().map(Token::from).collect(),
            Self::Fallback(v) => v.0,
        }
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::Fallback(fallback::TokenStream::new())
    }
}

impl Extend<Token> for TokenStream {
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        match self {
            Self::Compiler(v) => v.extend(iter.into_iter().map(proc_macro::TokenTree::from)),
            Self::Fallback(v) => v.extend(iter),
        }
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

impl From<fallback::TokenStream> for TokenStream {
    fn from(value: fallback::TokenStream) -> Self {
        Self::Fallback(value)
    }
}

impl From<TokenStream> for fallback::TokenStream {
    fn from(value: TokenStream) -> Self {
        match value {
            TokenStream::Compiler(_) => {
                let tokens: Vec<Token> = value.to_vec();
                fallback::TokenStream(tokens)
            }
            TokenStream::Fallback(v) => v,
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
    type IntoIter = super::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Compiler(ts) => ts.into_iter().into(),
            Self::Fallback(v) => v.into_iter().into(),
        }
    }
}

impl FromStr for TokenStream {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if proc_macro::is_available() {
            let pm = s.parse().map_err(ParseError::from)?;
            return Ok(Self::Compiler(pm));
        }

        Ok(Self::Fallback(s.parse()?))
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

impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Compiler(v) => tokens.extend(v.clone().into_iter().map(Token::from)),
            Self::Fallback(v) => v.to_tokens(tokens),
        }
    }
}
