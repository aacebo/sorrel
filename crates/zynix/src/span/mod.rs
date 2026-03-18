mod delim;
mod range;

pub use delim::*;
pub use range::*;

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

impl MultiSpan for Span {
    fn into_spans(self) -> Vec<Span> {
        vec![self]
    }
}

impl proc_macro::MultiSpan for Span {
    fn into_spans(self) -> Vec<proc_macro::Span> {
        vec![proc_macro2::Span::from(self).unwrap()]
    }
}
