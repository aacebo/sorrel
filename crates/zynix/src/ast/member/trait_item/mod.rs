use crate::ast::Attribute;
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Const, Type as KwType};
use crate::token::punct::{Colon, Eq, Semi};
use crate::token::{Delim, ToTokens, TokenTree};
use crate::{Parse, Span, TokenStream};

mod trait_item_const;
mod trait_item_fn;
mod trait_item_macro;
mod trait_item_type;

pub use trait_item_const::*;
pub use trait_item_fn::*;
pub use trait_item_macro::*;
pub use trait_item_type::*;

#[doc = "An item inside a `trait` definition."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TraitItem {
    Fn(TraitItemFn),
    Const(TraitItemConst),
    Type(TraitItemType),
    Macro(TraitItemMacro),
}

macro_rules! impl_from {
    ($($variant:ident => $ty:ty),+ $(,)?) => {
        $(impl From<$ty> for TraitItem { fn from(v: $ty) -> Self { TraitItem::$variant(v) } })+
    };
}
impl_from! {
    Fn => TraitItemFn,
    Const => TraitItemConst,
    Type => TraitItemType,
    Macro => TraitItemMacro,
}

impl Parse for TraitItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;

        if super::is_kw(stream.curr(), "const") {
            let _ = stream.parse::<Const>()?;
            let ident = stream.parse::<crate::ast::Ident>()?;
            let generics = stream.parse::<crate::ast::Generics>()?;
            let _ = stream.parse::<Colon>()?;
            let ty = stream.parse::<crate::ast::Type>()?;
            let default = if stream.peek::<Eq>().is_some() {
                let _ = stream.parse::<Eq>()?;
                Some(stream.parse::<crate::ast::Expr>()?)
            } else {
                None
            };
            let _ = stream.parse::<Semi>();
            return Ok(TraitItem::Const(TraitItemConst {
                span: Span::default(),
                attrs,
                ident,
                generics,
                ty,
                default,
            }));
        }
        if super::is_kw(stream.curr(), "type") {
            let _ = stream.parse::<KwType>()?;
            let ident = stream.parse::<crate::ast::Ident>()?;
            let generics = stream.parse::<crate::ast::Generics>()?;
            let bounds = if stream.peek::<Colon>().is_some() {
                let _ = stream.parse::<Colon>()?;
                super::parse_plus_bounds(stream)?
            } else {
                crate::ast::Punctuated::new()
            };
            let default = if stream.peek::<Eq>().is_some() {
                let _ = stream.parse::<Eq>()?;
                Some(stream.parse::<crate::ast::Type>()?)
            } else {
                None
            };
            let _ = stream.parse::<Semi>();
            return Ok(TraitItem::Type(TraitItemType {
                span: Span::default(),
                attrs,
                ident,
                generics,
                bounds,
                default,
            }));
        }
        if super::is_fn_start(stream) {
            let sig = stream.parse::<crate::ast::Signature>()?;
            let default_body = if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
                Some(stream.parse::<crate::ast::StmtBlock>()?)
            } else {
                let _ = stream.parse::<Semi>();
                None
            };
            return Ok(TraitItem::Fn(TraitItemFn {
                span: Span::default(),
                attrs,
                sig,
                default_body,
            }));
        }
        let (mac, semi) = super::parse_semi_macro(stream, Vec::new())?;
        Ok(TraitItem::Macro(TraitItemMacro {
            span: Span::default(),
            attrs,
            mac,
            semi,
        }))
    }
}

impl ToTokens for TraitItem {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            TraitItem::Fn(v) => v.to_tokens(t),
            TraitItem::Const(v) => v.to_tokens(t),
            TraitItem::Type(v) => v.to_tokens(t),
            TraitItem::Macro(v) => v.to_tokens(t),
        }
    }
}
