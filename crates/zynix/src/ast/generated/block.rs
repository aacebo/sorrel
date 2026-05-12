use super::*;
#[doc = "A braced block of statements (`{ stmt; stmt; expr }`)."]
#[derive(Debug, Clone)]
pub struct Block {
    pub span: crate::Span,
    pub stmts: Vec<Stmt>,
}
