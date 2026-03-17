use crate::Span;

#[derive(Debug, Clone)]
pub struct Ident {
    name: String,
    span: Span,
}

impl Ident {
    pub fn new(name: &str, span: Span) -> Self {
        Self {
            name: name.to_string(),
            span,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl From<proc_macro2::Ident> for Ident {
    fn from(value: proc_macro2::Ident) -> Self {
        Self {
            name: value.to_string(),
            span: value.span().into(),
        }
    }
}

impl From<Ident> for proc_macro2::Ident {
    fn from(value: Ident) -> Self {
        let mut ident = proc_macro2::Ident::new(&value.name, value.span.into());
        ident.set_span(value.span.into());
        ident
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
