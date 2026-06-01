use super::ForeignItem;
use crate::ast::{Attribute, Ident, Mutability, Type, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::Static;
use crate::token::punct::{Colon, Semi};
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A foreign static declaration inside an `extern` block (`static NAME: Type;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemStatic {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Ident,
    pub ty: Type,
}

impl Parse for ForeignItemStatic {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match ForeignItem::parse(stream)? {
            ForeignItem::Static(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected foreign static").into()),
        }
    }
}

impl ToTokens for ForeignItemStatic {
    fn to_tokens(&self, t: &mut TokenStream) {
        super::super::emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        Static::default().to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
