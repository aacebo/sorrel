#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitFloat {
    pub span: crate::Span,
    pub digits: String,
    pub suffix: Option<Ident>,
}
impl crate::ast::Visit for LitFloat {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_float(self);
    }
}
impl crate::ast::Fold for LitFloat {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_float(self)
    }
}

impl crate::Parse for LitFloat {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        let span = stream.span();
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Literal(lit))) => {
                let repr = format!("{}", lit);
                let first = repr.chars().next();
                let is_float = first.is_some_and(|c| c.is_ascii_digit())
                    && (repr.contains('.') || repr.to_ascii_lowercase().contains('e'));
                if is_float {
                    let (digits, suffix_text) = split_float_suffix(&repr);
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
                        "expected float literal",
                    ))
                }
            }
            _ => Err(crate::parse::ParseError::new(
                span,
                "expected float literal",
            )),
        }
    }
}

fn split_float_suffix(repr: &str) -> (&str, Option<&str>) {
    for suffix in &["f64", "f32"] {
        if repr.ends_with(suffix) {
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
    fn parse_plain_float() {
        let ts = crate::TokenStream::from_str("3.14").unwrap();
        let mut ps = ts.parse();
        let lit = ps.parse::<LitFloat>().unwrap();
        assert_eq!(lit.digits, "3.14");
        assert!(lit.suffix.is_none());
    }

    #[test]
    fn parse_suffixed_float() {
        let ts = crate::TokenStream::from_str("3.14f64").unwrap();
        let mut ps = ts.parse();
        let lit = ps.parse::<LitFloat>().unwrap();
        assert_eq!(lit.digits, "3.14");
        assert_eq!(lit.suffix.as_ref().map(|s| s.text.as_str()), Some("f64"));
    }
}
