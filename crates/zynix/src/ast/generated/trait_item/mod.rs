#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum TraitItem {
    Fn { value: TraitItemFn },
    Const { value: TraitItemConst },
    Type { value: TraitItemType },
    Macro { value: TraitItemMacro },
}
mod trait_item_const;
pub use trait_item_const::*;
mod trait_item_fn;
pub use trait_item_fn::*;
mod trait_item_macro;
pub use trait_item_macro::*;
mod trait_item_type;
pub use trait_item_type::*;
