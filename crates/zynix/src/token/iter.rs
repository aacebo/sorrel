use crate::TokenTree;

#[derive(Clone)]
pub enum IntoIter {
    Compiler(proc_macro::token_stream::IntoIter),
    Fallback(std::vec::IntoIter<TokenTree>),
}

impl From<std::vec::IntoIter<TokenTree>> for IntoIter {
    fn from(value: std::vec::IntoIter<TokenTree>) -> Self {
        Self::Fallback(value)
    }
}

impl From<proc_macro::token_stream::IntoIter> for IntoIter {
    fn from(value: proc_macro::token_stream::IntoIter) -> Self {
        Self::Compiler(value)
    }
}

impl From<Vec<TokenTree>> for IntoIter {
    fn from(value: Vec<TokenTree>) -> Self {
        Self::Fallback(value.into_iter())
    }
}

impl Iterator for IntoIter {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Compiler(v) => v.next().map(|t| t.into()),
            Self::Fallback(v) => v.next(),
        }
    }
}
