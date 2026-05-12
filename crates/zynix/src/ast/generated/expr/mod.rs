#[allow(unused)]
use super::*;
#[doc = "A Rust expression. The primary recursive node covering all expression forms."]
#[derive(Debug, Clone)]
pub enum Expr {
    Lit { value: ExprLit },
    Path { value: ExprPath },
    Block { value: ExprBlock },
    Unsafe { value: ExprUnsafe },
    Const { value: ExprConst },
    If { value: ExprIf },
    While { value: ExprWhile },
    ForLoop { value: ExprForLoop },
    Loop { value: ExprLoop },
    Match { value: ExprMatch },
    Closure { value: ExprClosure },
    Async { value: ExprAsync },
    Await { value: ExprAwait },
    Try { value: ExprTry },
    TryBlock { value: ExprTryBlock },
    Yield { value: ExprYield },
    Return { value: ExprReturn },
    Break { value: ExprBreak },
    Continue { value: ExprContinue },
    Call { value: ExprCall },
    MethodCall { value: ExprMethodCall },
    Field { value: ExprField },
    Index { value: ExprIndex },
    Reference { value: ExprReference },
    Unary { value: ExprUnary },
    Binary { value: ExprBinary },
    Assign { value: ExprAssign },
    AssignOp { value: ExprAssignOp },
    Cast { value: ExprCast },
    Type { value: ExprType },
    Let { value: ExprLet },
    Struct { value: ExprStruct },
    Tuple { value: ExprTuple },
    Array { value: ExprArray },
    Repeat { value: ExprRepeat },
    Range { value: ExprRange },
    Macro { value: MacroCall },
    Group { value: ExprGroup },
    Paren { value: ExprParen },
    Infer {},
    Verbatim { tokens: crate::TokenStream },
}
impl crate::ast::Visit for Expr {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Expr::Lit { value } => {
                let _ = &value;
            }
            Expr::Path { value } => {
                let _ = &value;
            }
            Expr::Block { value } => {
                let _ = &value;
            }
            Expr::Unsafe { value } => {
                let _ = &value;
            }
            Expr::Const { value } => {
                let _ = &value;
            }
            Expr::If { value } => {
                let _ = &value;
            }
            Expr::While { value } => {
                let _ = &value;
            }
            Expr::ForLoop { value } => {
                let _ = &value;
            }
            Expr::Loop { value } => {
                let _ = &value;
            }
            Expr::Match { value } => {
                let _ = &value;
            }
            Expr::Closure { value } => {
                let _ = &value;
            }
            Expr::Async { value } => {
                let _ = &value;
            }
            Expr::Await { value } => {
                let _ = &value;
            }
            Expr::Try { value } => {
                let _ = &value;
            }
            Expr::TryBlock { value } => {
                let _ = &value;
            }
            Expr::Yield { value } => {
                let _ = &value;
            }
            Expr::Return { value } => {
                let _ = &value;
            }
            Expr::Break { value } => {
                let _ = &value;
            }
            Expr::Continue { value } => {
                let _ = &value;
            }
            Expr::Call { value } => {
                let _ = &value;
            }
            Expr::MethodCall { value } => {
                let _ = &value;
            }
            Expr::Field { value } => {
                let _ = &value;
            }
            Expr::Index { value } => {
                let _ = &value;
            }
            Expr::Reference { value } => {
                let _ = &value;
            }
            Expr::Unary { value } => {
                let _ = &value;
            }
            Expr::Binary { value } => {
                let _ = &value;
            }
            Expr::Assign { value } => {
                let _ = &value;
            }
            Expr::AssignOp { value } => {
                let _ = &value;
            }
            Expr::Cast { value } => {
                let _ = &value;
            }
            Expr::Type { value } => {
                let _ = &value;
            }
            Expr::Let { value } => {
                let _ = &value;
            }
            Expr::Struct { value } => {
                let _ = &value;
            }
            Expr::Tuple { value } => {
                let _ = &value;
            }
            Expr::Array { value } => {
                let _ = &value;
            }
            Expr::Repeat { value } => {
                let _ = &value;
            }
            Expr::Range { value } => {
                let _ = &value;
            }
            Expr::Macro { value } => {
                let _ = &value;
            }
            Expr::Group { value } => {
                let _ = &value;
            }
            Expr::Paren { value } => {
                let _ = &value;
            }
            Expr::Infer {} => {}
            Expr::Verbatim { tokens } => {
                let _ = &tokens;
            }
        }
    }
}
impl crate::ast::Fold for Expr {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Expr::Lit { value } => Expr::Lit { value },
            Expr::Path { value } => Expr::Path { value },
            Expr::Block { value } => Expr::Block { value },
            Expr::Unsafe { value } => Expr::Unsafe { value },
            Expr::Const { value } => Expr::Const { value },
            Expr::If { value } => Expr::If { value },
            Expr::While { value } => Expr::While { value },
            Expr::ForLoop { value } => Expr::ForLoop { value },
            Expr::Loop { value } => Expr::Loop { value },
            Expr::Match { value } => Expr::Match { value },
            Expr::Closure { value } => Expr::Closure { value },
            Expr::Async { value } => Expr::Async { value },
            Expr::Await { value } => Expr::Await { value },
            Expr::Try { value } => Expr::Try { value },
            Expr::TryBlock { value } => Expr::TryBlock { value },
            Expr::Yield { value } => Expr::Yield { value },
            Expr::Return { value } => Expr::Return { value },
            Expr::Break { value } => Expr::Break { value },
            Expr::Continue { value } => Expr::Continue { value },
            Expr::Call { value } => Expr::Call { value },
            Expr::MethodCall { value } => Expr::MethodCall { value },
            Expr::Field { value } => Expr::Field { value },
            Expr::Index { value } => Expr::Index { value },
            Expr::Reference { value } => Expr::Reference { value },
            Expr::Unary { value } => Expr::Unary { value },
            Expr::Binary { value } => Expr::Binary { value },
            Expr::Assign { value } => Expr::Assign { value },
            Expr::AssignOp { value } => Expr::AssignOp { value },
            Expr::Cast { value } => Expr::Cast { value },
            Expr::Type { value } => Expr::Type { value },
            Expr::Let { value } => Expr::Let { value },
            Expr::Struct { value } => Expr::Struct { value },
            Expr::Tuple { value } => Expr::Tuple { value },
            Expr::Array { value } => Expr::Array { value },
            Expr::Repeat { value } => Expr::Repeat { value },
            Expr::Range { value } => Expr::Range { value },
            Expr::Macro { value } => Expr::Macro { value },
            Expr::Group { value } => Expr::Group { value },
            Expr::Paren { value } => Expr::Paren { value },
            Expr::Infer {} => Expr::Infer {},
            Expr::Verbatim { tokens } => Expr::Verbatim { tokens },
        }
    }
}
mod expr_array;
pub use expr_array::*;
mod expr_assign;
pub use expr_assign::*;
mod expr_assign_op;
pub use expr_assign_op::*;
mod expr_async;
pub use expr_async::*;
mod expr_await;
pub use expr_await::*;
mod expr_binary;
pub use expr_binary::*;
mod expr_block;
pub use expr_block::*;
mod expr_break;
pub use expr_break::*;
mod expr_call;
pub use expr_call::*;
mod expr_cast;
pub use expr_cast::*;
mod expr_closure;
pub use expr_closure::*;
mod expr_const;
pub use expr_const::*;
mod expr_continue;
pub use expr_continue::*;
mod expr_field;
pub use expr_field::*;
mod expr_for_loop;
pub use expr_for_loop::*;
mod expr_group;
pub use expr_group::*;
mod expr_if;
pub use expr_if::*;
mod expr_index;
pub use expr_index::*;
mod expr_let;
pub use expr_let::*;
mod expr_lit;
pub use expr_lit::*;
mod expr_loop;
pub use expr_loop::*;
mod expr_match;
pub use expr_match::*;
mod expr_method_call;
pub use expr_method_call::*;
mod expr_paren;
pub use expr_paren::*;
mod expr_path;
pub use expr_path::*;
mod expr_range;
pub use expr_range::*;
mod expr_reference;
pub use expr_reference::*;
mod expr_repeat;
pub use expr_repeat::*;
mod expr_return;
pub use expr_return::*;
mod expr_struct;
pub use expr_struct::*;
mod expr_try;
pub use expr_try::*;
mod expr_try_block;
pub use expr_try_block::*;
mod expr_tuple;
pub use expr_tuple::*;
mod expr_type;
pub use expr_type::*;
mod expr_unary;
pub use expr_unary::*;
mod expr_unsafe;
pub use expr_unsafe::*;
mod expr_while;
pub use expr_while::*;
mod expr_yield;
pub use expr_yield::*;
