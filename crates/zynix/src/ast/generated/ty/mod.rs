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
