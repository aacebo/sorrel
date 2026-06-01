use super::emit_attrs;
use crate::ast::{Attribute, Ident, MacroCall};
use crate::token::ToTokens;
use crate::token::punct::Semi;
use crate::{Span, TokenStream};

#[doc = "A macro invocation used as an item (`name!(...);`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Option<Ident>,
    pub mac: MacroCall,
    pub semi: bool,
}

impl ToTokens for ItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.mac.to_tokens(t);
        if self.semi {
            Semi::default().to_tokens(t);
        }
    }
}
