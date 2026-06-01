use super::emit_attrs;
use crate::ast::{Attribute, Fields, Generics, Ident, Visibility};
use crate::token::ToTokens;
use crate::token::keyword::Struct;
use crate::token::punct::Semi;
use crate::{Span, TokenStream};

#[doc = "A struct item (`struct Name<T> { ... }` or `struct Name(T);`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemStruct {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: Fields,
}

impl ToTokens for ItemStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        Struct::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.fields.to_tokens(t);
        if !matches!(self.fields, Fields::Named(_)) {
            Semi::default().to_tokens(t);
        }
    }
}
