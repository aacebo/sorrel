use super::*;
#[derive(Debug, Clone)]
pub enum Item {
    Use { value: ItemUse },
    ExternCrate { value: ItemExternCrate },
    Mod { value: ItemMod },
    Fn { value: ItemFn },
    Struct { value: ItemStruct },
    Enum { value: ItemEnum },
    Union { value: ItemUnion },
    Trait { value: ItemTrait },
    TraitAlias { value: ItemTraitAlias },
    Impl { value: ItemImpl },
    TypeAlias { value: ItemTypeAlias },
    Const { value: ItemConst },
    Static { value: ItemStatic },
    Macro { value: ItemMacro },
    Macro2 { value: ItemMacroRules },
    ForeignMod { value: ItemForeignMod },
    ExternBlock { value: ItemForeignMod },
}
