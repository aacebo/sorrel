#[allow(unused)]
use super::*;
#[doc = "An identifier token (e.g. a variable name, type name, or keyword-like ident)."]
#[derive(Debug, Clone)]
pub struct Ident {
    pub span: crate::Span,
    pub text: String,
    pub raw: bool,
}
impl crate::ast::Visit for Ident {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_ident(self);
    }
}
impl crate::ast::Fold for Ident {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_ident(self)
    }
}

impl crate::Parse for Ident {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        let span = stream.span();
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Ident(id))) => Ok(Self {
                span: id.span(),
                text: id.name().into_owned(),
                raw: false,
            }),
            _ => Err(crate::parse::ParseError::new(span, "expected identifier")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_ident() {
        let ts = crate::TokenStream::from_str("foo").unwrap();
        let mut ps = ts.parse();
        let id = ps.parse::<Ident>().unwrap();
        assert_eq!(id.text, "foo");
        assert!(!id.raw);
    }
}
