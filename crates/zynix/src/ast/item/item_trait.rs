use super::{emit_attrs, emit_brace_items};
use crate::ast::{Attribute, Generics, Ident, Punctuated, TraitItem, TypeBound, Unsafety, Visibility};
use crate::token::ToTokens;
use crate::token::keyword::{Auto, Trait};
use crate::token::punct::{Colon, Plus};
use crate::{Span, TokenStream};

#[doc = "A trait definition item (`trait Name: Super { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTrait {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub auto: bool,
    pub ident: Ident,
    pub generics: Generics,
    pub supertraits: Punctuated<TypeBound, Plus>,
    pub items: Vec<TraitItem>,
}

impl ToTokens for ItemTrait {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        if self.auto {
            Auto::default().to_tokens(t);
        }
        Trait::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        if !self.supertraits.is_empty() {
            Colon::default().to_tokens(t);
            self.supertraits.to_tokens(t);
        }
        emit_brace_items(&self.items, t);
    }
}
