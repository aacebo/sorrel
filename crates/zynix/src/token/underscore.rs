use super::lex::LexError;
use super::{Ident, ToTokens};
use crate::parse::{ParseError, ParseStream};
use crate::{Parse, Span, Token, TokenStream, TokenTree};

#[derive(Debug, Default, Copy, Clone)]
pub struct Underscore {
    span: Span,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Underscore {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Self::TEXT.serialize(s)
    }
}

impl Underscore {
    pub const TEXT: &'static str = "_";

    pub fn new(span: Span) -> Self {
        Self { span }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl std::fmt::Display for Underscore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("_")
    }
}

impl Parse for Underscore {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match stream.advance() {
            Some(TokenTree::Token(Token::Ident(id))) if id.name().as_ref() == "_" => {
                Ok(Self::new(id.span()))
            }
            _ => Err(LexError::new(at).message("expected `_`").into()),
        }
    }
}

impl ToTokens for Underscore {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(Ident::new("_", self.span).into());
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    mod serde {
        use crate::token::Underscore;

        #[test]
        fn underscore_serializes_as_string() {
            assert_eq!(
                serde_json::to_value(Underscore::default()).unwrap(),
                serde_json::json!("_")
            );
        }
    }
}
