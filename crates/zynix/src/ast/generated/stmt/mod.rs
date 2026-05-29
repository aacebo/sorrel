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
mod local;
pub use local::*;
mod local_init;
pub use local_init::*;
mod stmt_macro;
pub use stmt_macro::*;
