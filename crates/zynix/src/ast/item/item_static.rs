use super::emit_attrs;
use crate::ast::{Attribute, Expr, Ident, Mutability, Type, Visibility};
use crate::token::ToTokens;
use crate::token::keyword::Static;
use crate::token::punct::{Colon, Eq, Semi};
use crate::{Span, TokenStream};

#[doc = "A static item (`static [mut] NAME: Type = expr;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStatic {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Ident,
    pub ty: Type,
    pub expr: Expr,
}

impl ToTokens for ItemStatic {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        Static::default().to_tokens(t);
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
        Eq::default().to_tokens(t);
        self.expr.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
