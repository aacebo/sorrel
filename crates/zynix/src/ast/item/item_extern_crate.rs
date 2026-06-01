use super::emit_attrs;
use crate::ast::{Attribute, Ident, Visibility};
use crate::token::ToTokens;
use crate::token::keyword::{As, Crate as KwCrate, Extern};
use crate::token::punct::Semi;
use crate::{Span, TokenStream};

#[doc = "An `extern crate` item (`extern crate foo;` or `extern crate foo as bar;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemExternCrate {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub rename: Option<Ident>,
}

impl ToTokens for ItemExternCrate {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        Extern::default().to_tokens(t);
        KwCrate::default().to_tokens(t);
        self.ident.to_tokens(t);
        if let Some(r) = &self.rename {
            As::default().to_tokens(t);
            r.to_tokens(t);
        }
        Semi::default().to_tokens(t);
    }
}
