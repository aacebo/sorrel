use super::emit_attrs;
use crate::ast::{Attribute, Ident};
use crate::token::keyword::MacroRules;
use crate::token::punct::Not;
use crate::token::{Delim, Group, ToTokens, TokenTree};
use crate::{Span, TokenStream};

#[doc = "A `macro_rules!` definition item."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacroRules {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub rules: TokenStream,
}

impl ToTokens for ItemMacroRules {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        MacroRules::default().to_tokens(t);
        Not::default().to_tokens(t);
        self.ident.to_tokens(t);
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, self.rules.clone())));
    }
}
