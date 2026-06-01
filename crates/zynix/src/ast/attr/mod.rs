mod attr_args;
mod attr_style;
mod attribute;
mod doc_string;
mod meta;

pub use attr_args::*;
pub use attr_style::*;
pub use attribute::*;
pub use doc_string::*;
pub use meta::*;

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

    // `TokenStream` Display spaces top-level tokens (only the lifetime tick is
    // glued), so `#`/`#!` render with a space before the `[...]` group.
    #[test]
    fn outer_empty() {
        let a: Attribute = parse("#[inline]");
        assert_eq!(a.style, AttrStyle::Outer);
        assert!(matches!(a.args, AttrArgs::Empty));
        assert_eq!(render(&a), "# [inline]");
    }

    #[test]
    fn outer_delimited() {
        let a: Attribute = parse("#[derive(Clone, Debug)]");
        assert_eq!(a.style, AttrStyle::Outer);
        assert!(matches!(a.args, AttrArgs::Delimited { .. }));
        assert_eq!(render(&a), "# [derive (Clone , Debug)]");
    }

    #[test]
    fn inner() {
        let a: Attribute = parse("#![no_std]");
        assert_eq!(a.style, AttrStyle::Inner);
        assert_eq!(render(&a), "# ! [no_std]");
    }

    #[test]
    fn many() {
        let attrs: Vec<Attribute> = {
            let ts = TokenStream::from_str("#[a] #[b(1)]").unwrap();
            let mut ps = ts.parse();
            let mut out = Vec::new();
            while !ps.is_empty() {
                out.push(ps.parse::<Attribute>().unwrap());
            }
            out
        };
        assert_eq!(attrs.len(), 2);
    }

    #[test]
    fn name_value() {
        let a: Attribute = parse("#[path = \"x.rs\"]");
        assert!(matches!(a.args, AttrArgs::NameValue(_)));
        assert_eq!(render(&a), "# [path = \"x.rs\"]");
    }

    #[test]
    fn cfg_delimited() {
        let a: Attribute = parse("#[cfg(feature = \"x\")]");
        assert!(matches!(a.args, AttrArgs::Delimited { .. }));
    }

    #[test]
    fn meta_forms() {
        assert!(matches!(parse::<Meta>("inline"), Meta::Path(_)));
        assert!(matches!(parse::<Meta>("derive(Clone)"), Meta::List(_)));
        assert!(matches!(parse::<Meta>("path = \"x\""), Meta::NameValue(_)));
    }
}
