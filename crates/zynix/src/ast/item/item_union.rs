use super::emit_attrs;
use crate::ast::{Attribute, FieldsNamed, Generics, Ident, Visibility};
use crate::token::ToTokens;
use crate::token::keyword::Union;
use crate::{Span, TokenStream};

#[doc = "A union item (`union Name<T> { field: Type, ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemUnion {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: FieldsNamed,
}

impl ToTokens for ItemUnion {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        Union::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.fields.to_tokens(t);
    }
}
