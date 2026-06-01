use zynix_macros::ToTokens;

use super::PathArguments;
use crate::ast::Ident;
use crate::parse::{ParseError, ParseStream};
use crate::token::Delim;
use crate::{Parse, Span, TokenTree};

#[derive(Debug, Clone, ToTokens)]
pub struct PathSegment {
    #[parse(skip)]
    pub span: Span,
    pub ident: Ident,
    pub args: PathArguments,
}

impl Parse for PathSegment {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let ident = stream.parse::<Ident>()?;

        // `Fn`-family segments take parenthesized args (`Fn(A) -> B`); this only
        // applies to those trait names, so it never swallows expression calls.
        let args = if is_fn_family(&ident)
            && matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Paren)
        {
            PathArguments::parse_parenthesized(stream)?
        } else {
            stream.parse::<PathArguments>()?
        };

        Ok(Self {
            span: Span::default(),
            ident,
            args,
        })
    }
}

fn is_fn_family(ident: &Ident) -> bool {
    matches!(ident.text.as_str(), "Fn" | "FnMut" | "FnOnce")
}
