use super::emit_attrs;
use crate::ast::{Attribute, Generics, Ident, Punctuated, Variant, Visibility};
use crate::token::keyword::Enum;
use crate::token::punct::Comma;
use crate::token::{Delim, Group, ToTokens, TokenStream as TS, TokenTree};
use crate::{Span, TokenStream};

#[doc = "An enum item (`enum Name<T> { Variant, ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemEnum {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub variants: Punctuated<Variant, Comma>,
}

impl ToTokens for ItemEnum {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        Enum::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        let mut inner = TS::new();
        self.variants.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}
