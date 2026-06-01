use crate::ast::{Attribute, Expr, Pattern, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Else, Let};
use crate::token::punct::{Eq, Semi};
use crate::token::{Delim, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

mod stmt_block;
mod stmt_local;
mod stmt_macro;

pub use stmt_block::*;
pub use stmt_local::*;
pub use stmt_macro::*;

#[doc = "A statement in a block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Stmt {
    Local(Box<StmtLocal>),
    Block(StmtBlock),
    Item(Box<crate::ast::Item>),
    Expr(Box<Expr>, Option<Semi>),
    Macro(StmtMacro),
}

impl Parse for Stmt {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;

        if stream.peek::<Let>().is_some() {
            let _ = stream.parse::<Let>()?;
            let pat = stream.parse::<Pattern>()?;
            let ty = if stream.peek::<crate::token::punct::Colon>().is_some() {
                let _ = stream.parse::<crate::token::punct::Colon>()?;
                Some(stream.parse::<Type>()?)
            } else {
                None
            };
            let init = if stream.peek::<Eq>().is_some() {
                let _ = stream.parse::<Eq>()?;
                let expr = stream.parse::<Expr>()?;
                let diverge = if stream.peek::<Else>().is_some() {
                    let _ = stream.parse::<Else>()?;
                    Some(Box::new(stream.parse::<Expr>()?))
                } else {
                    None
                };
                Some(StmtLocalInit {
                    span: Span::default(),
                    expr,
                    diverge,
                })
            } else {
                None
            };
            let _ = stream.parse::<Semi>();
            return Ok(Stmt::Local(Box::new(StmtLocal {
                span: Span::default(),
                attrs,
                pat,
                ty,
                init,
            })));
        }

        if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
            return Ok(Stmt::Block(stream.parse::<StmtBlock>()?));
        }

        let expr = stream.parse::<Expr>()?;
        let semi = if stream.peek::<Semi>().is_some() {
            Some(stream.parse::<Semi>()?)
        } else {
            None
        };
        Ok(Stmt::Expr(Box::new(expr), semi))
    }
}

impl ToTokens for Stmt {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Stmt::Local(v) => v.to_tokens(t),
            Stmt::Block(v) => v.to_tokens(t),
            Stmt::Item(v) => v.to_tokens(t),
            Stmt::Expr(v, semi) => {
                v.to_tokens(t);
                if let Some(s) = semi {
                    s.to_tokens(t);
                }
            }
            Stmt::Macro(v) => v.to_tokens(t),
        }
    }
}
