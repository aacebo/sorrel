#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LifetimeName {
    pub span: crate::Span,
    pub text: String,
}
impl crate::ast::Visit for LifetimeName {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lifetime_name(self);
    }
}
impl crate::ast::Fold for LifetimeName {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lifetime_name(self)
    }
}

impl crate::Parse for LifetimeName {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        let span = stream.span();
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Punct(p))) if p.as_char() == '\'' => {
                let tok_span = p.span();
                match stream.advance() {
                    Some(crate::TokenTree::Token(crate::Token::Ident(id))) => Ok(Self {
                        span: tok_span,
                        text: id.name().into_owned(),
                    }),
                    _ => Err(crate::parse::ParseError::new(
                        span,
                        "expected lifetime name",
                    )),
                }
            }
            _ => Err(crate::parse::ParseError::new(
                span,
                "expected lifetime name",
            )),
        }
    }
}
