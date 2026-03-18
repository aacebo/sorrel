use crate::{MultiSpan, Span};

#[derive(Debug, Copy, Clone)]
pub struct RangeSpan {
    start: Span,
    end: Span,
}

impl RangeSpan {
    pub fn new(start: Span, end: Span) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Span {
        self.start
    }

    pub fn end(&self) -> Span {
        self.end
    }

    pub fn span(&self) -> Span {
        self.start.join(self.end)
    }
}

impl From<RangeSpan> for Span {
    fn from(value: RangeSpan) -> Self {
        value.span()
    }
}

impl MultiSpan for RangeSpan {
    fn into_spans(self) -> Vec<Span> {
        vec![self.start, self.end]
    }
}
