#![cfg_attr(nightly, feature(proc_macro_diagnostic, proc_macro_span))]

#[cfg(nightly)]
extern crate proc_macro;

mod delim;
mod error;
mod group;
mod ident;
mod iter;
mod literal;
mod punct;
mod spacing;
mod span;
mod stream;

#[cfg(feature = "report")]
pub mod report;

#[cfg(feature = "ast")]
pub mod ast;

pub use delim::*;
pub use error::*;
pub use group::*;
pub use ident::*;
pub use iter::*;
pub use literal::*;
pub use punct::*;
pub use spacing::*;
pub use span::*;
pub use stream::*;

pub trait Reader {
    /// the remaining token count.
    fn remaining(&self) -> usize;

    /// peek at the next token without moving forward.
    fn peek(&self) -> Option<&Token>;

    /// move the iterator forward by n and return the tokens.
    fn next_n(&mut self, n: usize) -> Option<&[Token]>;

    /// move the iterator forward and return the token.
    fn next(&mut self) -> Option<&Token> {
        self.next_n(1)?.first()
    }
}

pub trait Writer {
    type Error: Into<SpanError>;

    /// write tokens to a stream.
    fn write(&mut self, tokens: impl IntoIterator<Item = Token>) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone)]
pub enum Token {
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
    Group(Group),
}

impl Token {
    pub fn span(&self) -> Span {
        match self {
            Self::Ident(v) => v.span(),
            Self::Punct(v) => v.span(),
            Self::Literal(v) => v.span(),
            Self::Group(v) => v.span().into(),
        }
    }
}

impl From<Ident> for Token {
    fn from(value: Ident) -> Self {
        Self::Ident(value)
    }
}

impl From<Punct> for Token {
    fn from(value: Punct) -> Self {
        Self::Punct(value)
    }
}

impl From<Literal> for Token {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl From<Group> for Token {
    fn from(value: Group) -> Self {
        Self::Group(value)
    }
}

impl From<proc_macro2::TokenTree> for Token {
    fn from(value: proc_macro2::TokenTree) -> Self {
        match value {
            proc_macro2::TokenTree::Ident(v) => Ident::from(v).into(),
            proc_macro2::TokenTree::Punct(v) => Punct::from(v).into(),
            proc_macro2::TokenTree::Literal(v) => Literal::from(v).into(),
            proc_macro2::TokenTree::Group(v) => Group::from(v).into(),
        }
    }
}

impl From<Token> for proc_macro2::TokenTree {
    fn from(value: Token) -> Self {
        match value {
            Token::Ident(v) => proc_macro2::Ident::from(v).into(),
            Token::Punct(v) => proc_macro2::Punct::from(v).into(),
            Token::Literal(v) => proc_macro2::Literal::from(v).into(),
            Token::Group(v) => proc_macro2::Group::from(v).into(),
        }
    }
}

impl IntoIterator for Token {
    type Item = Token;
    type IntoIter = std::iter::Once<Token>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(v) => write!(f, "{}", v),
            Self::Punct(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        }
    }
}

impl ToStream for Token {
    fn to_stream(self) -> Stream {
        vec![self].into()
    }
}

impl ToStream for Ident {
    fn to_stream(self) -> Stream {
        Token::from(self).to_stream()
    }
}

impl ToStream for Punct {
    fn to_stream(self) -> Stream {
        Token::from(self).to_stream()
    }
}

impl ToStream for Literal {
    fn to_stream(self) -> Stream {
        Token::from(self).to_stream()
    }
}
