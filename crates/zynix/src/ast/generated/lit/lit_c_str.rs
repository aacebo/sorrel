#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitCStr {
    pub span: crate::Span,
    pub value: Vec<u8>,
}
impl crate::ast::Visit for LitCStr {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_c_str(self);
    }
}
impl crate::ast::Fold for LitCStr {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_c_str(self)
    }
}

impl crate::Parse for LitCStr {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        let span = stream.span();
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Literal(lit))) => {
                let repr = format!("{}", lit);
                if repr.starts_with("c\"") && repr.ends_with('"') && repr.len() >= 3 {
                    let inner = &repr[2..repr.len() - 1];
                    Ok(Self {
                        span: lit.span(),
                        value: inner.bytes().collect(),
                    })
                } else {
                    Err(crate::parse::ParseError::new(
                        span,
                        "expected C string literal",
                    ))
                }
            }
            _ => Err(crate::parse::ParseError::new(
                span,
                "expected C string literal",
            )),
        }
    }
}

// Note: the fallback lexer tokenizes `c"..."` as Ident("c") + the string,
// not as a single C string literal. Full parsing requires the nightly backend.
