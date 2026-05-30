use super::lex::LexError;
use super::{Ident, ToTokens};
use crate::parse::{ParseError, ParseStream};
use crate::{Parse, Span, Token, TokenStream, TokenTree};

macro_rules! define_keyword {
    ($($name:ident => $text:literal),+ $(,)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum Keyword {
            $($name($name),)*
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for Keyword {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.as_str().serialize(s)
            }
        }

        impl Keyword {
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

        impl ToTokens for Keyword {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                match self {
                    $(Self::$name(v) => v.to_tokens(tokens),)*
                }
            }
        }

        impl std::fmt::Display for Keyword {
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
                    let at = stream.span();

                    match stream.advance() {
                        Some(TokenTree::Token(Token::Ident(id))) if id.name().as_ref() == $text => {
                            Ok(Self::new(id.span()))
                        }
                        _ => Err(LexError::new(at)
                            .message(concat!("expected `", $text, "`"))
                            .into()),
                    }
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    tokens.extend_one(Ident::new($text, self.span).into());
                }
            }

            impl From<$name> for Keyword {
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }
        )+
    };
}

define_keyword! {
    As         => "as",
    Async      => "async",
    Auto       => "auto",
    Await      => "await",
    Become     => "become",
    Box        => "box",
    Break      => "break",
    Const      => "const",
    Continue   => "continue",
    Crate      => "crate",
    Default    => "default",
    Do         => "do",
    Dyn        => "dyn",
    Else       => "else",
    Enum       => "enum",
    Extern     => "extern",
    Final      => "final",
    Fn         => "fn",
    For        => "for",
    If         => "if",
    Impl       => "impl",
    In         => "in",
    Let        => "let",
    Loop       => "loop",
    Macro      => "macro",
    MacroRules => "macro_rules",
    Match      => "match",
    Mod        => "mod",
    Move       => "move",
    Mut        => "mut",
    Override   => "override",
    Priv       => "priv",
    Pub        => "pub",
    Raw        => "raw",
    Ref        => "ref",
    Return     => "return",
    SelfType   => "Self",
    SelfValue  => "self",
    Static     => "static",
    Struct     => "struct",
    Super      => "super",
    Trait      => "trait",
    Try        => "try",
    Type       => "type",
    Typeof     => "typeof",
    Union      => "union",
    Unsafe     => "unsafe",
    Unsized    => "unsized",
    Use        => "use",
    Virtual    => "virtual",
    Where      => "where",
    While      => "while",
    Yield      => "yield",
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::ToTokenStream;
    use std::str::FromStr;

    #[test]
    fn parses_fn() {
        let ts = TokenStream::from_str("fn").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Fn>().is_ok());
    }

    #[test]
    fn rejects_non_match() {
        let ts = TokenStream::from_str("foo").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Fn>().is_err());
    }

    #[test]
    fn display_writes_text() {
        assert_eq!(format!("{}", Fn::default()), "fn");
        assert_eq!(format!("{}", SelfType::default()), "Self");
        assert_eq!(format!("{}", SelfValue::default()), "self");
        assert_eq!(format!("{}", MacroRules::default()), "macro_rules");
    }

    #[test]
    fn roundtrip_fn() {
        let kw = Fn::default();
        let s = kw.to_token_stream().to_string();
        let ts = TokenStream::from_str(&s).unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Fn>().is_ok());
    }

    #[test]
    fn self_type_vs_self_value() {
        let ts_upper = TokenStream::from_str("Self").unwrap();
        let mut ps = ts_upper.parse();
        assert!(ps.parse::<SelfType>().is_ok());

        let ts_lower = TokenStream::from_str("self").unwrap();
        let mut ps = ts_lower.parse();
        assert!(ps.parse::<SelfValue>().is_ok());

        let ts_lower2 = TokenStream::from_str("self").unwrap();
        let mut ps = ts_lower2.parse();
        assert!(ps.parse::<SelfType>().is_err());
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn keyword_struct_serializes_as_string() {
            assert_eq!(
                serde_json::to_value(Fn::default()).unwrap(),
                serde_json::json!("fn")
            );
        }

        #[test]
        fn keyword_enum_serializes_as_string() {
            let kw = Keyword::from(Fn::default());
            assert_eq!(serde_json::to_value(kw).unwrap(), serde_json::json!("fn"));
        }
    }
}
