use crate::{
    Delim, Group, Ident, LexError, Literal, ParseError, Punct, Spacing, Span, Token, TokenStream,
    TokenTree,
};

// --- LexError ---

impl From<proc_macro2::LexError> for ParseError {
    fn from(value: proc_macro2::LexError) -> Self {
        Self::Fallback(LexError::new(Span::Fallback(value.span().into())).message(value))
    }
}

// --- Span ---

impl From<proc_macro2::Span> for crate::span::fallback::Span {
    fn from(value: proc_macro2::Span) -> Self {
        Self::new(
            value.byte_range().start as u32,
            value.byte_range().end as u32,
        )
    }
}

impl From<proc_macro2::Span> for Span {
    fn from(value: proc_macro2::Span) -> Self {
        Self::Fallback(value.into())
    }
}

// --- Delim ---

impl From<proc_macro2::Delimiter> for Delim {
    fn from(value: proc_macro2::Delimiter) -> Self {
        match value {
            proc_macro2::Delimiter::Parenthesis => Self::Paren,
            proc_macro2::Delimiter::Brace => Self::Brace,
            proc_macro2::Delimiter::Bracket => Self::Bracket,
            proc_macro2::Delimiter::None => Self::None,
        }
    }
}

impl From<Delim> for proc_macro2::Delimiter {
    fn from(value: Delim) -> Self {
        match value {
            Delim::Paren => proc_macro2::Delimiter::Parenthesis,
            Delim::Brace => proc_macro2::Delimiter::Brace,
            Delim::Bracket => proc_macro2::Delimiter::Bracket,
            Delim::None => proc_macro2::Delimiter::None,
        }
    }
}

// --- Spacing ---

impl From<proc_macro2::Spacing> for Spacing {
    fn from(value: proc_macro2::Spacing) -> Self {
        match value {
            proc_macro2::Spacing::Alone => Self::Alone,
            proc_macro2::Spacing::Joint => Self::Joint,
        }
    }
}

impl From<Spacing> for proc_macro2::Spacing {
    fn from(value: Spacing) -> Self {
        match value {
            Spacing::Alone => proc_macro2::Spacing::Alone,
            Spacing::Joint => proc_macro2::Spacing::Joint,
        }
    }
}

// --- Ident ---

impl From<proc_macro2::Ident> for Ident {
    fn from(value: proc_macro2::Ident) -> Self {
        let span: Span = value.span().into();
        Self::Fallback(crate::token::fallback::Ident::new(&value.to_string(), span))
    }
}

impl From<Ident> for proc_macro2::Ident {
    fn from(value: Ident) -> Self {
        proc_macro2::Ident::new(&value.name(), proc_macro2::Span::call_site())
    }
}

// --- Punct ---

impl From<proc_macro2::Punct> for Punct {
    fn from(value: proc_macro2::Punct) -> Self {
        let span: Span = value.span().into();
        Self::Fallback(crate::token::fallback::Punct {
            ch: value.as_char(),
            spacing: value.spacing().into(),
            span,
        })
    }
}

impl From<Punct> for proc_macro2::Punct {
    fn from(value: Punct) -> Self {
        proc_macro2::Punct::new(value.as_char(), value.spacing().into())
    }
}

// --- Literal ---

impl From<proc_macro2::Literal> for Literal {
    fn from(value: proc_macro2::Literal) -> Self {
        let span: Span = value.span().into();
        Self::Fallback(crate::token::fallback::Literal {
            repr: value.to_string().into_boxed_str(),
            span,
        })
    }
}

impl From<Literal> for proc_macro2::Literal {
    fn from(value: Literal) -> Self {
        let repr = format!("{}", value);
        repr.parse()
            .unwrap_or_else(|_| proc_macro2::Literal::string(&repr))
    }
}

// --- Group ---

impl From<proc_macro2::Group> for Group {
    fn from(value: proc_macro2::Group) -> Self {
        Self::new(value.delimiter().into(), value.stream().into())
    }
}

impl From<Group> for proc_macro2::Group {
    fn from(value: Group) -> Self {
        let delim: proc_macro2::Delimiter = value.delim().into();
        let stream: proc_macro2::TokenStream = value
            .stream()
            .into_iter()
            .map(proc_macro2::TokenTree::from)
            .collect();
        proc_macro2::Group::new(delim, stream)
    }
}

// --- TokenTree ---

impl From<proc_macro2::TokenTree> for TokenTree {
    fn from(value: proc_macro2::TokenTree) -> Self {
        match value {
            proc_macro2::TokenTree::Ident(v) => Self::Token(Token::Ident(v.into())),
            proc_macro2::TokenTree::Punct(v) => Self::Token(Token::Punct(v.into())),
            proc_macro2::TokenTree::Literal(v) => Self::Token(Token::Literal(v.into())),
            proc_macro2::TokenTree::Group(v) => Self::Group(v.into()),
        }
    }
}

impl From<TokenTree> for proc_macro2::TokenTree {
    fn from(value: TokenTree) -> Self {
        match value {
            TokenTree::Token(Token::Ident(v)) => proc_macro2::TokenTree::Ident(v.into()),
            TokenTree::Token(Token::Punct(v)) => proc_macro2::TokenTree::Punct(v.into()),
            TokenTree::Token(Token::Literal(v)) => proc_macro2::TokenTree::Literal(v.into()),
            TokenTree::Group(v) => proc_macro2::TokenTree::Group(v.into()),
        }
    }
}

// --- TokenStream ---

impl From<proc_macro2::TokenStream> for TokenStream {
    fn from(stream: proc_macro2::TokenStream) -> Self {
        Self::Fallback(stream.into_iter().map(TokenTree::from).collect())
    }
}

impl From<TokenStream> for proc_macro2::TokenStream {
    fn from(stream: TokenStream) -> Self {
        stream
            .into_iter()
            .map(proc_macro2::TokenTree::from)
            .collect()
    }
}
