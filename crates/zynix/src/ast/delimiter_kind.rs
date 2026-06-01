#[doc = "The delimiter kind of a group (`( )`, `[ ]`, `{ }`)."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelimiterKind {
    Paren,
    Bracket,
    Brace,
}
