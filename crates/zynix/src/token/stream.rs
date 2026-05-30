use std::str::FromStr;

use super::ToTokens;
use super::fallback;
use crate::TokenTree;
use crate::parse::{ParseError, ParseStream};

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
            Self::Compiler(v) => v.clone().into_iter().next().is_none(),
            Self::Fallback(v) => v.is_empty(),
        }
    }

    pub fn extend_one(&mut self, token: TokenTree) {
        match self {
            Self::Compiler(v) => v.extend_one(proc_macro::TokenTree::from(token)),
            Self::Fallback(v) => v.extend_one(token),
        }
    }

    pub fn parse(&self) -> ParseStream<'_> {
        ParseStream::new(self)
    }

    pub fn to_vec(self) -> Vec<TokenTree> {
        match self {
            Self::Compiler(v) => v.into_iter().map(TokenTree::from).collect(),
            Self::Fallback(v) => v.into_inner(),
        }
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::Fallback(fallback::TokenStream::new())
    }
}

impl Extend<TokenTree> for TokenStream {
    fn extend<T: IntoIterator<Item = TokenTree>>(&mut self, iter: T) {
        match self {
            Self::Compiler(v) => v.extend(iter.into_iter().map(proc_macro::TokenTree::from)),
            Self::Fallback(v) => v.extend(iter),
        }
    }
}

impl std::ops::Deref for TokenStream {
    type Target = [TokenTree];

    fn deref(&self) -> &[TokenTree] {
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
            TokenStream::Compiler(v) => v,
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
            TokenStream::Compiler(_) => fallback::TokenStream::from(value.to_vec()),
            TokenStream::Fallback(v) => v,
        }
    }
}

impl From<&[TokenTree]> for TokenStream {
    fn from(value: &[TokenTree]) -> Self {
        Self::Fallback(fallback::TokenStream::from(value.to_vec()))
    }
}

impl From<Vec<TokenTree>> for TokenStream {
    fn from(value: Vec<TokenTree>) -> Self {
        Self::Fallback(fallback::TokenStream::from(value))
    }
}

impl From<TokenStream> for Vec<TokenTree> {
    fn from(value: TokenStream) -> Self {
        match value {
            TokenStream::Compiler(v) => v.into_iter().map(TokenTree::from).collect(),
            TokenStream::Fallback(v) => v.into_iter().collect(),
        }
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<T: IntoIterator<Item = TokenTree>>(iter: T) -> Self {
        Self::Fallback(iter.into_iter().collect())
    }
}

impl FromIterator<Self> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        Self::Fallback(iter.into_iter().flat_map(|s| s.into_iter()).collect())
    }
}

impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = super::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Compiler(v) => v.into_iter().into(),
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
            Self::Compiler(v) => write!(f, "TokenStream::Compiler({})", v),
            Self::Fallback(v) => write!(f, "TokenStream::Fallback({:?})", v),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for TokenStream {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Fallback(v) => v.serialize(s),
            Self::Compiler(_) => {
                let tokens: Vec<TokenTree> = self.clone().into();
                tokens.serialize(s)
            }
        }
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler(v) => write!(f, "{}", v),
            Self::Fallback(v) => write!(f, "{}", v),
        }
    }
}

impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Compiler(v) => tokens.extend(v.clone().into_iter().map(TokenTree::from)),
            Self::Fallback(v) => v.to_tokens(tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    mod display {
        use crate::TokenStream;
        use std::str::FromStr;

        fn render(src: &str) -> String {
            TokenStream::from_str(src).unwrap().to_string()
        }

        #[test]
        fn separates_word_tokens() {
            // adjacent idents/keywords must not merge into invalid source
            assert!(!render("let x").contains("letx"));
            assert_eq!(render("let x"), "let x");
            assert_eq!(render("a + b"), "a + b");
        }

        #[test]
        fn keeps_joint_punct_tight() {
            // multi-char puncts stay glued (Joint spacing)
            assert_eq!(render("x == y"), "x == y");
            assert_eq!(render("a -> b"), "a -> b");
            assert_eq!(render("a :: b"), "a :: b");
            assert_eq!(render("..="), "..=");
        }

        #[test]
        fn renders_group_with_delimiters() {
            // delimiters are preserved (proc_macro2-style spacing inside)
            assert_eq!(render("(a + b)"), "(a + b)");
            assert_eq!(render("[a, b]"), "[a , b]");
            assert_eq!(render("{ x }"), "{x}");
        }

        #[test]
        fn group_follows_word_with_space() {
            // proc_macro2-style: groups are word tokens
            assert_eq!(render("foo(a, b)"), "foo (a , b)");
        }

        #[test]
        fn roundtrips_to_reparseable_source() {
            for src in ["let x = (a + b) * c;", "x == y", "a::b::c", "foo(1, 2)"] {
                let rendered = render(src);
                // must re-parse without error
                assert!(
                    TokenStream::from_str(&rendered).is_ok(),
                    "failed to reparse {:?} from {:?}",
                    rendered,
                    src
                );
                // and rendering is stable (idempotent)
                assert_eq!(render(&rendered), rendered);
            }
        }
    }

    #[cfg(feature = "serde")]
    mod serde {
        use crate::TokenStream;
        use std::str::FromStr;

        #[test]
        fn serializes_as_array_of_token_strings() {
            let ts = TokenStream::from_str("a + 1").unwrap();
            let json = serde_json::to_value(&ts).unwrap();
            assert_eq!(json, serde_json::json!(["a", "+", "1"]));
        }

        #[test]
        fn nested_group_serializes_as_nested_array() {
            let ts = TokenStream::from_str("a + (b * 2)").unwrap();
            let json = serde_json::to_value(&ts).unwrap();
            assert_eq!(
                json,
                serde_json::json!([
                    "a",
                    "+",
                    { "delim": "paren", "tokens": ["b", "*", "2"] }
                ])
            );
        }
    }
}
