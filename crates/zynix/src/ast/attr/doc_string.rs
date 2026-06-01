use super::AttrStyle;

#[doc = "A documentation comment (`///` outer or `//!` inner). Not parsed from the token stream directly — doc comments surface as `#[doc = \"...\"]` attributes."]
#[derive(Debug, Clone)]
pub struct DocString {
    pub span: crate::Span,
    pub value: String,
    pub style: AttrStyle,
}
