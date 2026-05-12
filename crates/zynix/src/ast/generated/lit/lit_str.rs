#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitStr {
    pub span: crate::Span,
    pub value: String,
}
impl crate::ast::Visit for LitStr {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_str(self);
    }
}
impl crate::ast::Fold for LitStr {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_str(self)
    }
}

impl crate::Parse for LitStr {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        let span = stream.span();
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Literal(lit))) => {
                let repr = format!("{}", lit);
                if repr.starts_with('"') && repr.ends_with('"') && repr.len() >= 2 {
                    let value = repr[1..repr.len() - 1].to_string();
                    Ok(Self {
                        span: lit.span(),
                        value,
                    })
                } else {
                    Err(crate::parse::ParseError::new(
                        span,
                        "expected string literal",
                    ))
                }
            }
            _ => Err(crate::parse::ParseError::new(
                span,
                "expected string literal",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_lit_str() {
        let ts = crate::TokenStream::from_str(r#""hello""#).unwrap();
        let mut ps = ts.parse();
        let lit = ps.parse::<LitStr>().unwrap();
        assert_eq!(lit.value, "hello");
    }
}
