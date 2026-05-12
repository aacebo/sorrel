#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum UseTree {
    Path { value: UsePath },
    Name { value: UseName },
    Rename { value: UseRename },
    Glob {},
    Group { value: UseGroup },
}
impl crate::ast::Visit for UseTree {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            UseTree::Path { value } => {
                let _ = &value;
            }
            UseTree::Name { value } => {
                let _ = &value;
            }
            UseTree::Rename { value } => {
                let _ = &value;
            }
            UseTree::Glob {} => {}
            UseTree::Group { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for UseTree {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            UseTree::Path { value } => UseTree::Path { value },
            UseTree::Name { value } => UseTree::Name { value },
            UseTree::Rename { value } => UseTree::Rename { value },
            UseTree::Glob {} => UseTree::Glob {},
            UseTree::Group { value } => UseTree::Group { value },
        }
    }
}
