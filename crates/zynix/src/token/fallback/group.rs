use super::TokenStream;
use crate::token::lex::{Cursor, LexError, Scan};
use crate::{Delim, DelimSpan, Span};

#[derive(Debug, Clone)]
pub struct Group {
    pub(crate) delim: Delim,
    pub(crate) span: DelimSpan,
    pub(crate) tokens: TokenStream,
}

impl Group {
    pub fn new(delim: Delim, stream: TokenStream) -> Self {
        Self {
            delim,
            span: DelimSpan::new(Span::call_site(), Span::call_site()),
            tokens: stream,
        }
    }

    pub fn delim(&self) -> Delim {
        self.delim
    }

    pub fn span(&self) -> DelimSpan {
        self.span
    }

    pub fn stream(&self) -> TokenStream {
        self.tokens.clone()
    }

    pub fn set_span(&mut self, span: DelimSpan) {
        self.span = span;
    }
}

impl From<proc_macro::Group> for Group {
    fn from(value: proc_macro::Group) -> Self {
        let mut group = Self::new(value.delimiter().into(), value.stream().into());

        group.set_span(DelimSpan::new(
            value.span_open().into(),
            value.span_close().into(),
        ));

        group
    }
}

impl From<Group> for proc_macro::Group {
    fn from(value: Group) -> Self {
        proc_macro::Group::new(value.delim.into(), value.tokens.into())
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.tokens)
    }
}

impl Scan for Group {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let ch = cursor.first().ok_or(cursor.error())?;
        let delim = Delim::from_open(ch).ok_or(cursor.error())?;
        let c = cursor.advance(ch.len_utf8());
        // Scan inner tokens until matching close delimiter
        let (c, inner) = TokenStream::scan(c)?;
        let close_ch = c.first().ok_or_else(|| {
            cursor
                .error()
                .message(format!("unclosed delimiter '{}'", delim.open()))
        })?;

        let close_delim = Delim::from_close(close_ch).ok_or_else(|| {
            c.error().message(format!(
                "expected '{}', found '{}'",
                delim.close(),
                close_ch
            ))
        })?;

        if delim != close_delim {
            return Err(c.error().message(format!(
                "mismatched delimiter: expected '{}', found '{}'",
                delim.close(),
                close_ch,
            )));
        }

        let c = c.advance(close_ch.len_utf8());
        let mut group = Self::new(delim, inner);
        group.set_span(DelimSpan::new(cursor.span(), c.span()));

        Ok((c, group))
    }
}

impl crate::ToTokens for Group {
    fn to_tokens(&self, tokens: &mut crate::TokenStream) {
        tokens.extend_one(crate::Group::from(self.clone()).into());
    }
}
