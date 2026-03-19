use crate::{Delim, Group, Ident, Literal, Punct, Spacing, Span, ToTokens, Token, TokenStream};

#[derive(Debug)]
pub enum ParseError {
    Compiler(proc_macro::LexError),
    Fallback(proc_macro2::LexError),
    #[cfg(feature = "report")]
    Diagnostic(crate::report::Diagnostic),
}

impl ParseError {
    pub fn span(&self) -> Option<Span> {
        match self {
            Self::Compiler(_) => None,
            Self::Fallback(v) => Some(v.span().into()),
            #[cfg(feature = "report")]
            Self::Diagnostic(v) => v.spans().first().cloned(),
        }
    }

    pub fn to_compile_error(&self) -> TokenStream {
        let ident = Ident::new("compile_error", self.span().unwrap_or_default());
        let mut bang = Punct::new('!', Spacing::Alone);
        let mut lit = Literal::string(&self.to_string());

        if let Some(span) = self.span() {
            bang.set_span(span);
            lit.set_span(span);
        }

        let inner = Token::Literal(lit).into_token_stream();
        let group = Group::new(Delim::Paren, inner);
        vec![Token::Ident(ident), Token::Punct(bang), Token::Group(group)].into()
    }
}

impl From<proc_macro::LexError> for ParseError {
    fn from(value: proc_macro::LexError) -> Self {
        Self::Compiler(value)
    }
}

impl From<proc_macro2::LexError> for ParseError {
    fn from(value: proc_macro2::LexError) -> Self {
        Self::Fallback(value)
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
            Self::Compiler(v) => write!(f, "{}", v),
            Self::Fallback(v) => write!(f, "{}", v),
            #[cfg(feature = "report")]
            Self::Diagnostic(v) => write!(f, "{}", v),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(not(nightly))]
impl ToTokens for ParseError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        #[cfg(feature = "report")]
        if let Self::Diagnostic(d) = self {
            d.to_tokens(tokens);
            return;
        }

        self.to_compile_error().to_tokens(tokens);
    }
}

#[cfg(nightly)]
impl proc_macro::ToTokens for ParseError {
    fn to_tokens(&self, tokens: &mut proc_macro::TokenStream) {
        let stream = self.to_compile_error();
        tokens.extend(proc_macro::TokenStream::from(stream));
    }
}
