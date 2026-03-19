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
    fn to_tokens(self) -> TokenStream;
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

// On stable: explicit ToTokens impls for zynix's own types.
// On nightly: blanket ToTokens for any T: proc_macro::ToTokens covers everything,
//   so zynix types only need proc_macro::ToTokens impls (defined further below).

#[cfg(not(nightly))]
impl ToTokens for Token {
    fn to_tokens(self) -> TokenStream {
        vec![self].into()
    }
}

#[cfg(not(nightly))]
impl ToTokens for Ident {
    fn to_tokens(self) -> TokenStream {
        Token::from(self).to_tokens()
    }
}

#[cfg(not(nightly))]
impl ToTokens for Punct {
    fn to_tokens(self) -> TokenStream {
        Token::from(self).to_tokens()
    }
}

#[cfg(not(nightly))]
impl ToTokens for Literal {
    fn to_tokens(self) -> TokenStream {
        Token::from(self).to_tokens()
    }
}

#[cfg(not(nightly))]
impl ToTokens for TokenStream {
    fn to_tokens(self) -> TokenStream {
        self
    }
}

#[cfg(not(nightly))]
impl ToTokens for proc_macro2::TokenStream {
    fn to_tokens(self) -> TokenStream {
        self.into()
    }
}

#[cfg(not(nightly))]
impl ToTokens for &str {
    fn to_tokens(self) -> TokenStream {
        use std::str::FromStr;
        TokenStream::from_str(self).unwrap_or_default()
    }
}

// On nightly: blanket covers any T: proc_macro::ToTokens, including zynix types
// (which implement proc_macro::ToTokens in their respective modules).
#[cfg(nightly)]
impl<T: proc_macro::ToTokens> ToTokens for T {
    fn to_tokens(self) -> TokenStream {
        let mut ts = proc_macro::TokenStream::new();
        proc_macro::ToTokens::to_tokens(&self, &mut ts);
        proc_macro2::TokenStream::from(ts).into()
    }
}

// proc_macro::ToTokens impls for zynix types (nightly only).
// These feed into the blanket impl above.
#[cfg(nightly)]
impl proc_macro::ToTokens for Token {
    fn to_tokens(&self, tokens: &mut proc_macro::TokenStream) {
        let pm2: proc_macro2::TokenStream = std::iter::once(self.clone())
            .map(proc_macro2::TokenTree::from)
            .collect();
        tokens.extend(proc_macro::TokenStream::from(pm2));
    }
}

#[cfg(nightly)]
impl proc_macro::ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut proc_macro::TokenStream) {
        let pm2: proc_macro2::TokenStream = self.clone().into();
        tokens.extend(proc_macro::TokenStream::from(pm2));
    }
}
