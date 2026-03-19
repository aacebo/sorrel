mod buffer;
mod delim;
pub(crate) mod fallback;
mod group;
mod ident;
mod iter;
mod limit;
mod literal;
mod punct;
mod spacing;
mod stream;

pub use buffer::*;
pub use delim::*;
pub use group::*;
pub use ident::*;
pub use iter::*;
pub use limit::*;
pub use literal::*;
pub use punct::*;
pub use spacing::*;
pub use stream::*;

use crate::{ParseError, Span};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceMap;
    use crate::span::fallback as span_fb;

    fn span(start: u32, end: u32) -> Span {
        SourceMap::with_mut(|sm| {
            if sm.is_empty() {
                sm.push("0123456789abcdef");
            }
        });
        Span::Fallback(span_fb::Span::new(start, end))
    }

    // --- Ident ---

    #[test]
    fn ident_new_and_name() {
        let id = Ident::new("foo", Span::default());
        assert_eq!(id.name().as_ref(), "foo");
    }

    #[test]
    fn ident_span_and_set_span() {
        let mut id = Ident::new("x", span(0, 1));
        assert_eq!(id.span().start().index(), 0);
        id.set_span(span(5, 6));
        assert_eq!(id.span().start().index(), 5);
    }

    #[test]
    fn ident_display() {
        let id = Ident::new("hello", Span::default());
        assert_eq!(format!("{}", id), "hello");
    }

    #[test]
    fn ident_fallback_roundtrip() {
        let id = Ident::new("bar", Span::default());
        let fb: fallback::Ident = id.clone().into();
        assert_eq!(fb.name().as_ref(), "bar");
        let back: Ident = fb.into();
        assert_eq!(back.name().as_ref(), "bar");
    }

    // --- Punct ---

    #[test]
    fn punct_new_and_accessors() {
        let p = Punct::new('+', Spacing::Alone);
        assert_eq!(p.as_char(), '+');
        assert_eq!(p.spacing(), Spacing::Alone);
    }

    #[test]
    fn punct_display() {
        let p = Punct::new(';', Spacing::Alone);
        assert_eq!(format!("{}", p), ";");
    }

    #[test]
    fn punct_fallback_roundtrip() {
        let p = Punct::new('!', Spacing::Joint);
        let fb: fallback::Punct = p.clone().into();
        assert_eq!(fb.as_char(), '!');
        assert_eq!(fb.spacing(), Spacing::Joint);
        let back: Punct = fb.into();
        assert_eq!(back.as_char(), '!');
    }

    // --- Literal ---

    #[test]
    fn literal_string() {
        let lit = Literal::string("hello");
        let s = format!("{}", lit);
        assert!(s.contains("hello"));
    }

    #[test]
    fn literal_integer() {
        let lit = Literal::u32_suffixed(42);
        let s = format!("{}", lit);
        assert!(s.contains("42"));
    }

    #[test]
    fn literal_fallback_roundtrip() {
        let lit = Literal::string("test");
        let fb: fallback::Literal = lit.clone().into();
        let back: Literal = fb.into();
        let s = format!("{}", back);
        assert!(s.contains("test"));
    }

    // --- Group ---

    #[test]
    fn group_new_and_delim() {
        let g = Group::new(Delim::Paren, TokenStream::new());
        assert_eq!(g.delim(), Delim::Paren);
    }

    #[test]
    fn group_as_tokens() {
        let mut ts = TokenStream::new();
        ts.extend_one(Ident::new("x", Span::default()).into());
        let g = Group::new(Delim::Brace, ts);
        assert!(!g.as_tokens().is_empty());
    }

    #[test]
    fn group_fallback_roundtrip() {
        let g = Group::new(Delim::Bracket, TokenStream::new());
        let fb: fallback::Group = g.clone().into();
        assert_eq!(fb.delim(), Delim::Bracket);
        let back: Group = fb.into();
        assert_eq!(back.delim(), Delim::Bracket);
    }

    // --- TokenStream ---

    #[test]
    fn token_stream_new_is_empty() {
        let ts = TokenStream::new();
        assert!(ts.is_empty());
    }

    #[test]
    fn token_stream_extend_one() {
        let mut ts = TokenStream::new();
        ts.extend_one(Ident::new("a", Span::default()).into());
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn token_stream_iter() {
        let mut ts = TokenStream::new();
        ts.extend_one(Ident::new("x", Span::default()).into());
        ts.extend_one(Punct::new('+', Spacing::Alone).into());
        let count = ts.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn token_stream_fallback_roundtrip() {
        let mut ts = TokenStream::new();
        ts.extend_one(Ident::new("y", Span::default()).into());
        let fb: fallback::TokenStream = ts.into();
        assert_eq!(fb.len(), 1);
        let back: TokenStream = fb.into();
        assert!(!back.is_empty());
    }

    #[test]
    fn token_stream_from_str() {
        use std::str::FromStr;
        let ts = TokenStream::from_str("fn main() {}").unwrap();
        assert!(!ts.is_empty());
    }

    // --- Token enum ---

    #[test]
    fn token_from_ident() {
        let t: Token = Ident::new("foo", Span::default()).into();
        assert!(matches!(t, Token::Ident(_)));
    }

    #[test]
    fn token_from_punct() {
        let t: Token = Punct::new('+', Spacing::Alone).into();
        assert!(matches!(t, Token::Punct(_)));
    }

    #[test]
    fn token_from_literal() {
        let t: Token = Literal::string("x").into();
        assert!(matches!(t, Token::Literal(_)));
    }

    #[test]
    fn token_from_group() {
        let t: Token = Group::new(Delim::Paren, TokenStream::new()).into();
        assert!(matches!(t, Token::Group(_)));
    }

    #[test]
    fn token_span() {
        let t: Token = Ident::new("x", span(3, 4)).into();
        assert_eq!(t.span().start().index(), 3);
    }

    #[test]
    fn token_display() {
        let t: Token = Ident::new("hello", Span::default()).into();
        assert_eq!(format!("{}", t), "hello");
    }
}
