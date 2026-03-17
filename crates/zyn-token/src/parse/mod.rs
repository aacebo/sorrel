mod buffer;
mod cursor;
mod stream;

pub use buffer::*;
pub use cursor::*;
pub use stream::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn empty_stream() {
        let buf = TokenBuffer::new("".parse::<proc_macro2::TokenStream>().unwrap());
        assert!(buf.begin().is_empty());
    }

    #[test]
    fn simple_idents_and_punct() {
        let buf = TokenBuffer::new("a + b".parse::<proc_macro2::TokenStream>().unwrap());
        let c = buf.begin();

        let (a, c) = c.ident().unwrap();
        assert_eq!(a.to_string(), "a");

        let (plus, c) = c.punct().unwrap();
        assert_eq!(plus.as_char(), '+');

        let (b, c) = c.ident().unwrap();
        assert_eq!(b.to_string(), "b");

        assert!(c.is_empty());
    }

    #[test]
    fn group_advance_is_o1() {
        let buf = TokenBuffer::new("(a + b) c".parse::<proc_macro2::TokenStream>().unwrap());
        let c = buf.begin();

        let after = c.advance();
        let (id, after) = after.ident().unwrap();
        assert_eq!(id.to_string(), "c");
        assert!(after.is_empty());
    }

    #[test]
    fn group_enter() {
        let buf = TokenBuffer::new("(a + b) c".parse::<proc_macro2::TokenStream>().unwrap());
        let c = buf.begin();

        let (inner, _span, rest) = c.group(Delimiter::Parenthesis).unwrap();

        let (a, inner) = inner.ident().unwrap();
        assert_eq!(a.to_string(), "a");
        let (_plus, inner) = inner.punct().unwrap();
        let (b, inner) = inner.ident().unwrap();
        assert_eq!(b.to_string(), "b");
        assert!(inner.is_empty());

        let (c_id, rest) = rest.ident().unwrap();
        assert_eq!(c_id.to_string(), "c");
        assert!(rest.is_empty());
    }

    #[test]
    fn fork_and_advance_to() {
        let buf = TokenBuffer::new("a b".parse::<proc_macro2::TokenStream>().unwrap());
        let mut stream = ParseStream::new(buf.begin());

        let mut fork = stream.fork();
        let (a, rest) = fork.cursor.ident().unwrap();
        assert_eq!(a.to_string(), "a");
        fork.cursor = rest;

        assert!(!stream.is_empty());
        assert_eq!(stream.cursor.ident().unwrap().0.to_string(), "a");

        stream.advance_to(&fork);
        assert_eq!(stream.cursor.ident().unwrap().0.to_string(), "b");
    }

    #[test]
    fn token_stream_reconstruction() {
        let input = "a + b".parse::<proc_macro2::TokenStream>().unwrap();
        let buf = TokenBuffer::new(input.clone());
        let reconstructed = buf.begin().token_stream();
        assert_eq!(reconstructed.to_string(), input.to_string());
    }
}
