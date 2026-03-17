use proc_macro2::Span;

use super::*;

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
    pub fn fork(&self) -> Self {
        Self {
            cursor: self.cursor,
        }
    }

    /// Advance this stream to the position of `other` (commit a fork).
    pub fn advance_to(&mut self, other: &Self) {
        self.cursor = other.cursor;
    }
}
