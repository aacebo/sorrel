use std::borrow::Cow;

use crate::Span;

#[derive(Debug, Clone)]
pub struct Ident {
    pub(crate) name: Box<str>,
    pub(crate) span: Span,
}

impl Ident {
    pub fn new(name: &str, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }

    pub fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.name.as_ref())
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl From<proc_macro::Ident> for Ident {
    fn from(value: proc_macro::Ident) -> Self {
        Self::new(&value.to_string(), value.span().into())
    }
}

impl From<Ident> for proc_macro::Ident {
    fn from(value: Ident) -> Self {
        proc_macro::Ident::new(&value.name, value.span.into())
    }
}

impl crate::token::lex::Scan for Ident {
    fn scan(
        cursor: crate::token::lex::Cursor<'_>,
    ) -> Result<(crate::token::lex::Cursor<'_>, Self), crate::token::lex::LexError> {
        // Raw ident: r#ident
        if cursor.starts_with("r#") {
            let after = cursor.advance(2);
            let end = after.skip_while(unicode_ident::is_xid_continue);

            if end.offset() == after.offset() {
                return cursor.error().into();
            }

            let span = cursor.span_to(&end);
            let name = &cursor.rest()[..end.offset() as usize - cursor.offset() as usize];
            return Ok((end, Self::new(name, span)));
        }

        let first = cursor.first().ok_or(cursor.error())?;

        if first != '_' && !unicode_ident::is_xid_start(first) {
            return cursor.error().into();
        }

        let end = cursor
            .advance(first.len_utf8())
            .skip_while(unicode_ident::is_xid_continue);
        let span = cursor.span_to(&end);
        let name = &cursor.rest()[..end.offset() as usize - cursor.offset() as usize];
        Ok((end, Self::new(name, span)))
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl crate::ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut crate::TokenStream) {
        use crate::Token;

        tokens.extend_one(Token::from(crate::Ident::from(self.clone())));
    }
}
