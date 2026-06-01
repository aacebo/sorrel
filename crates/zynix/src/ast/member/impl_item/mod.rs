use crate::ast::{Attribute, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::{Const, Type as KwType};
use crate::token::punct::{Colon, Eq, Semi};
use crate::{Parse, Span, TokenStream};

mod impl_item_const;
mod impl_item_fn;
mod impl_item_macro;
mod impl_item_type;

pub use impl_item_const::*;
pub use impl_item_fn::*;
pub use impl_item_macro::*;
pub use impl_item_type::*;

#[doc = "An item inside an `impl` block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ImplItem {
    Fn(ImplItemFn),
    Const(ImplItemConst),
    Type(ImplItemType),
    Macro(ImplItemMacro),
}

macro_rules! impl_from {
    ($($variant:ident => $ty:ty),+ $(,)?) => {
        $(impl From<$ty> for ImplItem { fn from(v: $ty) -> Self { ImplItem::$variant(v) } })+
    };
}
impl_from! {
    Fn => ImplItemFn,
    Const => ImplItemConst,
    Type => ImplItemType,
    Macro => ImplItemMacro,
}

impl Parse for ImplItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = stream.parse::<crate::ast::Defaultness>()?;

        if super::is_kw(stream.curr(), "const") {
            let _ = stream.parse::<Const>()?;
            let ident = stream.parse::<crate::ast::Ident>()?;
            let generics = stream.parse::<crate::ast::Generics>()?;
            let _ = stream.parse::<Colon>()?;
            let ty = stream.parse::<crate::ast::Type>()?;
            let _ = stream.parse::<Eq>()?;
            let expr = stream.parse::<crate::ast::Expr>()?;
            let _ = stream.parse::<Semi>();
            return Ok(ImplItem::Const(ImplItemConst {
                span: Span::default(),
                attrs,
                vis,
                defaultness,
                ident,
                generics,
                ty,
                expr,
            }));
        }
        if super::is_kw(stream.curr(), "type") {
            let _ = stream.parse::<KwType>()?;
            let ident = stream.parse::<crate::ast::Ident>()?;
            let generics = stream.parse::<crate::ast::Generics>()?;
            let _ = stream.parse::<Eq>()?;
            let ty = stream.parse::<crate::ast::Type>()?;
            let _ = stream.parse::<Semi>();
            return Ok(ImplItem::Type(ImplItemType {
                span: Span::default(),
                attrs,
                vis,
                defaultness,
                ident,
                generics,
                ty,
            }));
        }
        if super::is_fn_start(stream) {
            let sig = stream.parse::<crate::ast::Signature>()?;
            let body = stream.parse::<crate::ast::StmtBlock>()?;
            return Ok(ImplItem::Fn(ImplItemFn {
                span: Span::default(),
                attrs,
                vis,
                defaultness,
                sig,
                body,
            }));
        }
        let (mac, semi) = super::parse_semi_macro(stream, Vec::new())?;
        Ok(ImplItem::Macro(ImplItemMacro {
            span: Span::default(),
            attrs,
            mac,
            semi,
        }))
    }
}

impl ToTokens for ImplItem {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            ImplItem::Fn(v) => v.to_tokens(t),
            ImplItem::Const(v) => v.to_tokens(t),
            ImplItem::Type(v) => v.to_tokens(t),
            ImplItem::Macro(v) => v.to_tokens(t),
        }
    }
}
