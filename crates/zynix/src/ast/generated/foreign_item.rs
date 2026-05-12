use super::*;
#[derive(Debug, Clone)]
pub enum ForeignItem {
    Fn { value: ForeignItemFn },
    Static { value: ForeignItemStatic },
    Type { value: ForeignItemType },
    Macro { value: ForeignItemMacro },
}
