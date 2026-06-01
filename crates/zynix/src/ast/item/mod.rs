use crate::ast::{
    Abi, Attribute, BoundPolarity, Defaultness, Expr, Fields, FieldsNamed, ForeignItem, Generics, Ident, ImplItem, MacroCall,
    Mutability, Punctuated, Signature, StmtBlock, TraitItem, TraitRef, Type, Unsafety, UseTree, Visibility,
};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{
    As, Auto, Const, Crate as KwCrate, Enum, Extern, For, Impl, Mod, Static, Struct, Trait, Type as KwType, Union, Use,
};
use crate::token::punct::{Colon, Eq, Not, Semi};
use crate::token::{Delim, Group, LexError, ToTokens, Token, TokenStream as TS, TokenTree};
use crate::{Parse, Span, TokenStream};

mod item_const;
mod item_enum;
mod item_extern_crate;
mod item_fn;
mod item_foreign_mod;
mod item_impl;
mod item_macro;
mod item_macro_rules;
mod item_mod;
mod item_static;
mod item_struct;
mod item_trait;
mod item_trait_alias;
mod item_type_alias;
mod item_union;
mod item_use;

pub use item_const::*;
pub use item_enum::*;
pub use item_extern_crate::*;
pub use item_fn::*;
pub use item_foreign_mod::*;
pub use item_impl::*;
pub use item_macro::*;
pub use item_macro_rules::*;
pub use item_mod::*;
pub use item_static::*;
pub use item_struct::*;
pub use item_trait::*;
pub use item_trait_alias::*;
pub use item_type_alias::*;
pub use item_union::*;
pub use item_use::*;

#[doc = "A top-level item (fn, struct, enum, trait, impl, use, ...)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Item {
    Use(ItemUse),
    ExternCrate(ItemExternCrate),
    Mod(ItemMod),
    Fn(ItemFn),
    Struct(ItemStruct),
    Enum(ItemEnum),
    Union(ItemUnion),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Impl(ItemImpl),
    TypeAlias(ItemTypeAlias),
    Const(ItemConst),
    Static(ItemStatic),
    Macro(ItemMacro),
    Macro2(ItemMacroRules),
    ForeignMod(ItemForeignMod),
}

macro_rules! impl_from {
    ($($variant:ident => $ty:ty),+ $(,)?) => {
        $(
            impl From<$ty> for Item {
                fn from(value: $ty) -> Self {
                    Item::$variant(value)
                }
            }
        )+
    };
}

impl_from! {
    Use => ItemUse,
    ExternCrate => ItemExternCrate,
    Mod => ItemMod,
    Fn => ItemFn,
    Struct => ItemStruct,
    Enum => ItemEnum,
    Union => ItemUnion,
    Trait => ItemTrait,
    TraitAlias => ItemTraitAlias,
    Impl => ItemImpl,
    TypeAlias => ItemTypeAlias,
    Const => ItemConst,
    Static => ItemStatic,
    Macro => ItemMacro,
    Macro2 => ItemMacroRules,
    ForeignMod => ItemForeignMod,
}

impl Parse for Item {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;

        // `macro_rules! name { ... }`
        if is_kw(stream.curr(), "macro_rules") {
            let _ = stream.parse::<crate::token::keyword::MacroRules>()?;
            let _ = stream.parse::<Not>()?;
            let ident = stream.parse::<Ident>()?;
            let rules = match stream.curr() {
                Some(TokenTree::Group(g)) => {
                    let s = g.stream();
                    stream.advance();
                    s
                }
                _ => {
                    return Err(LexError::new(stream.span()).message("expected macro body").into());
                }
            };
            return Ok(Item::Macro2(ItemMacroRules {
                span: Span::default(),
                attrs,
                ident,
                rules,
            }));
        }

        if is_kw(stream.curr(), "use") {
            let _ = stream.parse::<Use>()?;
            let tree = stream.parse::<UseTree>()?;
            let _ = stream.parse::<Semi>();
            return Ok(Item::Use(ItemUse {
                span: Span::default(),
                attrs,
                vis,
                tree,
            }));
        }
        if is_kw(stream.curr(), "extern") && is_kw_after_extern(stream, "crate") {
            let _ = stream.parse::<Extern>()?;
            let _ = stream.parse::<KwCrate>()?;
            let ident = stream.parse::<Ident>()?;
            let rename = if stream.peek::<As>().is_some() {
                let _ = stream.parse::<As>()?;
                Some(stream.parse::<Ident>()?)
            } else {
                None
            };
            let _ = stream.parse::<Semi>();
            return Ok(Item::ExternCrate(ItemExternCrate {
                span: Span::default(),
                attrs,
                vis,
                ident,
                rename,
            }));
        }
        // `extern "abi" { ... }` foreign mod (also `unsafe extern`).
        if is_kw(stream.curr(), "extern") || (is_kw(stream.curr(), "unsafe") && is_extern_block(stream)) {
            let unsafety = stream.parse::<Unsafety>()?;
            let abi = stream.parse::<Abi>()?;
            let group = stream.parse_group(Delim::Brace)?;
            let mut inner = group.parse();
            let items = inner.parse_vec::<ForeignItem>()?;
            return Ok(Item::ForeignMod(ItemForeignMod {
                span: Span::default(),
                attrs,
                unsafety,
                abi,
                items,
            }));
        }
        if is_kw(stream.curr(), "mod") {
            let unsafety = Unsafety::Safe;
            let _ = stream.parse::<Mod>()?;
            let ident = stream.parse::<Ident>()?;
            let content = if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace) {
                let group = stream.parse_group(Delim::Brace)?;
                let mut inner = group.parse();
                Some(inner.parse_vec::<Item>()?)
            } else {
                let _ = stream.parse::<Semi>();
                None
            };
            return Ok(Item::Mod(ItemMod {
                span: Span::default(),
                attrs,
                vis,
                unsafety,
                ident,
                content,
            }));
        }
        if is_kw(stream.curr(), "struct") {
            let _ = stream.parse::<Struct>()?;
            let ident = stream.parse::<Ident>()?;
            let mut generics = stream.parse::<Generics>()?;
            // `struct S where ...;` or `struct S { ... }` or `struct S(...);`
            if stream.peek::<crate::token::keyword::Where>().is_some() {
                generics.where_clause = Some(stream.parse()?);
            }
            let fields = stream.parse::<Fields>()?;
            let _ = stream.parse::<Semi>();
            return Ok(Item::Struct(ItemStruct {
                span: Span::default(),
                attrs,
                vis,
                ident,
                generics,
                fields,
            }));
        }
        if is_kw(stream.curr(), "enum") {
            let _ = stream.parse::<Enum>()?;
            let ident = stream.parse::<Ident>()?;
            let mut generics = stream.parse::<Generics>()?;
            if stream.peek::<crate::token::keyword::Where>().is_some() {
                generics.where_clause = Some(stream.parse()?);
            }
            let group = stream.parse_group(Delim::Brace)?;
            let mut inner = group.parse();
            let variants = Punctuated::parse_terminated(&mut inner)?;
            return Ok(Item::Enum(ItemEnum {
                span: Span::default(),
                attrs,
                vis,
                ident,
                generics,
                variants,
            }));
        }
        if is_kw(stream.curr(), "union") {
            let _ = stream.parse::<Union>()?;
            let ident = stream.parse::<Ident>()?;
            let generics = stream.parse::<Generics>()?;
            let fields = stream.parse::<FieldsNamed>()?;
            return Ok(Item::Union(ItemUnion {
                span: Span::default(),
                attrs,
                vis,
                ident,
                generics,
                fields,
            }));
        }
        if is_kw(stream.curr(), "trait")
            || is_kw(stream.curr(), "auto") && is_kw_after_unsafety(stream, "trait")
            || is_kw(stream.curr(), "unsafe") && is_trait_after(stream)
        {
            return parse_trait(stream, attrs, vis);
        }
        if is_kw(stream.curr(), "impl") || (is_kw(stream.curr(), "unsafe") && is_impl_after(stream)) {
            return parse_impl(stream, attrs);
        }
        if is_kw(stream.curr(), "type") {
            let _ = stream.parse::<KwType>()?;
            let ident = stream.parse::<Ident>()?;
            let generics = stream.parse::<Generics>()?;
            let _ = stream.parse::<Eq>()?;
            let ty = stream.parse::<Type>()?;
            let _ = stream.parse::<Semi>();
            return Ok(Item::TypeAlias(ItemTypeAlias {
                span: Span::default(),
                attrs,
                vis,
                ident,
                generics,
                ty,
            }));
        }
        if is_kw(stream.curr(), "const") && !crate::ast::member::is_fn_start(stream) {
            let _ = stream.parse::<Const>()?;
            let ident = stream.parse::<Ident>()?;
            let generics = stream.parse::<Generics>()?;
            let _ = stream.parse::<Colon>()?;
            let ty = stream.parse::<Type>()?;
            let _ = stream.parse::<Eq>()?;
            let expr = stream.parse::<Expr>()?;
            let _ = stream.parse::<Semi>();
            return Ok(Item::Const(ItemConst {
                span: Span::default(),
                attrs,
                vis,
                ident,
                generics,
                ty,
                expr,
            }));
        }
        if is_kw(stream.curr(), "static") {
            let _ = stream.parse::<Static>()?;
            let mutability = stream.parse::<Mutability>()?;
            let ident = stream.parse::<Ident>()?;
            let _ = stream.parse::<Colon>()?;
            let ty = stream.parse::<Type>()?;
            let _ = stream.parse::<Eq>()?;
            let expr = stream.parse::<Expr>()?;
            let _ = stream.parse::<Semi>();
            return Ok(Item::Static(ItemStatic {
                span: Span::default(),
                attrs,
                vis,
                mutability,
                ident,
                ty,
                expr,
            }));
        }
        if crate::ast::member::is_fn_start(stream) {
            let defaultness = Defaultness::Final;
            let sig = stream.parse::<Signature>()?;
            let body = stream.parse::<StmtBlock>()?;
            return Ok(Item::Fn(ItemFn {
                span: Span::default(),
                attrs,
                vis,
                defaultness,
                sig,
                body,
            }));
        }
        // Macro invocation item: `path!(...);`
        if let Some(mac) = stream.parse_opt::<MacroCall>() {
            let semi = if stream.peek::<Semi>().is_some() {
                let _ = stream.parse::<Semi>()?;
                true
            } else {
                false
            };
            return Ok(Item::Macro(ItemMacro {
                span: Span::default(),
                attrs,
                ident: None,
                mac,
                semi,
            }));
        }

        Err(LexError::new(at).message("expected item").into())
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Item::Use(v) => v.to_tokens(t),
            Item::ExternCrate(v) => v.to_tokens(t),
            Item::Mod(v) => v.to_tokens(t),
            Item::Fn(v) => v.to_tokens(t),
            Item::Struct(v) => v.to_tokens(t),
            Item::Enum(v) => v.to_tokens(t),
            Item::Union(v) => v.to_tokens(t),
            Item::Trait(v) => v.to_tokens(t),
            Item::TraitAlias(v) => v.to_tokens(t),
            Item::Impl(v) => v.to_tokens(t),
            Item::TypeAlias(v) => v.to_tokens(t),
            Item::Const(v) => v.to_tokens(t),
            Item::Static(v) => v.to_tokens(t),
            Item::Macro(v) => v.to_tokens(t),
            Item::Macro2(v) => v.to_tokens(t),
            Item::ForeignMod(v) => v.to_tokens(t),
        }
    }
}

// ===========================================================================
// Crate
// ===========================================================================

#[doc = "A whole parsed crate (inner attributes + items)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Crate {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub items: Vec<Item>,
}

impl Parse for Crate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let items = stream.parse_vec::<Item>()?;
        Ok(Self {
            span: Span::default(),
            attrs,
            items,
        })
    }
}

impl ToTokens for Crate {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        for it in &self.items {
            it.to_tokens(t);
        }
    }
}

pub(super) fn emit_attrs(attrs: &[Attribute], t: &mut TokenStream) {
    for a in attrs {
        a.to_tokens(t);
    }
}

pub(super) fn is_kw(tt: Option<&TokenTree>, name: &str) -> bool {
    matches!(tt, Some(TokenTree::Token(Token::Keyword(k))) if k.as_str() == name)
}

fn parse_trait(stream: &mut ParseStream, attrs: Vec<Attribute>, vis: Visibility) -> Result<Item, ParseError> {
    let unsafety = stream.parse::<Unsafety>()?;
    let auto = if stream.peek::<Auto>().is_some() {
        let _ = stream.parse::<Auto>()?;
        true
    } else {
        false
    };
    let _ = stream.parse::<Trait>()?;
    let ident = stream.parse::<Ident>()?;
    let mut generics = stream.parse::<Generics>()?;

    // `trait T = Bound;` alias vs `trait T : Bounds { ... }`.
    if stream.peek::<Eq>().is_some() {
        let _ = stream.parse::<Eq>()?;
        let bounds = crate::ast::member::parse_plus_bounds(stream)?;
        let _ = stream.parse::<Semi>();
        return Ok(Item::TraitAlias(ItemTraitAlias {
            span: Span::default(),
            attrs,
            vis,
            ident,
            generics,
            bounds,
        }));
    }

    let supertraits = if stream.peek::<Colon>().is_some() {
        let _ = stream.parse::<Colon>()?;
        crate::ast::member::parse_plus_bounds(stream)?
    } else {
        Punctuated::new()
    };
    if stream.peek::<crate::token::keyword::Where>().is_some() {
        generics.where_clause = Some(stream.parse()?);
    }
    let group = stream.parse_group(Delim::Brace)?;
    let mut inner = group.parse();
    let items = inner.parse_vec::<TraitItem>()?;
    Ok(Item::Trait(ItemTrait {
        span: Span::default(),
        attrs,
        vis,
        unsafety,
        auto,
        ident,
        generics,
        supertraits,
        items,
    }))
}

fn parse_impl(stream: &mut ParseStream, attrs: Vec<Attribute>) -> Result<Item, ParseError> {
    let defaultness = stream.parse::<Defaultness>()?;
    let unsafety = stream.parse::<Unsafety>()?;
    let _ = stream.parse::<Impl>()?;
    let generics = stream.parse::<Generics>()?;

    // Optional `!` for a negative impl (`impl !Trait for T`).
    let polarity = if stream.peek::<Not>().is_some() {
        let _ = stream.parse::<Not>()?;
        BoundPolarity::Negative
    } else {
        BoundPolarity::Positive
    };

    // `impl Trait for Type` vs `impl Type`. Parse a type; if `for` follows, it was the trait.
    let first = stream.parse::<Type>()?;
    let (trait_ref, self_ty) = if stream.peek::<For>().is_some() {
        let _ = stream.parse::<For>()?;
        let self_ty = stream.parse::<Type>()?;
        (Some(type_to_trait_ref(first, polarity)?), self_ty)
    } else {
        (None, first)
    };

    let mut generics = generics;
    if stream.peek::<crate::token::keyword::Where>().is_some() {
        generics.where_clause = Some(stream.parse()?);
    }

    let group = stream.parse_group(Delim::Brace)?;
    let mut inner = group.parse();
    let items = inner.parse_vec::<ImplItem>()?;
    Ok(Item::Impl(ItemImpl {
        span: Span::default(),
        attrs,
        defaultness,
        unsafety,
        generics,
        trait_ref,
        self_ty,
        items,
    }))
}

fn type_to_trait_ref(ty: Type, polarity: BoundPolarity) -> Result<TraitRef, ParseError> {
    match ty {
        Type::Path(tp) => Ok(TraitRef {
            span: Span::default(),
            polarity,
            path: tp.path,
        }),
        _ => Err(LexError::new(Span::default()).message("expected trait path").into()),
    }
}

// lookahead helpers
fn is_kw_after_extern(stream: &mut ParseStream, name: &str) -> bool {
    let mut fork = stream.fork();
    let _ = fork.parse::<Extern>();
    is_kw(fork.curr(), name)
}

fn is_extern_block(stream: &mut ParseStream) -> bool {
    let mut fork = stream.fork();
    let _ = fork.parse::<Unsafety>();
    is_kw(fork.curr(), "extern")
}

fn is_impl_after(stream: &mut ParseStream) -> bool {
    let mut fork = stream.fork();
    let _ = fork.parse::<Unsafety>();
    is_kw(fork.curr(), "impl")
}

fn is_trait_after(stream: &mut ParseStream) -> bool {
    // `unsafe trait` or `unsafe auto trait`.
    let mut fork = stream.fork();
    let _ = fork.parse::<Unsafety>();
    if is_kw(fork.curr(), "auto") {
        fork.advance();
    }
    is_kw(fork.curr(), "trait")
}

fn is_kw_after_unsafety(stream: &mut ParseStream, name: &str) -> bool {
    // `auto trait` (no unsafe).
    let mut fork = stream.fork();
    if is_kw(fork.curr(), "auto") {
        fork.advance();
    }
    is_kw(fork.curr(), name)
}

pub(super) fn emit_brace_items<T: ToTokens>(items: &[T], t: &mut TokenStream) {
    let mut inner = TS::new();
    for it in items {
        it.to_tokens(&mut inner);
    }
    t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::token::ToTokenStream;

    fn parse<T: Parse>(src: &str) -> T {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>().unwrap()
    }

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn item_fn() {
        let i: Item = parse("fn f<T: A + 'a>(x: T) -> U where T: B { x }");
        assert!(matches!(i, Item::Fn(_)));
    }

    #[test]
    fn item_struct() {
        assert!(matches!(parse::<Item>("pub(crate) struct S<T> { a: T }"), Item::Struct(_)));
        assert!(matches!(parse::<Item>("struct P(u8, u16);"), Item::Struct(_)));
        assert!(matches!(parse::<Item>("struct U;"), Item::Struct(_)));
    }

    #[test]
    fn item_enum() {
        let i: Item = parse("enum E { A, B(u8), C { x: i32 } }");
        match i {
            Item::Enum(e) => assert_eq!(e.variants.len(), 3),
            _ => panic!("expected enum"),
        }
    }

    #[test]
    fn item_impl() {
        assert!(matches!(
            parse::<Item>("impl<T> Trait for S<T> { fn m(&self) {} }"),
            Item::Impl(_)
        ));
        assert!(matches!(parse::<Item>("impl S { }"), Item::Impl(_)));
    }

    #[test]
    fn item_trait() {
        let i: Item = parse("trait T: Clone { fn m(&self); type Out; }");
        match i {
            Item::Trait(t) => assert_eq!(t.items.len(), 2),
            _ => panic!("expected trait"),
        }
    }

    #[test]
    fn item_use() {
        assert!(matches!(parse::<Item>("use a::{b, c as d, e::*};"), Item::Use(_)));
    }

    #[test]
    fn item_const_static_type() {
        assert!(matches!(parse::<Item>("const X: u8 = 1;"), Item::Const(_)));
        assert!(matches!(parse::<Item>("static Y: u8 = 1;"), Item::Static(_)));
        assert!(matches!(parse::<Item>("type Z = u8;"), Item::TypeAlias(_)));
    }

    #[test]
    fn item_with_attr() {
        let i: Item = parse("#[derive(Clone)] pub fn g() {}");
        match i {
            Item::Fn(f) => assert_eq!(f.attrs.len(), 1),
            _ => panic!("expected fn"),
        }
    }

    #[test]
    fn item_mod_and_macro() {
        assert!(matches!(parse::<Item>("mod m { fn a() {} }"), Item::Mod(_)));
        assert!(matches!(parse::<Item>("mod m;"), Item::Mod(_)));
        assert!(matches!(parse::<Item>("macro_rules! m { () => {} }"), Item::Macro2(_)));
    }

    #[test]
    fn unsafe_auto_trait() {
        match parse::<Item>("unsafe trait T {}") {
            Item::Trait(t) => assert!(matches!(t.unsafety, crate::ast::Unsafety::Unsafe)),
            _ => panic!("expected trait"),
        }
        match parse::<Item>("auto trait T {}") {
            Item::Trait(t) => assert!(t.auto),
            _ => panic!("expected trait"),
        }
        assert!(matches!(parse::<Item>("unsafe auto trait T {}"), Item::Trait(_)));
    }

    #[test]
    fn negative_impl() {
        match parse::<Item>("impl !Send for S {}") {
            Item::Impl(i) => {
                let tr = i.trait_ref.unwrap();
                assert!(matches!(tr.polarity, crate::ast::BoundPolarity::Negative));
            }
            _ => panic!("expected impl"),
        }
    }

    #[test]
    fn variadic_fn() {
        // extern "C" fn(u8, ...) as a bare-fn type round-trips the variadic.
        let sig: crate::ast::Signature = {
            let ts = TokenStream::from_str("fn printf(fmt: u8, ...)").unwrap();
            ts.parse().parse().unwrap()
        };
        assert!(sig.variadic.is_some());
    }

    #[test]
    fn crate_roundtrip() {
        let c: Crate = parse("fn a() {} struct S { x: u8 }");
        assert_eq!(c.items.len(), 2);
        // re-render and re-parse stability
        let r = render(&c);
        let c2: Crate = parse(&r);
        assert_eq!(render(&c2), r);
    }
}
