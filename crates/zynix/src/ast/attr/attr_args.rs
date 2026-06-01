use crate::ast::{DelimiterKind, Expr};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Eq;
use crate::token::{Delim, Group, LexError, ToTokens};
use crate::{Parse, TokenStream, TokenTree};

#[doc = "The arguments of an attribute, after its path (e.g. the `(Clone)` in `derive(Clone)` or the `= \"x\"` in `path = \"x\"`)."]
#[derive(Debug, Clone)]
pub enum AttrArgs {
    Empty,
    Delimited {
        delim: DelimiterKind,
        tokens: TokenStream,
    },
    /// A name-value tail (`= expr`); the name is the enclosing `Attribute.path`.
    NameValue(Expr),
}

impl Parse for AttrArgs {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.curr() {
            None => Ok(AttrArgs::Empty),
            Some(TokenTree::Group(g)) => {
                let delim = match g.delim() {
                    Delim::Paren => DelimiterKind::Paren,
                    Delim::Bracket => DelimiterKind::Bracket,
                    Delim::Brace => DelimiterKind::Brace,
                    Delim::None => {
                        return Err(LexError::new(stream.span())
                            .message("invalid attribute argument delimiter")
                            .into());
                    }
                };
                let tokens = g.stream();
                stream.advance();
                Ok(AttrArgs::Delimited { delim, tokens })
            }
            Some(TokenTree::Token(crate::Token::Punct(crate::token::Punctuation::Eq(_)))) => {
                let _ = stream.parse::<Eq>()?;
                Ok(AttrArgs::NameValue(stream.parse::<Expr>()?))
            }
            _ => Err(LexError::new(stream.span())
                .message("expected attribute arguments")
                .into()),
        }
    }
}

impl ToTokens for AttrArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AttrArgs::Empty => {}
            AttrArgs::Delimited {
                delim,
                tokens: inner,
            } => {
                let delim = match delim {
                    DelimiterKind::Paren => Delim::Paren,
                    DelimiterKind::Bracket => Delim::Bracket,
                    DelimiterKind::Brace => Delim::Brace,
                };
                tokens.extend_one(TokenTree::Group(Group::new(delim, inner.clone())));
            }
            AttrArgs::NameValue(expr) => {
                Eq::default().to_tokens(tokens);
                expr.to_tokens(tokens);
            }
        }
    }
}
