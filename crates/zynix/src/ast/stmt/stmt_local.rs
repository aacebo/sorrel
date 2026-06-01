use crate::ast::{Attribute, Expr, Pattern, Type};
use crate::token::ToTokens;
use crate::token::keyword::Let;
use crate::token::punct::Semi;
use crate::{Span, TokenStream};

#[doc = "A `let` binding statement."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtLocal {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub ty: Option<Type>,
    pub init: Option<StmtLocalInit>,
}

#[doc = "The initializer of a `let` binding."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtLocalInit {
    pub span: Span,
    pub expr: Expr,
    pub diverge: Option<Box<Expr>>,
}

impl ToTokens for StmtLocal {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Let::default().to_tokens(t);
        self.pat.to_tokens(t);
        if let Some(ty) = &self.ty {
            crate::token::punct::Colon::default().to_tokens(t);
            ty.to_tokens(t);
        }
        if let Some(init) = &self.init {
            crate::token::punct::Eq::default().to_tokens(t);
            init.expr.to_tokens(t);
            if let Some(div) = &init.diverge {
                crate::token::keyword::Else::default().to_tokens(t);
                div.to_tokens(t);
            }
        }
        Semi::default().to_tokens(t);
    }
}
