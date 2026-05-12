#[allow(unused)]
use super::*;
#[doc = "A Rust type expression. Covers all positions where a type can appear in source code."]
#[derive(Debug, Clone)]
pub enum Type {
    Never {},
    Infer {},
    Path { value: TypePath },
    Tuple { value: TypeTuple },
    Array { value: TypeArray },
    Slice { value: TypeSlice },
    Reference { value: TypeReference },
    Pointer { value: TypePointer },
    BareFn { value: TypeBareFn },
    ImplTrait { value: TypeImplTrait },
    TraitObject { value: TypeTraitObject },
    Paren { value: TypeParen },
    Group { value: TypeGroup },
    Macro { value: MacroCall },
}
impl crate::ast::Visit for Type {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Type::Never {} => {}
            Type::Infer {} => {}
            Type::Path { value } => {
                let _ = &value;
            }
            Type::Tuple { value } => {
                let _ = &value;
            }
            Type::Array { value } => {
                let _ = &value;
            }
            Type::Slice { value } => {
                let _ = &value;
            }
            Type::Reference { value } => {
                let _ = &value;
            }
            Type::Pointer { value } => {
                let _ = &value;
            }
            Type::BareFn { value } => {
                let _ = &value;
            }
            Type::ImplTrait { value } => {
                let _ = &value;
            }
            Type::TraitObject { value } => {
                let _ = &value;
            }
            Type::Paren { value } => {
                let _ = &value;
            }
            Type::Group { value } => {
                let _ = &value;
            }
            Type::Macro { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for Type {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Type::Never {} => Type::Never {},
            Type::Infer {} => Type::Infer {},
            Type::Path { value } => Type::Path { value },
            Type::Tuple { value } => Type::Tuple { value },
            Type::Array { value } => Type::Array { value },
            Type::Slice { value } => Type::Slice { value },
            Type::Reference { value } => Type::Reference { value },
            Type::Pointer { value } => Type::Pointer { value },
            Type::BareFn { value } => Type::BareFn { value },
            Type::ImplTrait { value } => Type::ImplTrait { value },
            Type::TraitObject { value } => Type::TraitObject { value },
            Type::Paren { value } => Type::Paren { value },
            Type::Group { value } => Type::Group { value },
            Type::Macro { value } => Type::Macro { value },
        }
    }
}
mod type_array;
pub use type_array::*;
mod type_bare_fn;
pub use type_bare_fn::*;
mod type_group;
pub use type_group::*;
mod type_impl_trait;
pub use type_impl_trait::*;
mod type_param;
pub use type_param::*;
mod type_paren;
pub use type_paren::*;
mod type_path;
pub use type_path::*;
mod type_pointer;
pub use type_pointer::*;
mod type_predicate;
pub use type_predicate::*;
mod type_reference;
pub use type_reference::*;
mod type_slice;
pub use type_slice::*;
mod type_trait_object;
pub use type_trait_object::*;
mod type_tuple;
pub use type_tuple::*;
mod typed_param;
pub use typed_param::*;
