#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum ClosureParam {
    Typed { pat: Box<Pattern>, ty: Box<Type> },
    Inferred { pat: Box<Pattern> },
}
impl crate::ast::Visit for ClosureParam {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            ClosureParam::Typed { pat, ty } => {
                let _ = &pat;
                let _ = &ty;
            }
            ClosureParam::Inferred { pat } => {
                let _ = &pat;
            }
        }
    }
}
impl crate::ast::Fold for ClosureParam {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            ClosureParam::Typed { pat, ty } => ClosureParam::Typed { pat, ty },
            ClosureParam::Inferred { pat } => ClosureParam::Inferred { pat },
        }
    }
}
