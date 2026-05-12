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
