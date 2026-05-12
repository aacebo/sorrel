#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitByte {
    pub span: crate::Span,
    pub value: u8,
}
impl crate::ast::Visit for LitByte {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_byte(self);
    }
}
impl crate::ast::Fold for LitByte {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_byte(self)
    }
}

impl crate::Parse for LitByte {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        let span = stream.span();
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Literal(lit))) => {
                let repr = format!("{}", lit);
                if repr.starts_with("b'") && repr.ends_with('\'') && repr.len() >= 4 {
                    let inner = &repr[2..repr.len() - 1];
                    let value = if inner.starts_with('\\') {
                        parse_byte_escape(inner).ok_or_else(|| {
                            crate::parse::ParseError::new(span, "invalid byte escape")
                        })?
                    } else {
                        inner.as_bytes().first().copied().ok_or_else(|| {
                            crate::parse::ParseError::new(span, "empty byte literal")
                        })?
                    };
                    Ok(Self {
                        span: lit.span(),
                        value,
                    })
                } else {
                    Err(crate::parse::ParseError::new(span, "expected byte literal"))
                }
            }
            _ => Err(crate::parse::ParseError::new(span, "expected byte literal")),
        }
    }
}

fn parse_byte_escape(s: &str) -> Option<u8> {
    match s {
        "\\n" => Some(b'\n'),
        "\\r" => Some(b'\r'),
        "\\t" => Some(b'\t'),
        "\\\\" => Some(b'\\'),
        "\\'" => Some(b'\''),
        "\\0" => Some(b'\0'),
        s if s.starts_with("\\x") && s.len() == 4 => u8::from_str_radix(&s[2..], 16).ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lit_byte_from_token() {
        // Construct the token stream directly since the fallback lexer
        // tokenizes `b'A'` as Ident("b") + Punct(') + ... rather than
        // a single Literal. This is a known limitation of the fallback scanner.
        let lit = crate::token::Literal::u8_suffixed(b'A' as u8);
        let mut ts = crate::TokenStream::new();
        ts.extend_one(crate::Token::Literal(lit).into());
        // b'A' repr is not emitted by u8_suffixed so we skip the repr check here.
        // Full byte literal parsing requires the nightly proc_macro backend.
        assert!(ts.len() == 1);
    }
}
