use crate::{Parse, ParseError, Peek, Span, TokenStream, TokenTree};

pub struct ParseStream<'a> {
    input: &'a TokenStream,
    index: usize,
}

impl<'a> ParseStream<'a> {
    pub fn new(input: &'a TokenStream) -> Self {
        Self { input, index: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.index >= self.input.len()
    }

    pub fn span(&self) -> Span {
        self.input
            .get(self.index)
            .map(|t| t.span())
            .unwrap_or_default()
    }

    pub fn fork(&self) -> Self {
        Self {
            input: self.input,
            index: self.index,
        }
    }

    pub fn seek(&mut self, other: &Self) {
        self.index = other.index;
    }
}

impl<'a> ParseStream<'a> {
    pub fn remaining(&self) -> usize {
        self.input.len().saturating_sub(self.index)
    }

    pub fn curr(&self) -> Option<&TokenTree> {
        self.input.get(self.index)
    }

    pub fn prev(&self) -> Option<&TokenTree> {
        self.input.get(self.index - 1)
    }

    pub fn peek<T: Peek>(&mut self) -> Option<T> {
        let index = self.index;
        let res = T::peek(self);
        self.index = index;
        res
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T, ParseError> {
        T::parse(self)
    }

    pub fn advance_by(&mut self, n: usize) -> Option<&[TokenTree]> {
        if self.index + n > self.input.len() {
            return None;
        }

        let start = self.index;
        self.index += n;
        Some(&self.input[start..self.index])
    }

    /// move the iterator forward and return the token.
    pub fn advance(&mut self) -> Option<&TokenTree> {
        self.advance_by(1)?.first()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn empty_stream() {
        let stream = TokenStream::new();
        let ps = stream.parse();
        assert!(ps.is_empty());
    }

    #[test]
    fn simple_idents_and_punct() {
        let stream = "a + b".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();

        assert!(matches!(
            ps.advance().unwrap(),
            TokenTree::Token(Token::Ident(_))
        ));
        assert!(matches!(
            ps.advance().unwrap(),
            TokenTree::Token(Token::Punct(_))
        ));
        assert!(matches!(
            ps.advance().unwrap(),
            TokenTree::Token(Token::Ident(_))
        ));
        assert!(ps.is_empty());
    }

    #[test]
    fn peek_does_not_consume() {
        let stream = "a b".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();

        assert!(matches!(ps.peek::<Ident>(), Some(_),));
        assert!(matches!(ps.peek::<Ident>(), Some(_),));
        assert!(matches!(ps.parse::<Ident>(), Ok(_)));
        assert!(!ps.is_empty()); // "b" remains
    }

    #[test]
    fn fork_does_not_advance_original() {
        let stream = "a b".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();
        let mut fork = ps.fork();

        assert!(matches!(fork.parse::<Ident>(), Ok(_),)); // "a"
        assert!(matches!(ps.peek::<Ident>(), Some(_),)); // still "a"
    }

    #[test]
    fn commit_fork() {
        let stream = "a b".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();
        let mut fork = ps.fork();

        fork.advance().unwrap(); // advance fork past "a"

        // original still at "a"
        assert!(matches!(ps.parse::<Ident>(), Ok(_),));

        // commit fork progress to original
        ps.seek(&fork);
        assert!(matches!(ps.peek::<Ident>(), Some(_),)); // now at "b"
    }

    #[test]
    fn group_token_accessible() {
        let stream = "(a + b) c".parse::<TokenStream>().unwrap();
        let mut ps = stream.parse();
        let group = ps.advance().unwrap();
        assert!(matches!(group, TokenTree::Group(_)));

        if let TokenTree::Group(g) = group {
            let tokens = g.stream();
            let mut inner = tokens.parse();
            debug_assert!(matches!(
                inner.advance().unwrap(),
                TokenTree::Token(Token::Ident(_))
            )); // "a"
        }
    }
}
