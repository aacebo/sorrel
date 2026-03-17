use crate::{DelimSpan, Iter, Span, Token};

#[derive(Debug, Default, Clone)]
pub struct TokenBuffer(Vec<Token>);

impl TokenBuffer {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.0.iter()
    }

    pub fn first(&self) -> Span {
        self.0
            .first()
            .map(|v| v.span())
            .unwrap_or(Span::call_site())
    }

    pub fn last(&self) -> Span {
        self.0.last().map(|v| v.span()).unwrap_or(Span::call_site())
    }

    pub fn span(&self) -> Span {
        self.first().join(self.last()).unwrap_or(Span::call_site())
    }

    pub fn delim(&self) -> DelimSpan {
        DelimSpan::new(self.first(), self.last())
    }
}

impl FromIterator<Token> for TokenBuffer {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromIterator<Self> for TokenBuffer {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        Self(iter.into_iter().flatten().collect())
    }
}

impl IntoIterator for TokenBuffer {
    type IntoIter = Iter;
    type Item = Token;

    fn into_iter(self) -> Self::IntoIter {
        Iter::from(self.0.into_iter())
    }
}

impl std::fmt::Display for TokenBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.0.iter() {
            write!(f, "{}", token)?;
        }

        Ok(())
    }
}
