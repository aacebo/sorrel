use crate::{Span, Stream, ToStream, Token, Buffer, Reader, Writer};

pub struct ParseStream {
    pub(crate) buffer: Stream,
    pub(crate) index: usize,
    pub(crate) output: Buffer,
}

impl ParseStream {
    pub fn new(buffer: Stream) -> Self {
        Self {
            buffer,
            index: 0,
            output: Buffer::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.index >= self.buffer.len()
    }

    pub fn span(&self) -> Span {
        match self.buffer.get(self.index) {
            Some(Token::Group(g)) => g.span().into(),
            Some(Token::Ident(i)) => i.span().into(),
            Some(Token::Punct(p)) => p.span().into(),
            Some(Token::Literal(l)) => l.span().into(),
            None => Span::call_site(),
        }
    }
}

// impl Reader for ParseStream {
//     fn peek(&mut self) -> Option<&Token> {
//         self.buffer.get(self.index)
//     }

//     fn next(&mut self) -> Option<Token> {
//         let token = self.buffer.get(self.index)?.clone();
//         self.index += 1;
//         Some(token)
//     }

//     fn fork(&self) -> Self {
//         Self {
//             buffer: self.buffer.clone(),
//             index: self.index,
//             output: Buffer::new(),
//         }
//     }

//     fn seek(&mut self, other: &Self) {
//         self.index = other.index;
//     }
// }

// impl TokenWriter for ParseStream {
//     fn write(&mut self, value: impl Stream) -> Result<()> {
//         self.output.extend(value.stream());
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn parse(input: &str) -> ParseStream {
        let stream: Stream = input
            .parse::<proc_macro2::TokenStream>()
            .unwrap()
            .into_iter()
            .map(Token::from)
            .collect();
        ParseStream::new(stream)
    }

    #[test]
    fn empty_stream() {
        let stream = ParseStream::new(Stream::new());
        assert!(stream.is_empty());
    }

    #[test]
    fn simple_idents_and_punct() {
        let mut stream = parse("a + b");

        assert!(matches!(stream.next().unwrap(), Token::Ident(_)));
        assert!(matches!(stream.next().unwrap(), Token::Punct(_)));
        assert!(matches!(stream.next().unwrap(), Token::Ident(_)));
        assert!(stream.is_empty());
    }

    #[test]
    fn peek_does_not_consume() {
        let mut stream = parse("a b");

        assert!(matches!(stream.peek().unwrap(), Token::Ident(_)));
        assert!(matches!(stream.peek().unwrap(), Token::Ident(_)));
        assert!(matches!(stream.next().unwrap(), Token::Ident(_)));
        assert!(!stream.is_empty()); // "b" remains
    }

    #[test]
    fn fork_does_not_advance_original() {
        let mut stream = parse("a b");
        let mut fork = stream.fork();

        assert!(matches!(fork.next().unwrap(), Token::Ident(_))); // "a"
        assert!(matches!(stream.peek().unwrap(), Token::Ident(_))); // still "a"
    }

    #[test]
    fn commit_fork() {
        let mut stream = parse("a b");
        let mut fork = stream.fork();

        fork.next().unwrap(); // advance fork past "a"

        // original still at "a"
        assert!(matches!(stream.peek().unwrap(), Token::Ident(_)));

        // commit fork progress to original
        stream.seek(&fork);
        assert!(matches!(stream.peek().unwrap(), Token::Ident(_))); // now at "b"
    }

    #[test]
    fn write_appends() {
        let mut stream = ParseStream::new(Stream::new());
        let ident = Ident::new("x", proc_macro2::Span::call_site());
        stream.write(Token::Ident(ident)).unwrap();
        assert_eq!(stream.output.len(), 1);
    }

    #[test]
    fn group_token_accessible() {
        let mut stream = parse("(a + b) c");
        let group = stream.next().unwrap();
        assert!(matches!(group, Token::Group(_)));
        if let Token::Group(g) = group {
            let mut inner = ParseStream::new(g.stream());
            assert!(matches!(inner.next().unwrap(), Token::Ident(_))); // "a"
        }
    }
}
