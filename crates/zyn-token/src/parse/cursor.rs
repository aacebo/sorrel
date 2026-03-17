use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Span};

use super::*;
use crate::{Token, Token};

/// A cheaply-copyable read position into a [`TokenBuffer`].
///
/// `end` is the index of the `Token::Eof` sentinel that bounds this cursor's view.
/// A cursor inside a group uses the group's inner `Eof` as its end; the root
/// cursor uses the buffer's trailing `Eof`.
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    pub(super) buffer: &'a TokenBuffer,
    pub(super) index: usize,
    pub(super) end: usize,
}

impl<'a> Cursor<'a> {
    /// Returns `true` if there are no more tokens in the current scope.
    pub fn is_empty(self) -> bool {
        self.index == self.end
    }

    /// The span of the current token, or [`Span::call_site`] if empty.
    pub fn span(self) -> Span {
        match &self.buffer.entries[self.index] {
            Token::Group(g) => g.span(),
            Token::Ident(i) => i.span(),
            Token::Punct(p) => p.span(),
            Token::Literal(l) => l.span(),
            Token::Eof => Span::call_site(),
        }
    }

    /// If the current token is an [`Ident`], returns it and the cursor advanced
    /// past it. Otherwise returns `None` without consuming anything.
    pub fn ident(self) -> Option<(Ident, Self)> {
        match &self.buffer.entries[self.index] {
            Token::Ident(i) => Some((i.clone(), self.bump(1))),
            _ => None,
        }
    }

    /// If the current token is a [`Punct`], returns it and the cursor advanced
    /// past it. Otherwise returns `None`.
    pub fn punct(self) -> Option<(Punct, Self)> {
        match &self.buffer.entries[self.index] {
            Token::Punct(p) => Some((p.clone(), self.bump(1))),
            _ => None,
        }
    }

    /// If the current token is a [`Literal`], returns it and the cursor advanced
    /// past it. Otherwise returns `None`.
    pub fn literal(self) -> Option<(Literal, Self)> {
        match &self.buffer.entries[self.index] {
            Token::Literal(l) => Some((l.clone(), self.bump(1))),
            _ => None,
        }
    }

    /// If the current token is a [`Group`] with the given `delimiter`, enters it
    /// and returns `(inner, span, rest)`:
    /// - `inner`: cursor scoped to the group's contents
    /// - `span`: the group's span
    /// - `rest`: cursor positioned after the group
    pub fn group(self, delimiter: Delimiter) -> Option<(Self, Span, Self)> {
        match &self.buffer.entries[self.index] {
            Token::Group(g) if g.delimiter() == delimiter => {
                let eof = self.find_eof(self.index + 1);
                let inner = Self {
                    buffer: self.buffer,
                    index: self.index + 1,
                    end: eof,
                };

                let rest = Self {
                    buffer: self.buffer,
                    index: eof + 1,
                    end: self.end,
                };

                Some((inner, g.span(), rest))
            }
            _ => None,
        }
    }

    /// Advance past the current token.
    ///
    /// For `Group` tokens, this scans forward to the matching `Eof` sentinel and
    /// positions the cursor after it.
    pub fn advance(self) -> Self {
        if self.is_empty() {
            return self;
        }

        let next = match &self.buffer.entries[self.index] {
            Token::Group(_) => self.find_eof(self.index + 1) + 1,
            _ => self.index + 1,
        };

        Self {
            buffer: self.buffer,
            index: next,
            end: self.end,
        }
    }

    /// Reconstruct a [`proc_macro2::TokenStream`] from the current position to the end boundary.
    pub fn to_stream(self) -> crate::TokenStream {
        let mut cursor = self;
        let mut trees: Vec<Token> = Vec::new();

        while !cursor.is_empty() {
            match &cursor.buffer.entries[cursor.index] {
                Token::Group(g) => {
                    let eof = cursor.find_eof(cursor.index + 1);
                    let inner = Cursor {
                        buffer: cursor.buffer,
                        index: cursor.index + 1,
                        end: eof,
                    };

                    let stream = inner.to_stream();
                    let mut group = Group::new(g.delimiter(), stream.into());

                    group.set_span(g.span());
                    trees.push(group.into());
                    cursor = Cursor {
                        buffer: cursor.buffer,
                        index: eof + 1,
                        end: cursor.end,
                    };
                }
                Token::Ident(i) => {
                    trees.push(Token::Ident(i.clone()));
                    cursor = cursor.bump(1);
                }
                Token::Punct(p) => {
                    trees.push(Token::Punct(p.clone()));
                    cursor = cursor.bump(1);
                }
                Token::Literal(l) => {
                    trees.push(Token::Literal(l.clone()));
                    cursor = cursor.bump(1);
                }
                Token::Eof => break,
            }
        }

        trees.into_iter().collect()
    }

    /// Scan forward from `from` to find the index of the matching `Eof` sentinel,
    /// accounting for nested groups.
    fn find_eof(self, from: usize) -> usize {
        let mut depth = 0usize;
        let mut i = from;
        while i <= self.buffer.entries.len() - 1 {
            match &self.buffer.entries[i] {
                Token::Group(_) => depth += 1,
                Token::Eof => {
                    if depth == 0 {
                        return i;
                    }
                    depth -= 1;
                }
                _ => {}
            }
            i += 1;
        }
        self.buffer.entries.len() - 1
    }

    fn bump(self, n: usize) -> Self {
        Self {
            buffer: self.buffer,
            index: self.index + n,
            end: self.end,
        }
    }
}
