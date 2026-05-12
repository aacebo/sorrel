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
impl crate::ast::Visit for Pattern {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Pattern::Wild {} => {}
            Pattern::Rest {} => {}
            Pattern::Ident { value } => {
                let _ = &value;
            }
            Pattern::Path { value } => {
                let _ = &value;
            }
            Pattern::Tuple { value } => {
                let _ = &value;
            }
            Pattern::TupleStruct { value } => {
                let _ = &value;
            }
            Pattern::Struct { value } => {
                let _ = &value;
            }
            Pattern::Slice { value } => {
                let _ = &value;
            }
            Pattern::Reference { value } => {
                let _ = &value;
            }
            Pattern::Or { value } => {
                let _ = &value;
            }
            Pattern::Lit { value } => {
                let _ = &value;
            }
            Pattern::Range { value } => {
                let _ = &value;
            }
            Pattern::Macro { value } => {
                let _ = &value;
            }
            Pattern::Type { value } => {
                let _ = &value;
            }
            Pattern::Group { value } => {
                let _ = &value;
            }
            Pattern::Paren { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for Pattern {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Pattern::Wild {} => Pattern::Wild {},
            Pattern::Rest {} => Pattern::Rest {},
            Pattern::Ident { value } => Pattern::Ident { value },
            Pattern::Path { value } => Pattern::Path { value },
            Pattern::Tuple { value } => Pattern::Tuple { value },
            Pattern::TupleStruct { value } => Pattern::TupleStruct { value },
            Pattern::Struct { value } => Pattern::Struct { value },
            Pattern::Slice { value } => Pattern::Slice { value },
            Pattern::Reference { value } => Pattern::Reference { value },
            Pattern::Or { value } => Pattern::Or { value },
            Pattern::Lit { value } => Pattern::Lit { value },
            Pattern::Range { value } => Pattern::Range { value },
            Pattern::Macro { value } => Pattern::Macro { value },
            Pattern::Type { value } => Pattern::Type { value },
            Pattern::Group { value } => Pattern::Group { value },
            Pattern::Paren { value } => Pattern::Paren { value },
        }
    }
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
