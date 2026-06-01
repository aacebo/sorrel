use super::emit_attrs;
use crate::ast::{Attribute, UseTree, Visibility};
use crate::token::ToTokens;
use crate::token::keyword::Use;
use crate::token::punct::Semi;
use crate::{Span, TokenStream};

#[doc = "A `use` item (`use path::to::Name;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemUse {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub tree: UseTree,
}

impl ToTokens for ItemUse {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        Use::default().to_tokens(t);
        self.tree.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
