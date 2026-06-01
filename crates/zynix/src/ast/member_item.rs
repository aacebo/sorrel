use crate::ast::{
    Attribute, Block, Defaultness, Expr, Generics, Ident, MacroCall, Mutability, Punctuated,
    Signature, Type, TypeBound, Visibility,
};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Const, Fn, Static, Type as KwType};
use crate::token::punct::{Colon, Eq, Plus, Semi};
use crate::token::{Delim, LexError, ToTokens, Token, TokenTree};
use crate::{Parse, Span, TokenStream};

fn emit_attrs(attrs: &[Attribute], t: &mut TokenStream) {
    for a in attrs {
        a.to_tokens(t);
    }
}

fn parse_semi_macro(
    stream: &mut ParseStream,
    attrs: Vec<Attribute>,
) -> Result<(MacroCall, bool), ParseError> {
    let _ = attrs;
    let mac = stream.parse::<MacroCall>()?;
    let semi = if stream.peek::<Semi>().is_some() {
        let _ = stream.parse::<Semi>()?;
        true
    } else {
        false
    };
    Ok((mac, semi))
}

fn is_kw(tt: Option<&TokenTree>, name: &str) -> bool {
    matches!(tt, Some(TokenTree::Token(Token::Keyword(k))) if k.as_str() == name)
}

// ===========================================================================
// ImplItem
// ===========================================================================

#[derive(Debug, Clone)]
pub struct ImplItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub sig: Signature,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct ImplItemConst {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct ImplItemType {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct ImplItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}

#[derive(Debug, Clone)]
pub enum ImplItem {
    Fn(ImplItemFn),
    Const(ImplItemConst),
    Type(ImplItemType),
    Macro(ImplItemMacro),
}

impl Parse for ImplItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let defaultness = stream.parse::<Defaultness>()?;

        if is_kw(stream.curr(), "const") {
            let _ = stream.parse::<Const>()?;
            let ident = stream.parse::<Ident>()?;
            let generics = stream.parse::<Generics>()?;
            let _ = stream.parse::<Colon>()?;
            let ty = stream.parse::<Type>()?;
            let _ = stream.parse::<Eq>()?;
            let expr = stream.parse::<Expr>()?;
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
        if is_kw(stream.curr(), "type") {
            let _ = stream.parse::<KwType>()?;
            let ident = stream.parse::<Ident>()?;
            let generics = stream.parse::<Generics>()?;
            let _ = stream.parse::<Eq>()?;
            let ty = stream.parse::<Type>()?;
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
        if is_fn_start(stream) {
            let sig = stream.parse::<Signature>()?;
            let body = stream.parse::<Block>()?;
            return Ok(ImplItem::Fn(ImplItemFn {
                span: Span::default(),
                attrs,
                vis,
                defaultness,
                sig,
                body,
            }));
        }
        // macro invocation
        let (mac, semi) = parse_semi_macro(stream, Vec::new())?;
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
            ImplItem::Fn(v) => {
                emit_attrs(&v.attrs, t);
                v.vis.to_tokens(t);
                v.defaultness.to_tokens(t);
                v.sig.to_tokens(t);
                v.body.to_tokens(t);
            }
            ImplItem::Const(v) => {
                emit_attrs(&v.attrs, t);
                v.vis.to_tokens(t);
                v.defaultness.to_tokens(t);
                Const::default().to_tokens(t);
                v.ident.to_tokens(t);
                v.generics.to_tokens(t);
                Colon::default().to_tokens(t);
                v.ty.to_tokens(t);
                Eq::default().to_tokens(t);
                v.expr.to_tokens(t);
                Semi::default().to_tokens(t);
            }
            ImplItem::Type(v) => {
                emit_attrs(&v.attrs, t);
                v.vis.to_tokens(t);
                v.defaultness.to_tokens(t);
                KwType::default().to_tokens(t);
                v.ident.to_tokens(t);
                v.generics.to_tokens(t);
                Eq::default().to_tokens(t);
                v.ty.to_tokens(t);
                Semi::default().to_tokens(t);
            }
            ImplItem::Macro(v) => {
                emit_attrs(&v.attrs, t);
                v.mac.to_tokens(t);
                if v.semi {
                    Semi::default().to_tokens(t);
                }
            }
        }
    }
}

// ===========================================================================
// TraitItem
// ===========================================================================

#[derive(Debug, Clone)]
pub struct TraitItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub sig: Signature,
    pub default_body: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct TraitItemConst {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
    pub default: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct TraitItemType {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: Punctuated<TypeBound, Plus>,
    pub default: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct TraitItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}

#[derive(Debug, Clone)]
pub enum TraitItem {
    Fn(TraitItemFn),
    Const(TraitItemConst),
    Type(TraitItemType),
    Macro(TraitItemMacro),
}

impl Parse for TraitItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;

        if is_kw(stream.curr(), "const") {
            let _ = stream.parse::<Const>()?;
            let ident = stream.parse::<Ident>()?;
            let generics = stream.parse::<Generics>()?;
            let _ = stream.parse::<Colon>()?;
            let ty = stream.parse::<Type>()?;
            let default = if stream.peek::<Eq>().is_some() {
                let _ = stream.parse::<Eq>()?;
                Some(stream.parse::<Expr>()?)
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
        if is_kw(stream.curr(), "type") {
            let _ = stream.parse::<KwType>()?;
            let ident = stream.parse::<Ident>()?;
            let generics = stream.parse::<Generics>()?;
            let bounds = if stream.peek::<Colon>().is_some() {
                let _ = stream.parse::<Colon>()?;
                parse_plus_bounds(stream)?
            } else {
                Punctuated::new()
            };
            let default = if stream.peek::<Eq>().is_some() {
                let _ = stream.parse::<Eq>()?;
                Some(stream.parse::<Type>()?)
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
        if is_fn_start(stream) {
            let sig = stream.parse::<Signature>()?;
            let default_body = if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace)
            {
                Some(stream.parse::<Block>()?)
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
        let (mac, semi) = parse_semi_macro(stream, Vec::new())?;
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
            TraitItem::Fn(v) => {
                emit_attrs(&v.attrs, t);
                v.sig.to_tokens(t);
                match &v.default_body {
                    Some(b) => b.to_tokens(t),
                    None => Semi::default().to_tokens(t),
                }
            }
            TraitItem::Const(v) => {
                emit_attrs(&v.attrs, t);
                Const::default().to_tokens(t);
                v.ident.to_tokens(t);
                v.generics.to_tokens(t);
                Colon::default().to_tokens(t);
                v.ty.to_tokens(t);
                if let Some(d) = &v.default {
                    Eq::default().to_tokens(t);
                    d.to_tokens(t);
                }
                Semi::default().to_tokens(t);
            }
            TraitItem::Type(v) => {
                emit_attrs(&v.attrs, t);
                KwType::default().to_tokens(t);
                v.ident.to_tokens(t);
                v.generics.to_tokens(t);
                if !v.bounds.is_empty() {
                    Colon::default().to_tokens(t);
                    v.bounds.to_tokens(t);
                }
                if let Some(d) = &v.default {
                    Eq::default().to_tokens(t);
                    d.to_tokens(t);
                }
                Semi::default().to_tokens(t);
            }
            TraitItem::Macro(v) => {
                emit_attrs(&v.attrs, t);
                v.mac.to_tokens(t);
                if v.semi {
                    Semi::default().to_tokens(t);
                }
            }
        }
    }
}

// ===========================================================================
// ForeignItem
// ===========================================================================

#[derive(Debug, Clone)]
pub struct ForeignItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
}

#[derive(Debug, Clone)]
pub struct ForeignItemStatic {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Ident,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct ForeignItemType {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
}

#[derive(Debug, Clone)]
pub struct ForeignItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}

#[derive(Debug, Clone)]
pub enum ForeignItem {
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
}

impl Parse for ForeignItem {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;

        if is_kw(stream.curr(), "static") {
            let _ = stream.parse::<Static>()?;
            let mutability = stream.parse::<Mutability>()?;
            let ident = stream.parse::<Ident>()?;
            let _ = stream.parse::<Colon>()?;
            let ty = stream.parse::<Type>()?;
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
        if is_kw(stream.curr(), "type") {
            let _ = stream.parse::<KwType>()?;
            let ident = stream.parse::<Ident>()?;
            let generics = stream.parse::<Generics>()?;
            let _ = stream.parse::<Semi>();
            return Ok(ForeignItem::Type(ForeignItemType {
                span: Span::default(),
                attrs,
                vis,
                ident,
                generics,
            }));
        }
        if is_fn_start(stream) {
            let sig = stream.parse::<Signature>()?;
            let _ = stream.parse::<Semi>();
            return Ok(ForeignItem::Fn(ForeignItemFn {
                span: Span::default(),
                attrs,
                vis,
                sig,
            }));
        }
        let (mac, semi) = parse_semi_macro(stream, Vec::new())?;
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
            ForeignItem::Fn(v) => {
                emit_attrs(&v.attrs, t);
                v.vis.to_tokens(t);
                v.sig.to_tokens(t);
                Semi::default().to_tokens(t);
            }
            ForeignItem::Static(v) => {
                emit_attrs(&v.attrs, t);
                v.vis.to_tokens(t);
                Static::default().to_tokens(t);
                v.mutability.to_tokens(t);
                v.ident.to_tokens(t);
                Colon::default().to_tokens(t);
                v.ty.to_tokens(t);
                Semi::default().to_tokens(t);
            }
            ForeignItem::Type(v) => {
                emit_attrs(&v.attrs, t);
                v.vis.to_tokens(t);
                KwType::default().to_tokens(t);
                v.ident.to_tokens(t);
                v.generics.to_tokens(t);
                Semi::default().to_tokens(t);
            }
            ForeignItem::Macro(v) => {
                emit_attrs(&v.attrs, t);
                v.mac.to_tokens(t);
                if v.semi {
                    Semi::default().to_tokens(t);
                }
            }
        }
    }
}

// --- shared helpers ---

pub(crate) fn is_fn_start(stream: &mut ParseStream) -> bool {
    // a fn signature may begin with const/async/unsafe/extern then `fn`.
    let mut fork = stream.fork();
    let _ = fork.parse::<crate::ast::Constness>();
    let _ = fork.parse::<crate::ast::Asyncness>();
    let _ = fork.parse::<crate::ast::Unsafety>();
    if fork.peek::<crate::token::keyword::Extern>().is_some() {
        let _ = fork.parse::<crate::ast::sig::Abi>();
    }
    fork.peek::<Fn>().is_some()
}

pub(crate) fn parse_plus_bounds(
    stream: &mut ParseStream,
) -> Result<Punctuated<TypeBound, Plus>, ParseError> {
    let mut bounds = Punctuated::new();
    loop {
        bounds.push_value(stream.parse::<TypeBound>()?);
        if stream.peek::<Plus>().is_some() {
            bounds.push_punct(stream.parse::<Plus>()?);
        } else {
            break;
        }
    }
    Ok(bounds)
}

fn _unused(_: LexError) {}
