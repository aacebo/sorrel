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
