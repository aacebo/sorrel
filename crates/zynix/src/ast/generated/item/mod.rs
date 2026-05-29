#[allow(unused)]
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
mod item_const;
pub use item_const::*;
mod item_enum;
pub use item_enum::*;
mod item_extern_crate;
pub use item_extern_crate::*;
mod item_fn;
pub use item_fn::*;
mod item_foreign_mod;
pub use item_foreign_mod::*;
mod item_impl;
pub use item_impl::*;
mod item_macro;
pub use item_macro::*;
mod item_macro_rules;
pub use item_macro_rules::*;
mod item_mod;
pub use item_mod::*;
mod item_static;
pub use item_static::*;
mod item_struct;
pub use item_struct::*;
mod item_trait;
pub use item_trait::*;
mod item_trait_alias;
pub use item_trait_alias::*;
mod item_type_alias;
pub use item_type_alias::*;
mod item_union;
pub use item_union::*;
mod item_use;
pub use item_use::*;
