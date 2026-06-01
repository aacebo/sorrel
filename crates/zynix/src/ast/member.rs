use crate::ast::Ident;
use crate::parse::{ParseError, ParseStream};
use crate::token::{self, LexError, ToTokens, Token, TokenTree};
use crate::{Parse, TokenStream};

#[doc = "A struct/tuple field accessor — a named field (`.field`) or a tuple index (`.0`)."]
#[derive(Debug, Clone)]
pub enum Member {
    Named(Ident),
    Unnamed(u32),
}

impl Parse for Member {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match stream.curr() {
            Some(TokenTree::Token(Token::Literal(lit))) => {
                let index = lit.repr().parse::<u32>().map_err(|_| {
                    ParseError::from(LexError::new(at).message("expected tuple index"))
                })?;
                stream.advance();
                Ok(Member::Unnamed(index))
            }
            _ => Ok(Member::Named(stream.parse()?)),
        }
    }
}

impl ToTokens for Member {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Member::Named(ident) => ident.to_tokens(tokens),
            Member::Unnamed(index) => {
                token::Literal::from_repr(&index.to_string(), crate::Span::default())
                    .to_tokens(tokens);
            }
        }
    }
}
