#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum Meta {
    Path { path: Path },
    List { value: MetaList },
    NameValue { value: MetaNameValue },
}
impl crate::ast::Visit for Meta {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Meta::Path { path } => {
                let _ = &path;
            }
            Meta::List { value } => {
                let _ = &value;
            }
            Meta::NameValue { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for Meta {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Meta::Path { path } => Meta::Path { path },
            Meta::List { value } => Meta::List { value },
            Meta::NameValue { value } => Meta::NameValue { value },
        }
    }
}
