#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum Stmt {
    Local { value: Box<Local> },
    Item { value: Box<Item> },
    Expr { value: Box<Expr> },
    Semi { value: Box<Expr> },
    Macro { value: StmtMacro },
}
impl crate::ast::Visit for Stmt {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Stmt::Local { value } => {
                let _ = &value;
            }
            Stmt::Item { value } => {
                let _ = &value;
            }
            Stmt::Expr { value } => {
                let _ = &value;
            }
            Stmt::Semi { value } => {
                let _ = &value;
            }
            Stmt::Macro { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for Stmt {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Stmt::Local { value } => Stmt::Local { value },
            Stmt::Item { value } => Stmt::Item { value },
            Stmt::Expr { value } => Stmt::Expr { value },
            Stmt::Semi { value } => Stmt::Semi { value },
            Stmt::Macro { value } => Stmt::Macro { value },
        }
    }
}
mod local;
pub use local::*;
mod local_init;
pub use local_init::*;
mod stmt_macro;
pub use stmt_macro::*;
