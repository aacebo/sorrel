#[doc = "Whether an attribute is outer (`#[...]`) or inner (`#![...]`)."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrStyle {
    Outer,
    Inner,
}
