use zynix_macros::{Parse, ToTokens};

use crate::Span;
use crate::ast::Punctuated;
use crate::token::punct::PathSep;

mod lifetime;
mod lifetime_name;
mod path_arguments;
mod path_segment;

pub use lifetime::*;
pub use lifetime_name::*;
pub use path_arguments::*;
pub use path_segment::*;

#[doc = "A path expression or type path (e.g. `std::collections::HashMap`, `crate::Foo`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
pub struct Path {
    #[parse(skip)]
    pub span: Span,
    #[parse(peek = PathSep)]
    pub leading_colon: bool,
    #[parse(separated)]
    pub segments: Punctuated<PathSegment, PathSep>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::ToTokenStream;
    use crate::{Parse, TokenStream};
    use std::str::FromStr;

    fn parse<T: Parse>(src: &str) -> T {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>().unwrap()
    }

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn lifetime() {
        let l: Lifetime = parse("'a");
        assert_eq!(l.ident.text, "a");
        assert_eq!(render(&l), "'a");

        let s: Lifetime = parse("'static");
        assert_eq!(s.ident.text, "static");
        assert_eq!(render(&s), "'static");
    }

    #[test]
    fn simple_path() {
        let p: Path = parse("Foo");
        assert!(!p.leading_colon);
        assert_eq!(p.segments.len(), 1);
        assert_eq!(render(&p), "Foo");
    }

    #[test]
    fn multi_segment() {
        let p: Path = parse("std::collections::HashMap");
        assert_eq!(p.segments.len(), 3);
        assert_eq!(render(&p), "std :: collections :: HashMap");
    }

    #[test]
    fn leading_colon() {
        let p: Path = parse("::core::mem");
        assert!(p.leading_colon);
        assert_eq!(p.segments.len(), 2);
        assert_eq!(render(&p), ":: core :: mem");
    }

    #[test]
    fn angle_bracketed() {
        let p: Path = parse("Vec<T>");
        assert_eq!(p.segments.len(), 1);
        assert!(matches!(
            p.segments.first().unwrap().args,
            PathArguments::AngleBracketed(_)
        ));
        assert_eq!(render(&p), "Vec < T >");
    }

    #[test]
    fn path_arguments_from_token_stream() {
        let ts = TokenStream::from_str("T").unwrap();
        assert!(matches!(
            PathArguments::from(ts),
            PathArguments::AngleBracketed(_)
        ));
    }
}
