use crate::ast::{Attribute, MacroCall};
use crate::token::ToTokens;
use crate::token::punct::Semi;
use crate::{Span, TokenStream};

#[doc = "A macro invocation used as a statement (`name!(...);` or `name!(...)`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}

impl ToTokens for StmtMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.mac.to_tokens(t);
        if self.semi {
            Semi::default().to_tokens(t);
        }
    }
}
