#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum TraitItem {
    Fn { value: TraitItemFn },
    Const { value: TraitItemConst },
    Type { value: TraitItemType },
    Macro { value: TraitItemMacro },
}
impl crate::ast::Visit for TraitItem {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            TraitItem::Fn { value } => {
                let _ = &value;
            }
            TraitItem::Const { value } => {
                let _ = &value;
            }
            TraitItem::Type { value } => {
                let _ = &value;
            }
            TraitItem::Macro { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for TraitItem {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            TraitItem::Fn { value } => TraitItem::Fn { value },
            TraitItem::Const { value } => TraitItem::Const { value },
            TraitItem::Type { value } => TraitItem::Type { value },
            TraitItem::Macro { value } => TraitItem::Macro { value },
        }
    }
}
mod trait_item_const;
pub use trait_item_const::*;
mod trait_item_fn;
pub use trait_item_fn::*;
mod trait_item_macro;
pub use trait_item_macro::*;
mod trait_item_type;
pub use trait_item_type::*;
