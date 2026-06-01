use crate::ast::{MacroCall, Punctuated, TypeBound};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Dyn, Impl};
use crate::token::punct::{And, Comma, Plus, Star};
use crate::token::{Delim, ToTokens};
use crate::{Parse, Span, TokenStream};

mod q_self;
mod type_array;
mod type_bare_fn;
mod type_group;
mod type_impl_trait;
mod type_paren;
mod type_path;
mod type_pointer;
mod type_reference;
mod type_slice;
mod type_trait_object;
mod type_tuple;
mod typed_param;

pub use q_self::*;
pub use type_array::*;
pub use type_bare_fn::*;
pub use type_group::*;
pub use type_impl_trait::*;
pub use type_paren::*;
pub use type_path::*;
pub use type_pointer::*;
pub use type_reference::*;
pub use type_slice::*;
pub use type_trait_object::*;
pub use type_tuple::*;
pub use typed_param::*;

/// Parse `Bound + Bound + ...` for `impl`/`dyn` types.
pub(crate) fn parse_plus_bounds(stream: &mut ParseStream) -> Result<Punctuated<TypeBound, Plus>, ParseError> {
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

#[doc = "A Rust type expression. Covers all positions where a type can appear in source code."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Type {
    Never,
    Infer,
    Path(TypePath),
    Tuple(TypeTuple),
    Array(TypeArray),
    Slice(TypeSlice),
    Reference(TypeReference),
    Pointer(TypePointer),
    BareFn(TypeBareFn),
    ImplTrait(TypeImplTrait),
    TraitObject(TypeTraitObject),
    Paren(TypeParen),
    Group(TypeGroup),
    Macro(MacroCall),
}

macro_rules! impl_from {
    ($($variant:ident => $ty:ty),+ $(,)?) => {
        $(
            impl From<$ty> for Type {
                fn from(value: $ty) -> Self {
                    Type::$variant(value)
                }
            }
        )+
    };
}

impl_from! {
    Path => TypePath,
    Reference => TypeReference,
    Pointer => TypePointer,
    Tuple => TypeTuple,
    Paren => TypeParen,
    Slice => TypeSlice,
    ImplTrait => TypeImplTrait,
    TraitObject => TypeTraitObject,
    BareFn => TypeBareFn,
}

impl Parse for Type {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        // `&` reference, `*` raw pointer, `[` slice — disambiguated by the
        // leading token.
        if stream.peek::<And>().is_some() {
            return Ok(Type::Reference(stream.parse()?));
        }

        if stream.peek::<Star>().is_some() {
            return Ok(Type::Pointer(stream.parse()?));
        }

        // Never `!` and infer `_`.
        if stream.peek::<crate::token::punct::Not>().is_some() {
            let _ = stream.parse::<crate::token::punct::Not>()?;
            return Ok(Type::Never);
        }
        if matches!(stream.curr(), Some(tt) if is_named(tt, "_")) {
            stream.advance();
            return Ok(Type::Infer);
        }

        // `[T]` slice or `[T; N]` array — decided by a `;` inside the brackets.
        if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Bracket)) {
            let group = stream.parse_group(Delim::Bracket)?;
            let mut inner = group.parse();
            let elem = Box::new(inner.parse::<Type>()?);
            if inner.peek::<crate::token::punct::Semi>().is_some() {
                let _ = inner.parse::<crate::token::punct::Semi>()?;
                let len = inner.parse::<crate::ast::Expr>()?;
                return Ok(Type::Array(TypeArray {
                    span: Span::default(),
                    elem,
                    len,
                }));
            }
            return Ok(Type::Slice(TypeSlice {
                span: Span::default(),
                elem,
            }));
        }

        // `impl Trait` / `dyn Trait`.
        if stream.peek::<Impl>().is_some() {
            return Ok(Type::ImplTrait(stream.parse()?));
        }
        if stream.peek::<Dyn>().is_some() {
            return Ok(Type::TraitObject(stream.parse()?));
        }

        // Bare fn pointer: `fn(...)`, `extern "C" fn(...)`, `unsafe fn(...)`.
        if stream.peek::<crate::token::keyword::Fn>().is_some()
            || stream.peek::<crate::token::keyword::Extern>().is_some()
            || (stream.peek::<crate::token::keyword::Unsafe>().is_some())
        {
            return Ok(Type::BareFn(stream.parse()?));
        }

        // `(...)` — one element with no trailing comma is a parenthesized type;
        // anything else (empty, multiple, or trailing comma) is a tuple.
        if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Paren)) {
            let group = stream.parse_group(Delim::Paren)?;
            let mut inner = group.parse();
            let elems: Punctuated<Type, Comma> = Punctuated::parse_terminated(&mut inner)?;

            return if elems.len() == 1 && !elems.trailing_punct() {
                Ok(Type::Paren(TypeParen {
                    span: Span::default(),
                    elem: Box::new(elems.into_iter().next().unwrap()),
                }))
            } else {
                Ok(Type::Tuple(TypeTuple {
                    span: Span::default(),
                    elems,
                }))
            };
        }

        // Macro type `m!(...)` — a path followed by `!`.
        if let Some(mac) = stream.parse_opt::<MacroCall>() {
            return Ok(Type::Macro(mac));
        }

        // Otherwise a path type: `T`, `std::vec::Vec`, or a qualified
        // `<T as Trait>::Item` (which begins with `<`).
        Ok(Type::Path(stream.parse()?))
    }
}

fn is_named(tt: &crate::TokenTree, name: &str) -> bool {
    match tt {
        crate::TokenTree::Token(crate::Token::Ident(id)) => id.name() == name,
        crate::TokenTree::Token(crate::Token::Keyword(kw)) => kw.as_str() == name,
        _ => false,
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Type::Path(value) => value.to_tokens(tokens),
            Type::Reference(value) => value.to_tokens(tokens),
            Type::Pointer(value) => value.to_tokens(tokens),
            Type::Tuple(value) => value.to_tokens(tokens),
            Type::Paren(value) => value.to_tokens(tokens),
            Type::Slice(value) => value.to_tokens(tokens),
            Type::ImplTrait(value) => value.to_tokens(tokens),
            Type::TraitObject(value) => value.to_tokens(tokens),
            Type::BareFn(value) => value.to_tokens(tokens),
            Type::Array(value) => value.to_tokens(tokens),
            Type::Macro(value) => value.to_tokens(tokens),
            Type::Never => crate::token::punct::Not::default().to_tokens(tokens),
            Type::Infer => crate::token::Ident::new("_", Span::default()).to_tokens(tokens),
            // `Group` is only produced via the proc-macro bridge, never `from_str`.
            Type::Group(_) => {}
        }
    }
}

fn is_group(tt: &crate::TokenTree, delim: Delim) -> bool {
    matches!(tt, crate::TokenTree::Group(g) if g.delim() == delim)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::token::ToTokenStream;

    fn parse(src: &str) -> Type {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<Type>().unwrap()
    }

    fn roundtrip(src: &str) -> String {
        parse(src).to_token_stream().to_string()
    }

    #[test]
    fn never_infer_array_macro() {
        assert!(matches!(parse("!"), Type::Never));
        assert!(matches!(parse("_"), Type::Infer));
        assert!(matches!(parse("[u8; 4]"), Type::Array(_)));
        assert!(matches!(parse("[u8]"), Type::Slice(_)));
        assert!(matches!(parse("m!(x)"), Type::Macro(_)));
        assert_eq!(roundtrip("[u8 ; 4]"), "[u8 ; 4]");
    }

    #[test]
    fn fn_trait_bounds() {
        assert!(matches!(parse("Fn(u8) -> bool"), Type::Path(_)));
        // `Box<dyn Fn(u8) -> bool>` and `dyn FnMut()` should parse.
        assert!(matches!(parse("Box<dyn Fn(u8) -> bool>"), Type::Path(_)));
        assert!(matches!(parse("dyn FnMut()"), Type::TraitObject(_)));
    }

    #[test]
    fn reference() {
        assert!(matches!(parse("&'a T"), Type::Reference { .. }));
        assert!(matches!(parse("&mut T"), Type::Reference { .. }));
        assert!(matches!(parse("&T"), Type::Reference { .. }));
    }

    #[test]
    fn pointer() {
        assert!(matches!(parse("*const T"), Type::Pointer { .. }));
        assert!(matches!(parse("*mut T"), Type::Pointer { .. }));
        assert!(parse_err("*T"));
    }

    fn parse_err(src: &str) -> bool {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<Type>().is_err()
    }

    #[test]
    fn slice() {
        assert!(matches!(parse("[T]"), Type::Slice { .. }));
    }

    #[test]
    fn paren_vs_tuple() {
        assert!(matches!(parse("(T)"), Type::Paren { .. }));
        assert!(matches!(parse("(A, B)"), Type::Tuple { .. }));
        assert!(matches!(parse("(T,)"), Type::Tuple { .. }));
        assert!(matches!(parse("()"), Type::Tuple { .. }));
    }

    #[test]
    fn roundtrips() {
        // (source, rendered) — Display spaces top-level tokens; the lifetime
        // tick is the only glued case.
        for (src, rendered) in [
            ("&'a T", "& 'a T"),
            ("&mut T", "& mut T"),
            ("*const T", "* const T"),
            ("*mut T", "* mut T"),
            ("[T]", "[T]"),
            ("(T)", "(T)"),
            ("(A, B)", "(A , B)"),
        ] {
            assert_eq!(roundtrip(src), rendered, "roundtrip mismatch for {src}");
        }
    }

    #[test]
    fn path() {
        assert!(matches!(parse("T"), Type::Path { .. }));
        assert!(matches!(parse("std::vec::Vec"), Type::Path { .. }));
        assert_eq!(roundtrip("std :: vec :: Vec"), "std :: vec :: Vec");
    }

    #[test]
    fn qualified_path() {
        assert!(matches!(parse("<T as Trait>::Item"), Type::Path { .. }));
        // `TokenStream` Display spaces top-level tokens (the lifetime tick is the
        // only glued case), so the rendered forms carry spaces.
        assert_eq!(roundtrip("<T as Trait>::Item"), "< T as Trait > :: Item");
        assert_eq!(roundtrip("<T>::Item"), "< T > :: Item");
    }

    #[test]
    fn nested() {
        assert!(matches!(parse("&[T]"), Type::Reference { .. }));
        assert_eq!(roundtrip("&[T]"), "& [T]");
        assert_eq!(roundtrip("(A, B)"), "(A , B)");
    }

    #[test]
    fn from_variant() {
        let s = TypeSlice {
            span: Span::default(),
            elem: Box::new(parse("T")),
        };
        assert!(matches!(Type::from(s), Type::Slice { .. }));
    }
}
