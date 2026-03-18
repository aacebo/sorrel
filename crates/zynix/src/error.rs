use crate::{Delim, Group, Ident, Literal, Punct, Spacing, Span, Stream, ToStream, Token};

#[derive(Debug)]
pub enum ParseError {
    Lex(proc_macro2::LexError),
    #[cfg(feature = "report")]
    Diagnostic(crate::report::Diagnostic),
}

impl ParseError {
    pub fn span(&self) -> Option<Span> {
        match self {
            Self::Lex(v) => Some(v.span().into()),
            #[cfg(feature = "report")]
            Self::Diagnostic(v) => v.spans().first().cloned(),
        }
    }

    pub fn to_compile_error(&self) -> Stream {
        let ident = Ident::new("compile_error", self.span().unwrap_or_default());
        let mut bang = Punct::new('!', Spacing::Alone);
        let mut lit = Literal::string(&self.to_string());

        if let Some(span) = self.span() {
            bang.set_span(span);
            lit.set_span(span);
        }

        let inner = Token::Literal(lit).to_stream();
        let group = Group::new(Delim::Paren, inner);
        vec![Token::Ident(ident), Token::Punct(bang), Token::Group(group)].into()
    }
}

impl From<proc_macro2::LexError> for ParseError {
    fn from(value: proc_macro2::LexError) -> Self {
        Self::Lex(value)
    }
}

#[cfg(feature = "report")]
impl From<crate::report::Diagnostic> for ParseError {
    fn from(value: crate::report::Diagnostic) -> Self {
        Self::Diagnostic(value)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lex(v) => write!(f, "{}", v),
            #[cfg(feature = "report")]
            Self::Diagnostic(v) => write!(f, "{}", v),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Lex(v) => Some(v),
            #[cfg(feature = "report")]
            _ => None,
        }
    }
}

impl ToStream for ParseError {
    fn to_stream(self) -> Stream {
        match self {
            #[cfg(feature = "report")]
            Self::Diagnostic(d) => d.to_stream(),
            err => err.to_compile_error(),
        }
    }
}

#[cfg(nightly)]
impl proc_macro::ToTokens for ParseError {
    fn to_tokens(&self, tokens: &mut proc_macro::TokenStream) {
        let stream = self.to_compile_error();
        let token_stream = proc_macro2::TokenStream::from(stream);
        tokens.extend(proc_macro::TokenStream::from(token_stream));
    }
}
