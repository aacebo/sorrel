#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitInt {
    pub span: crate::Span,
    pub digits: String,
    pub suffix: Option<Ident>,
}
impl crate::ast::Visit for LitInt {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_int(self);
    }
}
impl crate::ast::Fold for LitInt {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_int(self)
    }
}

impl crate::Parse for LitInt {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        let span = stream.span();
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Literal(lit))) => {
                let repr = format!("{}", lit);
                let first = repr.chars().next();
                if first.is_some_and(|c| c.is_ascii_digit()) && !repr.contains('.') {
                    let (digits, suffix_text) = split_num_suffix(&repr);
                    let suffix = suffix_text.map(|s| Ident {
                        span: lit.span(),
                        text: s.to_string(),
                        raw: false,
                    });
                    Ok(Self {
                        span: lit.span(),
                        digits: digits.to_string(),
                        suffix,
                    })
                } else {
                    Err(crate::parse::ParseError::new(
                        span,
                        "expected integer literal",
                    ))
                }
            }
            _ => Err(crate::parse::ParseError::new(
                span,
                "expected integer literal",
            )),
        }
    }
}

/// Splits a numeric repr into (digits, optional suffix).
/// Integer suffixes are: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize.
/// Hex/binary/octal digits (a-f, A-F, x, b, o) are not suffixes.
fn split_num_suffix(repr: &str) -> (&str, Option<&str>) {
    // Find trailing suffix: a run of [a-zA-Z0-9] that starts with u or i
    // and is not part of a hex/bin/oct prefix (which appear at the start).
    // Strategy: scan from the end for a known suffix pattern.
    let lower = repr.to_ascii_lowercase();
    let suffixes = [
        "u128", "i128", "usize", "isize", "u64", "i64", "u32", "i32", "u16", "i16", "u8", "i8",
    ];
    for suffix in &suffixes {
        if lower.ends_with(suffix) {
            let digits_end = repr.len() - suffix.len();
            return (&repr[..digits_end], Some(&repr[digits_end..]));
        }
    }
    (repr, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_plain_int() {
        let ts = crate::TokenStream::from_str("42").unwrap();
        let mut ps = ts.parse();
        let lit = ps.parse::<LitInt>().unwrap();
        assert_eq!(lit.digits, "42");
        assert!(lit.suffix.is_none());
    }

    #[test]
    fn parse_suffixed_int() {
        let ts = crate::TokenStream::from_str("42u32").unwrap();
        let mut ps = ts.parse();
        let lit = ps.parse::<LitInt>().unwrap();
        assert_eq!(lit.digits, "42");
        assert_eq!(lit.suffix.as_ref().map(|s| s.text.as_str()), Some("u32"));
    }

    #[test]
    fn parse_hex_int() {
        let ts = crate::TokenStream::from_str("0xFF").unwrap();
        let mut ps = ts.parse();
        let lit = ps.parse::<LitInt>().unwrap();
        assert_eq!(lit.digits, "0xFF");
        assert!(lit.suffix.is_none());
    }
}
