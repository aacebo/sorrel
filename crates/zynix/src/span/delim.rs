use crate::Span;

#[derive(Debug, Copy, Clone)]
pub struct DelimSpan {
    open: Span,
    close: Span,
}

impl DelimSpan {
    pub fn new(open: Span, close: Span) -> Self {
        Self { open, close }
    }

    pub fn open(&self) -> Span {
        self.open
    }

    pub fn close(&self) -> Span {
        self.close
    }

    pub fn span(&self) -> Span {
        self.open.join(self.close)
    }
}

impl From<proc_macro2::extra::DelimSpan> for DelimSpan {
    fn from(value: proc_macro2::extra::DelimSpan) -> Self {
        Self {
            open: value.open().into(),
            close: value.close().into(),
        }
    }
}

impl From<DelimSpan> for Span {
    fn from(value: DelimSpan) -> Self {
        value.span()
    }
}
