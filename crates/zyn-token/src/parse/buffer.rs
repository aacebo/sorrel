use crate::{Stream, Token};

use super::*;

/// An immutable, flat buffer of tokens built from a [`crate::TokenStream`].
///
/// Groups are flattened into the buffer with their children inline, bounded by
/// `TokenTree::Eof` sentinels. This allows [`Cursor`] to navigate the token
/// sequence without unsafe pointer arithmetic.
pub struct TokenBuffer {
    pub(super) entries: Box<[Token]>,
}

impl TokenBuffer {
    /// Build a `TokenBuffer` from anything that implements [`Stream`].
    pub fn new(stream: impl Stream) -> Self {
        let mut entries = stream.stream().flat();
        entries.push(Token::Eof.into()); // root end sentinel
        Self {
            entries: entries.into_boxed_slice(),
        }
    }

    /// Returns a [`Cursor`] positioned at the beginning of the buffer.
    pub fn begin(&self) -> Cursor<'_> {
        let end = self.entries.len() - 1;
        Cursor {
            buffer: self,
            index: 0,
            end,
        }
    }
}

impl std::ops::Index<usize> for TokenBuffer {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}
