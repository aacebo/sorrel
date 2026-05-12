use super::lex::LexError;
use super::{Group, ToTokens};
use crate::parse::{ParseError, ParseStream};
use crate::{Parse, Span, TokenStream, TokenTree};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Delim {
    #[default]
    None,
    Paren,
    Brace,
    Bracket,
}

impl Delim {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Paren => "paren",
            Self::Brace => "brace",
            Self::Bracket => "bracket",
            Self::None => "none",
        }
    }

    pub fn open(&self) -> char {
        match self {
            Self::None => ' ',
            Self::Brace => '{',
            Self::Bracket => '[',
            Self::Paren => '(',
        }
    }

    pub fn close(&self) -> char {
        match self {
            Self::None => ' ',
            Self::Brace => '}',
            Self::Bracket => ']',
            Self::Paren => ')',
        }
    }

    pub fn from_open(ch: char) -> Option<Self> {
        match ch {
            '(' => Some(Self::Paren),
            '[' => Some(Self::Bracket),
            '{' => Some(Self::Brace),
            _ => None,
        }
    }

    pub fn from_close(ch: char) -> Option<Self> {
        match ch {
            ')' => Some(Self::Paren),
            ']' => Some(Self::Bracket),
            '}' => Some(Self::Brace),
            _ => None,
        }
    }
}

impl From<proc_macro::Delimiter> for Delim {
    fn from(value: proc_macro::Delimiter) -> Self {
        match value {
            proc_macro::Delimiter::Parenthesis => Self::Paren,
            proc_macro::Delimiter::Brace => Self::Brace,
            proc_macro::Delimiter::Bracket => Self::Bracket,
            proc_macro::Delimiter::None => Self::None,
        }
    }
}

impl From<Delim> for proc_macro::Delimiter {
    fn from(value: Delim) -> Self {
        match value {
            Delim::Paren => proc_macro::Delimiter::Parenthesis,
            Delim::Brace => proc_macro::Delimiter::Brace,
            Delim::Bracket => proc_macro::Delimiter::Bracket,
            Delim::None => proc_macro::Delimiter::None,
        }
    }
}

macro_rules! define_delim {
    ($($name:ident => $variant:ident),+ $(,)?) => {
        $(
            #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $name {
                pub span: Span,
            }

            impl $name {
                pub fn new(span: Span) -> Self {
                    Self { span }
                }

                pub fn span(&self) -> Span {
                    self.span
                }

                pub fn set_span(&mut self, span: Span) {
                    self.span = span;
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(Delim::$variant.as_str())
                }
            }

            impl Parse for $name {
                fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
                    let at = stream.span();

                    match stream.curr() {
                        Some(TokenTree::Group(g)) if g.delim() == Delim::$variant => {
                            let span = g.span().open();
                            stream.advance();
                            Ok(Self::new(span))
                        }
                        _ => Err(LexError::new(at)
                            .message(concat!("expected `", stringify!($variant), "` delimiter"))
                            .into()),
                    }
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    let g = Group::new(Delim::$variant, TokenStream::new());
                    tokens.extend_one(g.into());
                }
            }
        )+
    };
}

define_delim! {
    Paren   => Paren,
    Brace   => Brace,
    Bracket => Bracket,
}

#[cfg(test)]
mod delim_token_tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parses_paren_group() {
        let ts = TokenStream::from_str("(a)").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Paren>().is_ok());
        assert!(ps.is_empty());
    }

    #[test]
    fn paren_rejects_bracket() {
        let ts = TokenStream::from_str("[a]").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Paren>().is_err());
    }

    #[test]
    fn bracket_parses_bracket() {
        let ts = TokenStream::from_str("[a]").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Bracket>().is_ok());
    }

    #[test]
    fn brace_parses_brace() {
        let ts = TokenStream::from_str("{a}").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Brace>().is_ok());
    }

    #[test]
    fn to_tokens_emits_group() {
        let stream = Paren::default().to_token_stream();
        assert_eq!(stream.len(), 1);
        let mut ps = stream.parse();
        assert!(ps.parse::<Paren>().is_ok());

        let stream = Bracket::default().to_token_stream();
        let mut ps = stream.parse();
        assert!(ps.parse::<Bracket>().is_ok());

        let stream = Brace::default().to_token_stream();
        let mut ps = stream.parse();
        assert!(ps.parse::<Brace>().is_ok());
    }
}
