use super::{emit_attrs, emit_brace_items};
use crate::ast::{Attribute, Defaultness, Generics, ImplItem, TraitRef, Type, Unsafety};
use crate::token::ToTokens;
use crate::token::keyword::{For, Impl};
use crate::{Span, TokenStream};

#[doc = "An `impl` block, optionally implementing a trait (`impl Trait for Type { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemImpl {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub defaultness: Defaultness,
    pub unsafety: Unsafety,
    pub generics: Generics,
    pub trait_ref: Option<TraitRef>,
    pub self_ty: Type,
    pub items: Vec<ImplItem>,
}

impl ToTokens for ItemImpl {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.defaultness.to_tokens(t);
        self.unsafety.to_tokens(t);
        Impl::default().to_tokens(t);
        self.generics.to_tokens(t);
        if let Some(tr) = &self.trait_ref {
            tr.to_tokens(t);
            For::default().to_tokens(t);
        }
        self.self_ty.to_tokens(t);
        emit_brace_items(&self.items, t);
    }
}
