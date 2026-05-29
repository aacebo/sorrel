#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum Pattern {
    Wild {},
    Rest {},
    Ident { value: PatIdent },
    Path { value: PatPath },
    Tuple { value: PatTuple },
    TupleStruct { value: PatTupleStruct },
    Struct { value: PatStruct },
    Slice { value: PatSlice },
    Reference { value: PatReference },
    Or { value: PatOr },
    Lit { value: PatLit },
    Range { value: PatRange },
    Macro { value: MacroCall },
    Type { value: PatType },
    Group { value: PatGroup },
    Paren { value: PatParen },
}
mod pat_field;
pub use pat_field::*;
mod pat_group;
pub use pat_group::*;
mod pat_ident;
pub use pat_ident::*;
mod pat_lit;
pub use pat_lit::*;
mod pat_or;
pub use pat_or::*;
mod pat_paren;
pub use pat_paren::*;
mod pat_path;
pub use pat_path::*;
mod pat_range;
pub use pat_range::*;
mod pat_reference;
pub use pat_reference::*;
mod pat_slice;
pub use pat_slice::*;
mod pat_struct;
pub use pat_struct::*;
mod pat_tuple;
pub use pat_tuple::*;
mod pat_tuple_struct;
pub use pat_tuple_struct::*;
mod pat_type;
pub use pat_type::*;
