#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum AttrStyle {
    Outer,
    Inner,
}
impl crate::ast::Visit for AttrStyle {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            AttrStyle::Outer => {}
            AttrStyle::Inner => {}
        }
    }
}
impl crate::ast::Fold for AttrStyle {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            AttrStyle::Outer => AttrStyle::Outer,
            AttrStyle::Inner => AttrStyle::Inner,
        }
    }
}
