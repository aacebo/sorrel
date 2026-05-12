use super::*;
#[derive(Debug, Clone)]
pub enum ImplItem {
    Fn { value: ImplItemFn },
    Const { value: ImplItemConst },
    Type { value: ImplItemType },
    Macro { value: ImplItemMacro },
}
