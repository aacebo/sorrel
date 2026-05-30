use std::borrow::Cow;

use super::fallback;
use super::{ToTokens, TokenStream};
use crate::Span;

#[derive(Debug, Clone)]
pub enum Ident {
    Compiler(proc_macro::Ident),
    Fallback(fallback::Ident),
}

impl Ident {
    pub fn new(name: &str, span: Span) -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Ident::new(name, span.into()))
        } else {
            Self::Fallback(fallback::Ident::new(name, span))
        }
    }

    pub fn name(&self) -> Cow<'_, str> {
        match self {
            Self::Compiler(v) => Cow::Owned(v.to_string()),
            Self::Fallback(v) => v.name(),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Compiler(v) => v.span().into(),
            Self::Fallback(v) => v.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Compiler(v) => v.set_span(span.into()),
            Self::Fallback(v) => v.set_span(span),
        }
    }
}

impl From<proc_macro::Ident> for Ident {
    fn from(value: proc_macro::Ident) -> Self {
        Self::Compiler(value)
    }
}

impl From<Ident> for proc_macro::Ident {
    fn from(value: Ident) -> Self {
        match value {
            Ident::Compiler(v) => v,
            Ident::Fallback(v) => proc_macro::Ident::new(&v.name, v.span.into()),
        }
    }
}

impl From<fallback::Ident> for Ident {
    fn from(value: fallback::Ident) -> Self {
        Self::Fallback(value)
    }
}

impl From<Ident> for fallback::Ident {
    fn from(value: Ident) -> Self {
        match value {
            Ident::Compiler(v) => fallback::Ident::new(&v.to_string(), v.span().into()),
            Ident::Fallback(v) => v,
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler(v) => write!(f, "{}", v),
            Self::Fallback(v) => write!(f, "{}", v),
        }
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(self.clone().into());
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Ident {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Fallback(v) => v.serialize(s),
            Self::Compiler(_) => self.to_string().serialize(s),
        }
    }
}

impl crate::Parse for Ident {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Ident(v))) => Ok(v.clone()),
            _ => Err(crate::token::lex::LexError::new(stream.span())
                .message("expected Ident")
                .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    mod serde {
        use crate::TokenStream;
        use crate::token::Ident;
        use std::str::FromStr;

        #[test]
        fn ident_serializes_as_string() {
            let ts = TokenStream::from_str("foo").unwrap();
            let id = ts.parse().parse::<Ident>().unwrap();
            assert_eq!(serde_json::to_value(&id).unwrap(), serde_json::json!("foo"));
        }
    }
}
