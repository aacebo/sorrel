use super::lex::LexError;
use super::{Spacing, ToTokens};
use crate::parse::{ParseError, ParseStream};
use crate::{Parse, Span, Token, TokenStream, TokenTree};

use super::fallback;

#[derive(Debug, Clone)]
pub enum Punct {
    Compiler(proc_macro::Punct),
    Fallback(fallback::Punct),
}

impl Punct {
    pub fn new(ch: char, spacing: Spacing) -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Punct::new(ch, spacing.into()))
        } else {
            Self::Fallback(fallback::Punct::new(ch, spacing))
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Self::Compiler(v) => v.as_char(),
            Self::Fallback(v) => v.as_char(),
        }
    }

    pub fn spacing(&self) -> Spacing {
        match self {
            Self::Compiler(v) => v.spacing().into(),
            Self::Fallback(v) => v.spacing(),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Compiler(v) => v.span().into(),
            Self::Fallback(v) => v.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Compiler(v) => v.set_span(span.into()),
            Self::Fallback(v) => v.set_span(span),
        }
    }
}

impl From<proc_macro::Punct> for Punct {
    fn from(value: proc_macro::Punct) -> Self {
        Self::Compiler(value)
    }
}

impl From<Punct> for proc_macro::Punct {
    fn from(value: Punct) -> Self {
        match value {
            Punct::Compiler(v) => v,
            Punct::Fallback(v) => {
                let mut p = proc_macro::Punct::new(v.ch, v.spacing.into());
                p.set_span(v.span.into());
                p
            }
        }
    }
}

impl From<fallback::Punct> for Punct {
    fn from(value: fallback::Punct) -> Self {
        Self::Fallback(value)
    }
}

impl From<Punct> for fallback::Punct {
    fn from(value: Punct) -> Self {
        match value {
            Punct::Compiler(v) => fallback::Punct {
                ch: v.as_char(),
                spacing: v.spacing().into(),
                span: v.span().into(),
            },
            Punct::Fallback(v) => v,
        }
    }
}

impl std::fmt::Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler(v) => write!(f, "{}", v),
            Self::Fallback(v) => write!(f, "{}", v),
        }
    }
}

impl ToTokens for Punct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(self.clone().into());
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Punct {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Fallback(v) => v.serialize(s),
            Self::Compiler(_) => self.to_string().serialize(s),
        }
    }
}

impl crate::Parse for Punct {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        match stream.advance() {
            Some(crate::TokenTree::Token(crate::Token::Punct(v))) => Ok(v.clone()),
            _ => Err(crate::token::lex::LexError::new(stream.span())
                .message("expected Punct")
                .into()),
        }
    }
}

fn parse_joint_seq(
    stream: &mut ParseStream,
    chars: &[char],
    display: &'static str,
) -> Result<Span, ParseError> {
    let start = stream.span();
    let need = chars.len();
    let mut first_span: Option<Span> = None;

    for (i, &ch) in chars.iter().enumerate() {
        let is_last = i + 1 == need;

        match stream.advance() {
            Some(TokenTree::Token(Token::Punct(p)))
                if p.as_char() == ch && (is_last || p.spacing() == Spacing::Joint) =>
            {
                if first_span.is_none() {
                    first_span = Some(p.span());
                }
            }
            _ => {
                return Err(LexError::new(start)
                    .message(format!("expected `{}`", display))
                    .into());
            }
        }
    }

    Ok(first_span.unwrap_or(start))
}

macro_rules! define_punct {
    ($($name:ident => $text:literal [ $($ch:literal),+ $(,)? ]),+ $(,)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum Punctuation {
            $($name($name),)*
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for Punctuation {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.as_str().serialize(s)
            }
        }

        impl Punctuation {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$name(v) => v.as_str(),)*
                }
            }

            pub fn span(&self) -> Span {
                match self {
                    $(Self::$name(v) => v.span(),)*
                }
            }

            pub fn set_span(&mut self, span: Span) {
                match self {
                    $(Self::$name(v) => v.set_span(span),)*
                }
            }
        }

        impl ToTokens for Punctuation {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                match self {
                    $(Self::$name(v) => v.to_tokens(tokens),)*
                }
            }
        }

        impl std::fmt::Display for Punctuation {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$name(v) => v.fmt(f),)*
                }
            }
        }

        $(
            #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $name {
                pub span: Span,
            }

            #[cfg(feature = "serde")]
            impl serde::Serialize for $name {
                fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    self.as_str().serialize(s)
                }
            }

            impl $name {
                pub const TEXT: &'static str = $text;

                pub fn new(span: Span) -> Self {
                    Self { span }
                }

                pub fn span(&self) -> Span {
                    self.span
                }

                pub fn set_span(&mut self, span: Span) {
                    self.span = span;
                }

                pub fn as_str(&self) -> &'static str {
                    Self::TEXT
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str($text)
                }
            }

            impl Parse for $name {
                fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
                    let chars: &[char] = &[ $($ch),+ ];
                    let span = parse_joint_seq(stream, chars, $text)?;
                    Ok(Self::new(span))
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    let chars: &[char] = &[ $($ch),+ ];
                    let last = chars.len() - 1;
                    for (i, &ch) in chars.iter().enumerate() {
                        let sp = if i == last { Spacing::Alone } else { Spacing::Joint };
                        let mut p = Punct::new(ch, sp);
                        p.set_span(self.span);
                        tokens.extend_one(p.into());
                    }
                }
            }

            impl From<$name> for Punctuation {
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }
        )+
    };
}

define_punct! {
    And        => "&"   ['&'],
    Or         => "|"   ['|'],
    Not        => "!"   ['!'],
    Tilde      => "~"   ['~'],
    Plus       => "+"   ['+'],
    Minus      => "-"   ['-'],
    Star       => "*"   ['*'],
    Slash      => "/"   ['/'],
    Percent    => "%"   ['%'],
    Caret      => "^"   ['^'],
    Eq         => "="   ['='],
    Lt         => "<"   ['<'],
    Gt         => ">"   ['>'],
    At         => "@"   ['@'],
    Dot        => "."   ['.'],
    Comma      => ","   [','],
    Semi       => ";"   [';'],
    Colon      => ":"   [':'],
    Pound      => "#"   ['#'],
    Dollar     => "$"   ['$'],
    Question   => "?"   ['?'],

    AndAnd     => "&&"  ['&', '&'],
    OrOr       => "||"  ['|', '|'],
    Shl        => "<<"  ['<', '<'],
    Shr        => ">>"  ['>', '>'],
    EqEq       => "=="  ['=', '='],
    Ne         => "!="  ['!', '='],
    Le         => "<="  ['<', '='],
    Ge         => ">="  ['>', '='],
    AndEq      => "&="  ['&', '='],
    OrEq       => "|="  ['|', '='],
    PlusEq     => "+="  ['+', '='],
    MinusEq    => "-="  ['-', '='],
    StarEq     => "*="  ['*', '='],
    SlashEq    => "/="  ['/', '='],
    PercentEq  => "%="  ['%', '='],
    CaretEq    => "^="  ['^', '='],
    FatArrow   => "=>"  ['=', '>'],
    RArrow     => "->"  ['-', '>'],
    LArrow     => "<-"  ['<', '-'],
    PathSep    => "::"  [':', ':'],
    DotDot     => ".."  ['.', '.'],

    ShlEq      => "<<=" ['<', '<', '='],
    ShrEq      => ">>=" ['>', '>', '='],
    DotDotDot  => "..." ['.', '.', '.'],
    DotDotEq   => "..=" ['.', '.', '='],
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TokenStream;
    use crate::token::Underscore;
    use std::str::FromStr;

    #[test]
    fn parse_comma() {
        let ts = TokenStream::from_str(",").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Comma>().is_ok());
    }

    #[test]
    fn parse_eq_eq() {
        let ts = TokenStream::from_str("==").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<EqEq>().is_ok());
    }

    #[test]
    fn alone_spaced_eq_not_eq_eq() {
        let ts = TokenStream::from_str("= =").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<EqEq>().is_err());
    }

    #[test]
    fn alone_spaced_eq_parses_as_eq() {
        let ts = TokenStream::from_str("= =").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Eq>().is_ok());
    }

    #[test]
    fn parse_dot_dot_eq() {
        let ts = TokenStream::from_str("..=").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<DotDotEq>().is_ok());
    }

    #[test]
    fn parse_path_sep() {
        let ts = TokenStream::from_str("::").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<PathSep>().is_ok());
    }

    #[test]
    fn parse_fat_arrow() {
        let ts = TokenStream::from_str("=>").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<FatArrow>().is_ok());
    }

    #[test]
    fn roundtrip_comma() {
        let s = Comma::default().to_token_stream().to_string();
        let ts = TokenStream::from_str(&s).unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Comma>().is_ok());
    }

    #[test]
    fn roundtrip_eq_eq() {
        let s = EqEq::default().to_token_stream().to_string();
        let ts = TokenStream::from_str(&s).unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<EqEq>().is_ok());
    }

    #[test]
    fn underscore_parses() {
        let ts = TokenStream::from_str("_").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Underscore>().is_ok());
    }

    #[test]
    fn display_strings() {
        assert_eq!(format!("{}", Comma::default()), ",");
        assert_eq!(format!("{}", EqEq::default()), "==");
        assert_eq!(format!("{}", DotDotEq::default()), "..=");
        assert_eq!(format!("{}", Underscore::default()), "_");
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;
        use crate::Token;

        #[test]
        fn punct_serializes_as_string() {
            let ts = TokenStream::from_str("+").unwrap();
            let tree = ts.into_iter().next().unwrap();
            let TokenTree::Token(Token::Punct(p)) = tree else {
                panic!("expected punct");
            };
            assert_eq!(serde_json::to_value(&p).unwrap(), serde_json::json!("+"));
        }

        #[test]
        fn named_punct_serializes_as_string() {
            assert_eq!(
                serde_json::to_value(EqEq::default()).unwrap(),
                serde_json::json!("==")
            );
        }

        #[test]
        fn punctuation_serializes_as_string() {
            let p = Punctuation::from(Comma::default());
            assert_eq!(serde_json::to_value(p).unwrap(), serde_json::json!(","));
        }
    }
}
