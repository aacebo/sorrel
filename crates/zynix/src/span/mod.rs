mod delim;
mod range;

pub use delim::*;
pub use range::*;

pub trait Spanner {
    fn span(&self) -> Span;
    fn into_spans(self) -> SpanSet
    where
        Self: Sized,
    {
        SpanSet::new(self.span())
    }
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

    pub fn range(from: Self, to: Self) -> Self {
        from.join(to)
    }

    pub fn join(&self, other: Self) -> Self {
        self.0.join(other.0).map(|v| v.into()).unwrap_or(other)
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::call_site()
    }
}

impl Eq for Span {}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.start() == other.start() && self.end() == other.end()
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start().cmp(&other.start()) {
            std::cmp::Ordering::Equal => self.end().cmp(&other.end()),
            ord => ord,
        }
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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

impl proc_macro::MultiSpan for Span {
    fn into_spans(self) -> Vec<proc_macro::Span> {
        vec![proc_macro2::Span::from(self).unwrap()]
    }
}

impl Spanner for Span {
    fn span(&self) -> Span {
        *self
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SpanSet {
    primary: Option<Span>,
    secondary: Vec<Span>,
}

impl SpanSet {
    pub fn new(primary: Span) -> Self {
        Self {
            primary: Some(primary),
            secondary: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.primary.is_none() && self.secondary.is_empty()
    }

    pub fn primary(&self) -> Option<Span> {
        self.primary
    }

    pub fn secondary(&self) -> &[Span] {
        &self.secondary
    }

    pub fn span(&self) -> Span {
        self.primary
            .or(self.secondary.first().copied())
            .unwrap_or_default()
    }

    pub fn add(mut self, other: Span) -> Self {
        if self.primary.is_none() {
            self.primary = Some(other);
            self
        } else {
            self.secondary.push(other);
            self
        }
    }

    pub fn join(mut self, other: Self) -> Self {
        self.primary = self.primary.or(other.primary);
        self.secondary.extend(other.secondary);
        self
    }
}

impl From<Span> for SpanSet {
    fn from(value: Span) -> Self {
        Self {
            primary: Some(value),
            secondary: vec![],
        }
    }
}

impl Spanner for SpanSet {
    fn span(&self) -> Span {
        self.span()
    }

    fn into_spans(self) -> SpanSet {
        self
    }
}

impl proc_macro::MultiSpan for SpanSet {
    fn into_spans(self) -> Vec<proc_macro::Span> {
        let mut spans = vec![];

        if let Some(primary) = self.primary {
            spans.push(proc_macro2::Span::from(primary).unwrap());
        }

        for span in self.secondary {
            spans.push(proc_macro2::Span::from(span).unwrap());
        }

        spans
    }
}

impl proc_macro::MultiSpan for &SpanSet {
    fn into_spans(self) -> Vec<proc_macro::Span> {
        let mut spans = vec![];

        if let Some(primary) = self.primary {
            spans.push(proc_macro2::Span::from(primary).unwrap());
        }

        for span in self.secondary.iter().copied() {
            spans.push(proc_macro2::Span::from(span).unwrap());
        }

        spans
    }
}
