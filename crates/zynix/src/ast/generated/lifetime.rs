#[allow(unused)]
use super::*;
#[doc = "A named lifetime (e.g. `'a`, `'static`)."]
#[derive(Debug, Clone)]
pub struct Lifetime {
    pub span: crate::Span,
    pub apostrophe: crate::Span,
    pub ident: Ident,
}
impl crate::ast::Visit for Lifetime {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lifetime(self);
    }
}
impl crate::ast::Fold for Lifetime {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lifetime(self)
    }
}

impl crate::Parse for Lifetime {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        let span = stream.span();
        // The fallback lexer emits `'` as Alone spacing (next char is alphanumeric),
        // so we match on any `'` punct regardless of spacing.
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Punct(p))) if p.as_char() == '\'' => {
                let apostrophe = p.span();
                let ident: Ident = stream.parse()?;
                Ok(Self {
                    span: apostrophe,
                    apostrophe,
                    ident,
                })
            }
            _ => Err(crate::parse::ParseError::new(span, "expected lifetime")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_lifetime() {
        let ts = crate::TokenStream::from_str("'a").unwrap();
        let mut ps = ts.parse();
        let lt = ps.parse::<Lifetime>().unwrap();
        assert_eq!(lt.ident.text, "a");
    }

    #[test]
    fn parse_static_lifetime() {
        let ts = crate::TokenStream::from_str("'static").unwrap();
        let mut ps = ts.parse();
        let lt = ps.parse::<Lifetime>().unwrap();
        assert_eq!(lt.ident.text, "static");
    }
}
