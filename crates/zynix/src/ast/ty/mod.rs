use crate::ast::{MacroCall, Punctuated};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::{And, Comma, Star};
use crate::token::{Delim, ToTokens};
use crate::{Parse, Span, TokenStream};

mod q_self;
mod type_array;
mod type_bare_fn;
mod type_group;
mod type_impl_trait;
mod type_param;
mod type_paren;
mod type_path;
mod type_pointer;
mod type_predicate;
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
pub use type_param::*;
pub use type_paren::*;
pub use type_path::*;
pub use type_pointer::*;
pub use type_predicate::*;
pub use type_reference::*;
pub use type_slice::*;
pub use type_trait_object::*;
pub use type_tuple::*;
pub use typed_param::*;

#[doc = "A Rust type expression. Covers all positions where a type can appear in source code."]
#[derive(Debug, Clone)]
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

        if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Bracket)) {
            return Ok(Type::Slice(stream.parse()?));
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

        // Otherwise a path type: `T`, `std::vec::Vec`, or a qualified
        // `<T as Trait>::Item` (which begins with `<`).
        Ok(Type::Path(stream.parse()?))
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
            // Variants below are not yet produced by `Type::parse`.
            _ => {}
        }
    }
}

fn is_group(tt: &crate::TokenTree, delim: Delim) -> bool {
    matches!(tt, crate::TokenTree::Group(g) if g.delim() == delim)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::ToTokenStream;
    use std::str::FromStr;

    fn parse(src: &str) -> Type {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<Type>().unwrap()
    }

    fn roundtrip(src: &str) -> String {
        parse(src).to_token_stream().to_string()
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
