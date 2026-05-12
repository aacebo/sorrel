#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitBool {
    pub span: crate::Span,
    pub value: bool,
}
impl crate::ast::Visit for LitBool {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_bool(self);
    }
}
impl crate::ast::Fold for LitBool {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_bool(self)
    }
}

impl crate::Parse for LitBool {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        let span = stream.span();
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Ident(id))) => match id.name().as_ref() {
                "true" => Ok(Self {
                    span: id.span(),
                    value: true,
                }),
                "false" => Ok(Self {
                    span: id.span(),
                    value: false,
                }),
                _ => Err(crate::parse::ParseError::new(
                    span,
                    "expected `true` or `false`",
                )),
            },
            _ => Err(crate::parse::ParseError::new(
                span,
                "expected `true` or `false`",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_true() {
        let ts = crate::TokenStream::from_str("true").unwrap();
        let mut ps = ts.parse();
        let lit = ps.parse::<LitBool>().unwrap();
        assert!(lit.value);
    }

    #[test]
    fn parse_false() {
        let ts = crate::TokenStream::from_str("false").unwrap();
        let mut ps = ts.parse();
        let lit = ps.parse::<LitBool>().unwrap();
        assert!(!lit.value);
    }
}
