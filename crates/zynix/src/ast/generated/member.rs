#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum Member {
    Named { ident: Ident },
    Unnamed { index: u32 },
}
impl crate::ast::Visit for Member {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Member::Named { ident } => {
                let _ = &ident;
            }
            Member::Unnamed { index } => {
                let _ = &index;
            }
        }
    }
}
impl crate::ast::Fold for Member {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Member::Named { ident } => Member::Named { ident },
            Member::Unnamed { index } => Member::Unnamed { index },
        }
    }
}
