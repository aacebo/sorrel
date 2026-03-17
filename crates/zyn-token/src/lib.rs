pub use proc_macro2::{Ident, Literal, Punct, Spacing};

mod buffer;
mod delim;
mod error;
mod group;
mod iter;
// pub mod parse;
mod span;
mod stream;

pub use buffer::*;
pub use delim::*;
pub use error::*;
pub use group::*;
pub use iter::*;
pub use span::*;
pub use stream::*;

pub trait Syntax: Stream {
    fn span(&self) -> Span;
}

pub trait TokenReader {
    fn peek(&mut self) -> Option<&Token>;
    fn next(&mut self) -> Option<Token>;
    fn fork(&self) -> Self where Self: Sized;
}

pub trait TokenWriter {
    fn write(&mut self, token: Token) -> Result<()>;
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
            Self::Ident(v) => v.span().into(),
            Self::Punct(v) => v.span().into(),
            Self::Literal(v) => v.span().into(),
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
            proc_macro2::TokenTree::Ident(v) => v.into(),
            proc_macro2::TokenTree::Punct(v) => v.into(),
            proc_macro2::TokenTree::Literal(v) => v.into(),
            proc_macro2::TokenTree::Group(v) => Group::from(v).into(),
        }
    }
}

impl From<Token> for proc_macro2::TokenTree {
    fn from(value: Token) -> Self {
        match value {
            Token::Ident(v) => v.into(),
            Token::Punct(v) => v.into(),
            Token::Literal(v) => v.into(),
            Token::Group(v) => proc_macro2::Group::from(v).into(),
        }
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
