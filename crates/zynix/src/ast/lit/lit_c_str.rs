use super::Lit;
use crate::parse::{ParseError, ParseStream};
use crate::token::{self, LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[derive(Debug, Clone)]
pub struct LitCStr {
    pub span: Span,
    pub repr: String,
}

impl Parse for LitCStr {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match Lit::parse(stream)? {
            Lit::CStr(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected C-string literal").into()),
        }
    }
}

impl ToTokens for LitCStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        token::Literal::from_repr(&self.repr, self.span).to_tokens(tokens);
    }
}

impl std::fmt::Display for LitCStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.repr)
    }
}
