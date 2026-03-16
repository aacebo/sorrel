use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};

/// A flat entry in the token buffer.
///
/// `None` acts as a scope-end sentinel. Groups are flattened inline:
/// a `Some(Group)` entry is followed by its children, terminated by a `None` sentinel.
///
/// The `jump` field on `Some(Group)` entries holds the index of the matching `None`
/// sentinel, enabling O(1) group skipping in `Cursor::advance`.
#[derive(Clone)]
struct Entry {
    token: Option<TokenTree>,
    /// For `Some(Group)`: index of the matching `None` sentinel.
    /// For everything else: unused (0).
    jump: usize,
}

/// An immutable, flat buffer of tokens built from a [`TokenStream`].
///
/// Groups are flattened into the buffer with their children inline, bounded by
/// `None` sentinels. This allows [`Cursor`] to navigate in O(1) without unsafe
/// pointer arithmetic.
pub struct TokenBuffer {
    entries: Box<[Entry]>,
}

impl TokenBuffer {
    /// Flatten a [`TokenStream`] into a `TokenBuffer`.
    pub fn new(stream: TokenStream) -> Self {
        let mut entries = Vec::new();
        Self::flatten(stream, &mut entries);
        entries.push(Entry {
            token: None,
            jump: 0,
        }); // root end sentinel
        Self {
            entries: entries.into_boxed_slice(),
        }
    }

    /// Returns a [`Cursor`] positioned at the beginning of the buffer.
    pub fn begin(&self) -> Cursor<'_> {
        let scope = self.entries.len() - 1; // index of root end sentinel
        Cursor {
            buffer: self,
            index: 0,
            scope,
        }
    }

    fn flatten(stream: TokenStream, out: &mut Vec<Entry>) {
        for tt in stream {
            match tt {
                TokenTree::Group(g) => {
                    // Push placeholder; we'll back-patch `jump` after recursing.
                    let group_idx = out.len();
                    let stream = g.stream();

                    out.push(Entry {
                        token: Some(TokenTree::Group(g)),
                        jump: 0,
                    });

                    Self::flatten(stream, out);
                    let sentinel_idx = out.len();

                    out.push(Entry {
                        token: None,
                        jump: group_idx,
                    });
                    out[group_idx].jump = sentinel_idx;
                }
                leaf => {
                    out.push(Entry {
                        token: Some(leaf),
                        jump: 0,
                    });
                }
            }
        }
    }
}

/// A cheaply-copyable read position into a [`TokenBuffer`].
///
/// `scope` is the index of the `None` sentinel that bounds this cursor's view.
/// A cursor inside a group uses the group's inner `None` as its scope; the root
/// cursor uses the buffer's trailing `None`.
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    buffer: &'a TokenBuffer,
    index: usize,
    scope: usize,
}

impl<'a> Cursor<'a> {
    /// Returns `true` if there are no more tokens in the current scope.
    pub fn is_empty(self) -> bool {
        self.index == self.scope
    }

    /// The span of the current token, or [`Span::call_site`] if empty.
    pub fn span(self) -> Span {
        match self.buffer.entries[self.index].token.as_ref() {
            Some(TokenTree::Group(g)) => g.span(),
            Some(TokenTree::Ident(i)) => i.span(),
            Some(TokenTree::Punct(p)) => p.span(),
            Some(TokenTree::Literal(l)) => l.span(),
            None => Span::call_site(),
        }
    }

    /// If the current token is an [`Ident`], returns it and the cursor advanced
    /// past it. Otherwise returns `None` without consuming anything.
    pub fn ident(self) -> Option<(Ident, Cursor<'a>)> {
        if self.is_empty() {
            return None;
        }

        match &self.buffer.entries[self.index].token {
            Some(TokenTree::Ident(i)) => Some((i.clone(), self.bump(1))),
            _ => None,
        }
    }

    /// If the current token is a [`Punct`], returns it and the cursor advanced
    /// past it. Otherwise returns `None`.
    pub fn punct(self) -> Option<(Punct, Cursor<'a>)> {
        if self.is_empty() {
            return None;
        }

        match &self.buffer.entries[self.index].token {
            Some(TokenTree::Punct(p)) => Some((p.clone(), self.bump(1))),
            _ => None,
        }
    }

    /// If the current token is a [`Literal`], returns it and the cursor advanced
    /// past it. Otherwise returns `None`.
    pub fn literal(self) -> Option<(Literal, Cursor<'a>)> {
        if self.is_empty() {
            return None;
        }

        match &self.buffer.entries[self.index].token {
            Some(TokenTree::Literal(l)) => Some((l.clone(), self.bump(1))),
            _ => None,
        }
    }

    /// If the current token is a [`Group`] with the given `delimiter`, enters it
    /// and returns `(inner, span, rest)`:
    /// - `inner`: cursor scoped to the group's contents
    /// - `span`: the group's span
    /// - `rest`: cursor positioned after the group
    pub fn group(self, delimiter: Delimiter) -> Option<(Cursor<'a>, Span, Cursor<'a>)> {
        if self.is_empty() {
            return None;
        }

        let entry = &self.buffer.entries[self.index];

        match &entry.token {
            Some(TokenTree::Group(g)) if g.delimiter() == delimiter => {
                let sentinel = entry.jump; // index of the group's None sentinel
                let inner = Cursor {
                    buffer: self.buffer,
                    index: self.index + 1,
                    scope: sentinel,
                };
                let rest = Cursor {
                    buffer: self.buffer,
                    index: sentinel + 1,
                    scope: self.scope,
                };
                Some((inner, g.span(), rest))
            }
            _ => None,
        }
    }

    /// Advance past the current token.
    ///
    /// For `Group` tokens, this jumps directly to the entry after the group's
    /// closing sentinel — O(1), no scan needed.
    pub fn advance(self) -> Cursor<'a> {
        if self.is_empty() {
            return self;
        }

        let entry = &self.buffer.entries[self.index];
        let next = match entry.token {
            Some(TokenTree::Group(_)) => entry.jump + 1, // jump past sentinel
            _ => self.index + 1,
        };

        Cursor {
            buffer: self.buffer,
            index: next,
            scope: self.scope,
        }
    }

    /// Reconstruct a [`TokenStream`] from the current position to the scope boundary.
    pub fn token_stream(self) -> TokenStream {
        let mut cursor = self;
        let mut trees: Vec<TokenTree> = Vec::new();

        while !cursor.is_empty() {
            let entry = &cursor.buffer.entries[cursor.index];

            match &entry.token {
                Some(TokenTree::Group(g)) => {
                    let sentinel = entry.jump;
                    let inner = Cursor {
                        buffer: cursor.buffer,
                        index: cursor.index + 1,
                        scope: sentinel,
                    };

                    let inner_stream = inner.token_stream();
                    let mut rebuilt = Group::new(g.delimiter(), inner_stream);
                    
                    rebuilt.set_span(g.span());
                    trees.push(TokenTree::Group(rebuilt));
                    cursor = Cursor {
                        buffer: cursor.buffer,
                        index: sentinel + 1,
                        scope: cursor.scope,
                    };
                }
                Some(leaf) => {
                    trees.push(leaf.clone());
                    cursor = cursor.bump(1);
                }
                None => break,
            }
        }

        trees.into_iter().collect()
    }

    fn bump(self, n: usize) -> Cursor<'a> {
        Cursor {
            buffer: self.buffer,
            index: self.index + n,
            scope: self.scope,
        }
    }
}

/// A thin wrapper over [`Cursor`] that serves as the entry point for parsing.
///
/// `ParseStream` tracks a mutable parse position. Use [`fork`](ParseStream::fork)
/// for speculative parsing and [`advance_to`](ParseStream::advance_to) to commit.
pub struct ParseStream<'a> {
    pub cursor: Cursor<'a>,
}

impl<'a> ParseStream<'a> {
    pub fn new(cursor: Cursor<'a>) -> Self {
        Self { cursor }
    }

    pub fn is_empty(&self) -> bool {
        self.cursor.is_empty()
    }

    pub fn span(&self) -> Span {
        self.cursor.span()
    }

    /// Fork the stream at the current position. The fork can be advanced
    /// speculatively; commit by calling [`advance_to`](ParseStream::advance_to).
    pub fn fork(&self) -> ParseStream<'a> {
        ParseStream {
            cursor: self.cursor,
        }
    }

    /// Advance this stream to the position of `other` (commit a fork).
    pub fn advance_to(&mut self, other: &ParseStream<'a>) {
        self.cursor = other.cursor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ts(s: &str) -> TokenStream {
        s.parse().unwrap()
    }

    #[test]
    fn empty_stream() {
        let buf = TokenBuffer::new(ts(""));
        assert!(buf.begin().is_empty());
    }

    #[test]
    fn simple_idents_and_punct() {
        let buf = TokenBuffer::new(ts("a + b"));
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
        // advance() past a group must jump to after it, not into it
        let buf = TokenBuffer::new(ts("(a + b) c"));
        let c = buf.begin();

        // Advance past the group in one step
        let after = c.advance();
        let (id, after) = after.ident().unwrap();
        assert_eq!(id.to_string(), "c");
        assert!(after.is_empty());
    }

    #[test]
    fn group_enter() {
        let buf = TokenBuffer::new(ts("(a + b) c"));
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
        let buf = TokenBuffer::new(ts("a b"));
        let mut stream = ParseStream::new(buf.begin());

        let mut fork = stream.fork();
        let (a, rest) = fork.cursor.ident().unwrap();
        assert_eq!(a.to_string(), "a");
        fork.cursor = rest;

        // stream hasn't moved yet
        assert!(!stream.is_empty());
        assert_eq!(stream.cursor.ident().unwrap().0.to_string(), "a");

        // commit
        stream.advance_to(&fork);
        assert_eq!(stream.cursor.ident().unwrap().0.to_string(), "b");
    }

    #[test]
    fn token_stream_reconstruction() {
        let input = "a + b";
        let buf = TokenBuffer::new(ts(input));
        let reconstructed = buf.begin().token_stream().to_string();
        assert_eq!(reconstructed, ts(input).to_string());
    }
}
