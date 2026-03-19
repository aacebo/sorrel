use crate::{Span, span::fallback};

/// Zero-copy immutable cursor over source text.
/// Each parse step returns a new advanced cursor.
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    rest: &'a str,
    off: u32,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str, offset: u32) -> Self {
        Self {
            rest: src,
            off: offset,
        }
    }

    pub fn rest(&self) -> &'a str {
        self.rest
    }

    pub fn offset(&self) -> u32 {
        self.off
    }

    pub fn is_empty(&self) -> bool {
        self.rest.is_empty()
    }

    pub fn first(&self) -> Option<char> {
        self.rest.chars().next()
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.rest.starts_with(s)
    }

    /// Advance by `n` bytes, counting characters for the offset.
    pub fn advance(&self, n: usize) -> Self {
        let consumed = &self.rest[..n];
        let chars = consumed.chars().count() as u32;

        Self {
            rest: &self.rest[n..],
            off: self.off + chars,
        }
    }

    /// Advance while predicate holds on chars.
    pub fn skip_while(&self, mut pred: impl FnMut(char) -> bool) -> Self {
        let mut bytes = 0;
        for ch in self.rest.chars() {
            if !pred(ch) {
                break;
            }
            bytes += ch.len_utf8();
        }
        self.advance(bytes)
    }

    /// Create a fallback::Span from this cursor to another.
    pub fn span_to(&self, end: &Cursor<'_>) -> Span {
        fallback::Span::new(self.off, end.off).into()
    }

    pub fn error(&self) -> super::LexError {
        super::LexError::new(fallback::Span::new(self.off, self.off + 1).into())
    }

    pub fn skip_whitespace(mut self) -> Self {
        loop {
            // Whitespace
            let next = self.skip_while(|ch| ch.is_whitespace());

            if next.offset() != self.offset() {
                self = next;
                continue;
            }

            // Line comment
            if self.starts_with("//") {
                self = self.skip_while(|ch| ch != '\n');

                if self.starts_with("\n") {
                    self = self.advance(1);
                }

                continue;
            }

            // Block comment (nested)
            if self.starts_with("/*") {
                match self.skip_comment() {
                    None => break, // unterminated — let the main parser deal with it
                    Some(next) => {
                        self = next;
                        continue;
                    }
                }
            }

            break;
        }

        self
    }

    pub fn skip_comment(&self) -> Option<Self> {
        let mut cur = self.advance(2); // skip /*
        let mut depth = 1u32;

        while !cur.is_empty() {
            if cur.starts_with("/*") {
                depth += 1;
                cur = cur.advance(2);
            } else if cur.starts_with("*/") {
                depth -= 1;
                cur = cur.advance(2);

                if depth == 0 {
                    return Some(cur);
                }
            } else {
                let ch = cur.first().unwrap();
                cur = cur.advance(ch.len_utf8());
            }
        }

        None
    }
}
