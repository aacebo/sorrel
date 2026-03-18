use crate::{AsStream, Buffer, Reader, Span, SpanError, Stream, ToStream, Token, Writer};

pub struct ParseStream<'a> {
    input: &'a Stream,
    index: usize,
    output: Buffer,
}

impl<'a> ParseStream<'a> {
    pub fn new(input: &'a Stream) -> Self {
        Self {
            input,
            index: 0,
            output: Buffer::new(),
        }
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
            output: Buffer::new(),
        }
    }

    pub fn seek(&mut self, other: &Self) {
        self.index = other.index;
    }

    pub fn join(&mut self, other: Self) {
        assert!(self.index <= other.index);
        self.index = other.index;
        self.output.extend(other.output);
    }
}

impl<'a> From<&'a Stream> for ParseStream<'a> {
    fn from(value: &'a Stream) -> Self {
        Self::new(value)
    }
}

impl<'a> Reader for ParseStream<'a> {
    fn remaining(&self) -> usize {
        self.input.len().saturating_sub(self.index)
    }

    fn peek(&self) -> Option<&Token> {
        self.input.get(self.index)
    }

    fn next_n(&mut self, n: usize) -> Option<&[Token]> {
        if self.index + n > self.input.len() {
            return None;
        }

        let start = self.index;
        self.index += n;
        Some(&self.input[start..self.index])
    }
}

impl<'a> Writer for ParseStream<'a> {
    type Error = SpanError;

    fn write(&mut self, tokens: impl IntoIterator<Item = Token>) -> Result<(), Self::Error> {
        self.output.extend(tokens);
        Ok(())
    }
}

impl<'a> AsStream for ParseStream<'a> {
    fn as_stream(&self) -> &Stream {
        self.input
    }
}

impl<'a> ToStream for ParseStream<'a> {
    fn to_stream(self) -> Stream {
        self.output.to_stream()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn parse(input: &str) -> Stream {
        input
            .parse::<proc_macro2::TokenStream>()
            .unwrap()
            .into_iter()
            .map(Token::from)
            .collect()
    }

    #[test]
    fn empty_stream() {
        let stream = Stream::new();
        let ps = ParseStream::new(&stream);
        assert!(ps.is_empty());
    }

    #[test]
    fn simple_idents_and_punct() {
        let stream = parse("a + b");
        let mut ps = ParseStream::new(&stream);

        assert!(matches!(ps.next().unwrap(), Token::Ident(_)));
        assert!(matches!(ps.next().unwrap(), Token::Punct(_)));
        assert!(matches!(ps.next().unwrap(), Token::Ident(_)));
        assert!(ps.is_empty());
    }

    #[test]
    fn peek_does_not_consume() {
        let stream = parse("a b");
        let mut ps = ParseStream::new(&stream);

        assert!(matches!(ps.peek().unwrap(), Token::Ident(_)));
        assert!(matches!(ps.peek().unwrap(), Token::Ident(_)));
        assert!(matches!(ps.next().unwrap(), Token::Ident(_)));
        assert!(!ps.is_empty()); // "b" remains
    }

    #[test]
    fn fork_does_not_advance_original() {
        let stream = parse("a b");
        let ps = ParseStream::new(&stream);
        let mut fork = ps.fork();

        assert!(matches!(fork.next().unwrap(), Token::Ident(_))); // "a"
        assert!(matches!(ps.peek().unwrap(), Token::Ident(_))); // still "a"
    }

    #[test]
    fn commit_fork() {
        let stream = parse("a b");
        let mut ps = ParseStream::new(&stream);
        let mut fork = ps.fork();

        fork.next().unwrap(); // advance fork past "a"

        // original still at "a"
        assert!(matches!(ps.peek().unwrap(), Token::Ident(_)));

        // commit fork progress to original
        ps.seek(&fork);
        assert!(matches!(ps.peek().unwrap(), Token::Ident(_))); // now at "b"
    }

    #[test]
    fn write_appends() {
        let stream = Stream::new();
        let mut ps = ParseStream::new(&stream);
        let ident = Ident::new("x", Span::call_site());
        ps.write(Token::Ident(ident)).unwrap();
        assert_eq!(ps.output.freeze().len(), 1);
    }

    #[test]
    fn group_token_accessible() {
        let stream = parse("(a + b) c");
        let mut ps = ParseStream::new(&stream);
        let group = ps.next().unwrap();
        assert!(matches!(group, Token::Group(_)));

        if let Token::Group(g) = group {
            let tokens = g.as_stream().clone();
            let mut inner = ParseStream::new(&tokens);
            assert!(matches!(inner.next().unwrap(), Token::Ident(_))); // "a"
        }
    }
}
