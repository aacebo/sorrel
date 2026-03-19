use crate::token::lex::{Cursor, LexError, Scan};
use crate::{Delim, DelimSpan, TokenStream};

#[derive(Debug, Clone)]
pub struct Group {
    pub(crate) delim: Delim,
    pub(crate) span: DelimSpan,
    pub(crate) tokens: TokenStream,
}

impl Group {
    pub fn new(delim: Delim, mut stream: TokenStream) -> Self {
        Self {
            delim,
            span: stream.delim(),
            tokens: stream,
        }
    }

    pub fn delim(&self) -> Delim {
        self.delim
    }

    pub fn span(&self) -> DelimSpan {
        self.span
    }

    pub fn as_tokens(&self) -> &TokenStream {
        &self.tokens
    }
}

impl From<proc_macro::Group> for Group {
    fn from(value: proc_macro::Group) -> Self {
        Self::new(value.delimiter().into(), value.stream().into())
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
        let (c, inner) = super::TokenStream::scan(c)?;
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
        let stream: TokenStream = inner.into();
        Ok((c, Self::new(delim, stream)))
    }
}
