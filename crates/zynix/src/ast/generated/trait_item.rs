use super::*;
#[derive(Debug, Clone)]
pub enum TraitItem {
    Fn { value: TraitItemFn },
    Const { value: TraitItemConst },
    Type { value: TraitItemType },
    Macro { value: TraitItemMacro },
}
