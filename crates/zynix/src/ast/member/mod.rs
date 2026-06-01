use crate::ast::Ident;
use crate::parse::{ParseError, ParseStream};
use crate::token::{self, LexError, ToTokens, Token, TokenTree};
use crate::{Parse, TokenStream};

pub mod foreign_item;
pub mod impl_item;
pub mod trait_item;

pub use foreign_item::*;
pub use impl_item::*;
pub use trait_item::*;

#[doc = "A struct/tuple field accessor — a named field (`.field`) or a tuple index (`.0`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Member {
    Named(Ident),
    Unnamed(u32),
}

impl Parse for Member {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match stream.curr() {
            Some(TokenTree::Token(Token::Literal(lit))) => {
                let index = lit
                    .repr()
                    .parse::<u32>()
                    .map_err(|_| ParseError::from(LexError::new(at).message("expected tuple index")))?;
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
                token::Literal::from_repr(&index.to_string(), crate::Span::default()).to_tokens(tokens);
            }
        }
    }
}

pub(crate) fn emit_attrs(attrs: &[crate::ast::Attribute], t: &mut TokenStream) {
    for a in attrs {
        a.to_tokens(t);
    }
}

pub(crate) fn is_kw(tt: Option<&TokenTree>, name: &str) -> bool {
    matches!(tt, Some(TokenTree::Token(Token::Keyword(k))) if k.as_str() == name)
}

pub(crate) fn parse_semi_macro(
    stream: &mut ParseStream,
    _attrs: Vec<crate::ast::Attribute>,
) -> Result<(crate::ast::MacroCall, bool), ParseError> {
    use crate::token::punct::Semi;
    let mac = stream.parse::<crate::ast::MacroCall>()?;
    let semi = if stream.peek::<Semi>().is_some() {
        let _ = stream.parse::<Semi>()?;
        true
    } else {
        false
    };
    Ok((mac, semi))
}

pub(crate) fn is_fn_start(stream: &mut ParseStream) -> bool {
    let mut fork = stream.fork();
    let _ = fork.parse::<crate::ast::Constness>();
    let _ = fork.parse::<crate::ast::Asyncness>();
    let _ = fork.parse::<crate::ast::Unsafety>();
    if fork.peek::<crate::token::keyword::Extern>().is_some() {
        let _ = fork.parse::<crate::ast::sig::Abi>();
    }
    fork.peek::<crate::token::keyword::Fn>().is_some()
}

pub(crate) fn parse_plus_bounds(
    stream: &mut ParseStream,
) -> Result<crate::ast::Punctuated<crate::ast::TypeBound, crate::token::punct::Plus>, ParseError> {
    use crate::token::punct::Plus;
    let mut bounds = crate::ast::Punctuated::new();
    loop {
        bounds.push_value(stream.parse::<crate::ast::TypeBound>()?);
        if stream.peek::<Plus>().is_some() {
            bounds.push_punct(stream.parse::<Plus>()?);
        } else {
            break;
        }
    }
    Ok(bounds)
}
