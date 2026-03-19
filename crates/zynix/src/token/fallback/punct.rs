use crate::{Spacing, Span, token};

#[derive(Debug, Clone)]
pub struct Punct {
    pub(crate) ch: char,
    pub(crate) spacing: Spacing,
    pub(crate) span: Span,
}

impl Punct {
    pub fn new(ch: char, spacing: Spacing) -> Self {
        Self {
            ch,
            spacing,
            span: Span::default(),
        }
    }

    pub fn as_char(&self) -> char {
        self.ch
    }

    pub fn spacing(&self) -> Spacing {
        self.spacing
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl From<proc_macro::Punct> for Punct {
    fn from(value: proc_macro::Punct) -> Self {
        Self {
            ch: value.as_char(),
            spacing: value.spacing().into(),
            span: value.span().into(),
        }
    }
}

impl From<Punct> for proc_macro::Punct {
    fn from(value: Punct) -> Self {
        let mut p = proc_macro::Punct::new(value.ch, value.spacing.into());
        p.set_span(value.span.into());
        p
    }
}

impl std::fmt::Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ch)
    }
}

impl token::lex::Scan for Punct {
    fn scan(
        cursor: token::lex::Cursor<'_>,
    ) -> Result<(token::lex::Cursor<'_>, Self), token::lex::LexError> {
        let ch = cursor.first().ok_or(cursor.error())?;

        // Reject delimiters and alphanumeric
        if matches!(ch, '(' | ')' | '[' | ']' | '{' | '}') || ch.is_alphanumeric() || ch == '_' {
            return cursor.error().into();
        }

        // Lifetime tick: 'a is a lifetime, emit as punct
        let next = cursor.advance(ch.len_utf8());
        // Determine spacing: Joint if next char is also punct-like (not delimiter, whitespace, or EOF)
        let spacing = match next.first() {
            Some(nc)
                if !nc.is_whitespace()
                    && !matches!(nc, '(' | ')' | '[' | ']' | '{' | '}')
                    && !nc.is_alphanumeric()
                    && nc != '_'
                    && nc != '"'
                    && nc != '\'' =>
            {
                Spacing::Joint
            }
            _ => Spacing::Alone,
        };

        let span = next.span_to(&next);
        let token = Self { ch, spacing, span };
        Ok((next, token))
    }
}
