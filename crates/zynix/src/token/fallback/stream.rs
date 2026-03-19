use crate::{DelimSpan, Span, Token};

#[derive(Debug, Default, Clone)]
pub struct TokenStream(pub(crate) Vec<Token>);

impl TokenStream {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn inner_mut(&mut self) -> &mut Vec<Token> {
        &mut self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        self.0.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.0.iter()
    }

    pub fn first(&self) -> Span {
        self.0.first().map(|v| v.span()).unwrap_or_default()
    }

    pub fn last(&self) -> Span {
        self.0.last().map(|v| v.span()).unwrap_or_default()
    }

    pub fn span(&self) -> Span {
        self.first().join(self.last())
    }

    pub fn delim(&self) -> DelimSpan {
        DelimSpan::new(self.first(), self.last())
    }

    pub fn extend_one(&mut self, token: Token) {
        self.0.push(token);
    }
}

impl std::ops::Deref for TokenStream {
    type Target = [Token];

    fn deref(&self) -> &[Token] {
        self.0.as_slice()
    }
}

impl Extend<Token> for TokenStream {
    fn extend<T: IntoIterator<Item = Token>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<proc_macro::TokenStream> for TokenStream {
    fn from(value: proc_macro::TokenStream) -> Self {
        Self(value.into_iter().map(Token::from).collect())
    }
}

impl From<TokenStream> for proc_macro::TokenStream {
    fn from(value: TokenStream) -> Self {
        value
            .0
            .into_iter()
            .map(proc_macro::TokenTree::from)
            .collect()
    }
}

impl crate::token::lex::Scan for TokenStream {
    fn scan(
        cursor: crate::token::lex::Cursor<'_>,
    ) -> Result<(crate::token::lex::Cursor<'_>, Self), crate::token::lex::LexError> {
        let mut tokens = Vec::new();
        let mut c = cursor;

        loop {
            c = c.skip_whitespace();
            if c.is_empty() {
                break;
            }

            // Check for closing delimiter — return to caller (Group::scan handles matching)
            if let Some(')' | ']' | '}') = c.first() {
                break;
            }

            // Try group first (opening delimiter)
            if let Ok((next, group)) = super::Group::scan(c) {
                tokens.push(Token::Group(crate::Group::Fallback(group)));
                c = next;
                continue;
            }

            // Leaf tokens: ident, literal, punct
            if let Ok((next, ident)) = super::Ident::scan(c) {
                tokens.push(Token::Ident(crate::Ident::Fallback(ident)));
                c = next;
                continue;
            }

            if let Ok((next, lit)) = super::Literal::scan(c) {
                tokens.push(Token::Literal(crate::Literal::Fallback(lit)));
                c = next;
                continue;
            }

            if let Ok((next, punct)) = super::Punct::scan(c) {
                tokens.push(Token::Punct(crate::Punct::Fallback(punct)));
                c = next;
                continue;
            }

            return Err(c.error().message(format!(
                "unexpected character '{}'",
                c.first().unwrap_or('\0')
            )));
        }

        Ok((c, Self(tokens)))
    }
}

impl std::str::FromStr for TokenStream {
    type Err = crate::token::lex::LexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::source::SourceMap;
        use crate::token::lex::{Cursor, Scan};

        let offset = SourceMap::with_mut(|sm| sm.push(s));
        let cursor = Cursor::new(s, offset.byte_range().start as u32);
        let (rest, stream) = Self::scan(cursor)?;
        let rest = rest.skip_whitespace();

        if !rest.is_empty() {
            return Err(rest.error().message("unexpected trailing input"));
        }

        Ok(stream)
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.0.iter() {
            write!(f, "{}", token)?;
        }

        Ok(())
    }
}
