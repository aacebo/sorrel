use crate::ast::Attribute;
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::{Static, Type as KwType};
use crate::token::punct::{Colon, Semi};
use crate::{Parse, Span, TokenStream};

mod foreign_item_fn;
mod foreign_item_macro;
mod foreign_item_static;
mod foreign_item_type;

pub use foreign_item_fn::*;
pub use foreign_item_macro::*;
pub use foreign_item_static::*;
pub use foreign_item_type::*;

#[doc = "An item inside an `extern` block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ForeignItem {
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
}

macro_rules! impl_from {
    ($($variant:ident => $ty:ty),+ $(,)?) => {
        $(impl From<$ty> for ForeignItem { fn from(v: $ty) -> Self { ForeignItem::$variant(v) } })+
    };
}
impl_from! {
    Fn => ForeignItemFn,
    Static => ForeignItemStatic,
    Type => ForeignItemType,
    Macro => ForeignItemMacro,
}

impl Parse for ForeignItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<crate::ast::Visibility>()?;

        if super::is_kw(stream.curr(), "static") {
            let _ = stream.parse::<Static>()?;
            let mutability = stream.parse::<crate::ast::Mutability>()?;
            let ident = stream.parse::<crate::ast::Ident>()?;
            let _ = stream.parse::<Colon>()?;
            let ty = stream.parse::<crate::ast::Type>()?;
            let _ = stream.parse::<Semi>();
            return Ok(ForeignItem::Static(ForeignItemStatic {
                span: Span::default(),
                attrs,
                vis,
                mutability,
                ident,
                ty,
            }));
        }
        if super::is_kw(stream.curr(), "type") {
            let _ = stream.parse::<KwType>()?;
            let ident = stream.parse::<crate::ast::Ident>()?;
            let generics = stream.parse::<crate::ast::Generics>()?;
            let _ = stream.parse::<Semi>();
            return Ok(ForeignItem::Type(ForeignItemType {
                span: Span::default(),
                attrs,
                vis,
                ident,
                generics,
            }));
        }
        if super::is_fn_start(stream) {
            let sig = stream.parse::<crate::ast::Signature>()?;
            let _ = stream.parse::<Semi>();
            return Ok(ForeignItem::Fn(ForeignItemFn {
                span: Span::default(),
                attrs,
                vis,
                sig,
            }));
        }
        let (mac, semi) = super::parse_semi_macro(stream, Vec::new())?;
        Ok(ForeignItem::Macro(ForeignItemMacro {
            span: Span::default(),
            attrs,
            mac,
            semi,
        }))
    }
}

impl ToTokens for ForeignItem {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            ForeignItem::Fn(v) => v.to_tokens(t),
            ForeignItem::Static(v) => v.to_tokens(t),
            ForeignItem::Type(v) => v.to_tokens(t),
            ForeignItem::Macro(v) => v.to_tokens(t),
        }
    }
}
