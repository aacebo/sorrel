/// Trait implemented by types that can be converted into a set of `Spans`.
pub trait MultiSpan {
    fn into_spans(self) -> Vec<Span>;
}

#[derive(Debug, Copy, Clone)]
pub struct Span(proc_macro2::Span);

impl Span {
    pub fn call_site() -> Self {
        proc_macro2::Span::call_site().into()
    }

    pub fn mixed_site() -> Self {
        proc_macro2::Span::mixed_site().into()
    }

    pub fn join(&self, other: Self) -> Option<Self> {
        self.0.join(other.0).map(|v| v.into())
    }
}

impl From<proc_macro2::Span> for Span {
    fn from(value: proc_macro2::Span) -> Self {
        Self(value)
    }
}

impl From<Span> for proc_macro2::Span {
    fn from(value: Span) -> Self {
        value.0
    }
}

impl std::ops::Deref for Span {
    type Target = proc_macro2::Span;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Span {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MultiSpan for Span {
    fn into_spans(self) -> Vec<Span> {
        vec![self]
    }
}

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
        self.open.join(self.close).unwrap_or(Span::call_site())
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

impl MultiSpan for DelimSpan {
    fn into_spans(self) -> Vec<Span> {
        vec![self.span()]
    }
}
