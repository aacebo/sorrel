#![cfg_attr(
    nightly,
    feature(proc_macro_diagnostic, proc_macro_span, proc_macro_totokens)
)]

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

pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);

    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }

    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
}

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
    type Error: Into<ParseError>;

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
            proc_macro2::TokenTree::Ident(v) => Self::Ident(v.into()),
            proc_macro2::TokenTree::Punct(v) => Self::Punct(v.into()),
            proc_macro2::TokenTree::Literal(v) => Self::Literal(v.into()),
            proc_macro2::TokenTree::Group(v) => Self::Group(v.into()),
        }
    }
}

impl From<Token> for proc_macro2::TokenTree {
    fn from(value: Token) -> Self {
        match value {
            Token::Ident(v) => proc_macro2::TokenTree::Ident(v.into()),
            Token::Punct(v) => proc_macro2::TokenTree::Punct(v.into()),
            Token::Literal(v) => proc_macro2::TokenTree::Literal(v.into()),
            Token::Group(v) => proc_macro2::TokenTree::Group(v.into()),
        }
    }
}

#[cfg(nightly)]
impl From<proc_macro::TokenTree> for Token {
    fn from(value: proc_macro::TokenTree) -> Self {
        match value {
            proc_macro::TokenTree::Ident(v) => Self::Ident(v.into()),
            proc_macro::TokenTree::Punct(v) => Self::Punct(v.into()),
            proc_macro::TokenTree::Literal(v) => Self::Literal(v.into()),
            proc_macro::TokenTree::Group(v) => Self::Group(v.into()),
        }
    }
}

#[cfg(nightly)]
impl From<Token> for proc_macro::TokenTree {
    fn from(value: Token) -> Self {
        match value {
            Token::Ident(v) => proc_macro::TokenTree::Ident(v.into()),
            Token::Punct(v) => proc_macro::TokenTree::Punct(v.into()),
            Token::Literal(v) => proc_macro::TokenTree::Literal(v.into()),
            Token::Group(v) => proc_macro::TokenTree::Group(v.into()),
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

// On stable: explicit ToTokens impls for zynix's own types.
// On nightly: blanket ToTokens for any T: proc_macro::ToTokens covers everything,
//   so zynix types only need proc_macro::ToTokens impls (defined further below).

#[cfg(not(nightly))]
impl ToTokens for Token {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(self.clone());
    }
}

#[cfg(not(nightly))]
impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(self.clone().into());
    }
}

#[cfg(not(nightly))]
impl ToTokens for Punct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(self.clone().into());
    }
}

#[cfg(not(nightly))]
impl ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(self.clone().into());
    }
}

#[cfg(not(nightly))]
impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.iter().cloned());
    }
}

#[cfg(not(nightly))]
impl ToTokens for &str {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use std::str::FromStr;
        if let Ok(ts) = TokenStream::from_str(self) {
            ts.to_tokens(tokens);
        }
    }
}

// On nightly: blanket covers any T: proc_macro::ToTokens, including zynix types
// (which implement proc_macro::ToTokens below).
#[cfg(nightly)]
impl<T: proc_macro::ToTokens> ToTokens for T {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut ts = proc_macro::TokenStream::new();
        proc_macro::ToTokens::to_tokens(self, &mut ts);
        tokens.extend(TokenStream::from(ts));
    }
}

// proc_macro::ToTokens impls for zynix types (nightly only).
// These feed into the blanket impl above.
#[cfg(nightly)]
impl proc_macro::ToTokens for Token {
    fn to_tokens(&self, tokens: &mut proc_macro::TokenStream) {
        let tt: proc_macro::TokenTree = self.clone().into();
        tokens.extend(std::iter::once(tt));
    }
}

#[cfg(nightly)]
impl proc_macro::ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut proc_macro::TokenStream) {
        match self {
            TokenStream::External(ts) => tokens.extend(ts.clone()),
            TokenStream::Internal(_) => {
                let pm: proc_macro::TokenStream = self.clone().into();
                tokens.extend(pm);
            }
        }
    }
}
