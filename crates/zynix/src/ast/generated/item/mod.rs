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
impl crate::ast::Visit for Item {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Item::Use { value } => {
                let _ = &value;
            }
            Item::ExternCrate { value } => {
                let _ = &value;
            }
            Item::Mod { value } => {
                let _ = &value;
            }
            Item::Fn { value } => {
                let _ = &value;
            }
            Item::Struct { value } => {
                let _ = &value;
            }
            Item::Enum { value } => {
                let _ = &value;
            }
            Item::Union { value } => {
                let _ = &value;
            }
            Item::Trait { value } => {
                let _ = &value;
            }
            Item::TraitAlias { value } => {
                let _ = &value;
            }
            Item::Impl { value } => {
                let _ = &value;
            }
            Item::TypeAlias { value } => {
                let _ = &value;
            }
            Item::Const { value } => {
                let _ = &value;
            }
            Item::Static { value } => {
                let _ = &value;
            }
            Item::Macro { value } => {
                let _ = &value;
            }
            Item::Macro2 { value } => {
                let _ = &value;
            }
            Item::ForeignMod { value } => {
                let _ = &value;
            }
            Item::ExternBlock { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for Item {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Item::Use { value } => Item::Use { value },
            Item::ExternCrate { value } => Item::ExternCrate { value },
            Item::Mod { value } => Item::Mod { value },
            Item::Fn { value } => Item::Fn { value },
            Item::Struct { value } => Item::Struct { value },
            Item::Enum { value } => Item::Enum { value },
            Item::Union { value } => Item::Union { value },
            Item::Trait { value } => Item::Trait { value },
            Item::TraitAlias { value } => Item::TraitAlias { value },
            Item::Impl { value } => Item::Impl { value },
            Item::TypeAlias { value } => Item::TypeAlias { value },
            Item::Const { value } => Item::Const { value },
            Item::Static { value } => Item::Static { value },
            Item::Macro { value } => Item::Macro { value },
            Item::Macro2 { value } => Item::Macro2 { value },
            Item::ForeignMod { value } => Item::ForeignMod { value },
            Item::ExternBlock { value } => Item::ExternBlock { value },
        }
    }
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
