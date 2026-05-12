use super::*;
#[doc = "A Rust attribute (`#[...]` or `#![...]`) applied to an item, expression, or statement."]
#[derive(Debug, Clone)]
pub struct Attribute {
    pub span: crate::Span,
    pub style: AttrStyle,
    pub path: Path,
    pub args: AttrArgs,
}
