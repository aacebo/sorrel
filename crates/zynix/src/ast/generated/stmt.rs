use super::*;
#[derive(Debug, Clone)]
pub enum Stmt {
    Local { value: Local },
    Item { value: Item },
    Expr { value: Expr },
    Semi { value: Expr },
    Macro { value: StmtMacro },
}
