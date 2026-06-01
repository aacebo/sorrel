use crate::ast::{DelimiterKind, Expr, Path};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Eq;
use crate::token::{Delim, Group, ToTokens, TokenTree};
use crate::{Parse, Span, TokenStream};

#[doc = "A structured attribute meta item (`name`, `name(...)`, `name = expr`)."]
#[derive(Debug, Clone)]
pub enum Meta {
    Path(Path),
    List(MetaList),
    NameValue(MetaNameValue),
}

#[doc = "A list-style meta item (`name(tokens)`)."]
#[derive(Debug, Clone)]
pub struct MetaList {
    pub span: Span,
    pub path: Path,
    pub delimiter: DelimiterKind,
    pub tokens: TokenStream,
}

#[doc = "A name-value meta item (`name = expr`)."]
#[derive(Debug, Clone)]
pub struct MetaNameValue {
    pub span: Span,
    pub path: Path,
    pub value: Expr,
}

fn group_delim(tt: Option<&TokenTree>) -> Option<DelimiterKind> {
    match tt {
        Some(TokenTree::Group(g)) => match g.delim() {
            Delim::Paren => Some(DelimiterKind::Paren),
            Delim::Bracket => Some(DelimiterKind::Bracket),
            Delim::Brace => Some(DelimiterKind::Brace),
            Delim::None => None,
        },
        _ => None,
    }
}

impl Parse for Meta {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let path = stream.parse::<Path>()?;

        if let Some(delimiter) = group_delim(stream.curr()) {
            let delim = match delimiter {
                DelimiterKind::Paren => Delim::Paren,
                DelimiterKind::Bracket => Delim::Bracket,
                DelimiterKind::Brace => Delim::Brace,
            };
            let tokens = stream.parse_group(delim)?;
            return Ok(Meta::List(MetaList {
                span: Span::default(),
                path,
                delimiter,
                tokens,
            }));
        }

        if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            let value = stream.parse::<Expr>()?;
            return Ok(Meta::NameValue(MetaNameValue {
                span: Span::default(),
                path,
                value,
            }));
        }

        Ok(Meta::Path(path))
    }
}

impl ToTokens for Meta {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Meta::Path(p) => p.to_tokens(t),
            Meta::List(l) => l.to_tokens(t),
            Meta::NameValue(nv) => nv.to_tokens(t),
        }
    }
}

impl ToTokens for MetaList {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        let delim = match self.delimiter {
            DelimiterKind::Paren => Delim::Paren,
            DelimiterKind::Bracket => Delim::Bracket,
            DelimiterKind::Brace => Delim::Brace,
        };
        t.extend_one(TokenTree::Group(Group::new(delim, self.tokens.clone())));
    }
}

impl ToTokens for MetaNameValue {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        Eq::default().to_tokens(t);
        self.value.to_tokens(t);
    }
}
