use super::emit_attrs;
use crate::ast::{Attribute, Generics, Ident, Punctuated, TypeBound, Visibility};
use crate::token::ToTokens;
use crate::token::keyword::Trait;
use crate::token::punct::{Eq, Plus, Semi};
use crate::{Span, TokenStream};

#[doc = "A trait alias item (`trait Alias<T> = Bound1 + Bound2;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemTraitAlias {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl ToTokens for ItemTraitAlias {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        Trait::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        Eq::default().to_tokens(t);
        self.bounds.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
