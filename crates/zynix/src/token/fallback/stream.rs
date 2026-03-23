use crate::{DelimSpan, Span, ToTokens, TokenTree};

#[derive(Debug, Default, Clone)]
pub struct TokenStream(Vec<TokenTree>);

impl TokenStream {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenTree> {
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

    pub fn extend_one(&mut self, token: TokenTree) {
        self.0.push(token);
    }

    pub fn into_inner(self) -> Vec<TokenTree> {
        self.0
    }
}

impl std::ops::Deref for TokenStream {
    type Target = [TokenTree];

    fn deref(&self) -> &[TokenTree] {
        self.0.as_slice()
    }
}

impl Extend<TokenTree> for TokenStream {
    fn extend<T: IntoIterator<Item = TokenTree>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<T: IntoIterator<Item = TokenTree>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = std::vec::IntoIter<TokenTree>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<proc_macro::TokenStream> for TokenStream {
    fn from(value: proc_macro::TokenStream) -> Self {
        Self(value.into_iter().map(TokenTree::from).collect())
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

impl From<Vec<TokenTree>> for TokenStream {
    fn from(value: Vec<TokenTree>) -> Self {
        Self(value)
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
                tokens.push(crate::Group::Fallback(group).into());
                c = next;
                continue;
            }

            // Leaf tokens: ident, literal, punct
            if let Ok((next, ident)) = super::Ident::scan(c) {
                tokens.push(crate::Ident::Fallback(ident).into());
                c = next;
                continue;
            }

            if let Ok((next, lit)) = super::Literal::scan(c) {
                tokens.push(crate::Literal::Fallback(lit).into());
                c = next;
                continue;
            }

            if let Ok((next, punct)) = super::Punct::scan(c) {
                tokens.push(crate::Punct::Fallback(punct).into());
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

impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut crate::TokenStream) {
        tokens.extend(self.clone());
    }
}
