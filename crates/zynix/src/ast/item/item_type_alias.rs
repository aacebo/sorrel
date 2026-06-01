use super::emit_attrs;
use crate::ast::{Attribute, Generics, Ident, Type, Visibility};
use crate::token::ToTokens;
use crate::token::keyword::Type as KwType;
use crate::token::punct::{Eq, Semi};
use crate::{Span, TokenStream};

#[doc = "A type alias item (`type Name<T> = Type;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTypeAlias {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
}

impl ToTokens for ItemTypeAlias {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        KwType::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        Eq::default().to_tokens(t);
        self.ty.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
