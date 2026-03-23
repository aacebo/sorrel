mod buffer;
mod delim;
pub(crate) mod fallback;
mod group;
mod ident;
mod iter;
pub mod lex;
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
pub use lex::{LexError, Scan};
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
    fn peek(&self) -> Option<&TokenTree>;

    /// move the iterator forward by n and return the tokens.
    fn next_n(&mut self, n: usize) -> Option<&[TokenTree]>;

    /// move the iterator forward and return the token.
    fn next(&mut self) -> Option<&TokenTree> {
        self.next_n(1)?.first()
    }
}

pub trait Writer {
    type Error: Into<ParseError>;

    /// write tokens to a stream.
    fn write(&mut self, tokens: impl IntoIterator<Item = TokenTree>) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone)]
pub enum Token {
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}

impl Token {
    pub fn span(&self) -> Span {
        match self {
            Self::Ident(v) => v.span(),
            Self::Punct(v) => v.span(),
            Self::Literal(v) => v.span(),
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(v) => write!(f, "{}", v),
            Self::Punct(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
        }
    }
}

impl ToTokens for Token {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(TokenTree::from(self.clone()));
    }
}

#[derive(Debug, Clone)]
pub enum TokenTree {
    Token(Token),
    Group(Group),
}

impl TokenTree {
    pub fn span(&self) -> Span {
        match self {
            Self::Token(v) => v.span(),
            Self::Group(v) => v.span().into(),
        }
    }
}

impl From<Token> for TokenTree {
    fn from(value: Token) -> Self {
        Self::Token(value)
    }
}

impl From<Ident> for TokenTree {
    fn from(value: Ident) -> Self {
        Self::Token(Token::from(value))
    }
}

impl From<Punct> for TokenTree {
    fn from(value: Punct) -> Self {
        Self::Token(Token::from(value))
    }
}

impl From<Literal> for TokenTree {
    fn from(value: Literal) -> Self {
        Self::Token(Token::from(value))
    }
}

impl From<Group> for TokenTree {
    fn from(value: Group) -> Self {
        Self::Group(value)
    }
}

impl From<proc_macro::TokenTree> for TokenTree {
    fn from(value: proc_macro::TokenTree) -> Self {
        match value {
            proc_macro::TokenTree::Ident(v) => Self::Token(Token::Ident(v.into())),
            proc_macro::TokenTree::Punct(v) => Self::Token(Token::Punct(v.into())),
            proc_macro::TokenTree::Literal(v) => Self::Token(Token::Literal(v.into())),
            proc_macro::TokenTree::Group(v) => Self::Group(v.into()),
        }
    }
}

impl From<TokenTree> for proc_macro::TokenTree {
    fn from(value: TokenTree) -> Self {
        match value {
            TokenTree::Token(Token::Ident(v)) => Self::Ident(v.into()),
            TokenTree::Token(Token::Punct(v)) => Self::Punct(v.into()),
            TokenTree::Token(Token::Literal(v)) => Self::Literal(v.into()),
            TokenTree::Group(v) => Self::Group(v.into()),
        }
    }
}

impl IntoIterator for TokenTree {
    type Item = TokenTree;
    type IntoIter = std::iter::Once<TokenTree>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl std::fmt::Display for TokenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
        }
    }
}

impl ToTokens for TokenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(self.clone());
    }
}

impl ToTokens for &str {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use std::str::FromStr;

        if let Ok(ts) = TokenStream::from_str(self) {
            ts.to_tokens(tokens);
        }
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
    fn token_tree_from_group() {
        let t: TokenTree = Group::new(Delim::Paren, TokenStream::new()).into();
        assert!(matches!(t, TokenTree::Group(_)));
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
