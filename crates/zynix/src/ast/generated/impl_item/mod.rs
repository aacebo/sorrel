#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum ImplItem {
    Fn { value: ImplItemFn },
    Const { value: ImplItemConst },
    Type { value: ImplItemType },
    Macro { value: ImplItemMacro },
}
mod impl_item_const;
pub use impl_item_const::*;
mod impl_item_fn;
pub use impl_item_fn::*;
mod impl_item_macro;
pub use impl_item_macro::*;
mod impl_item_type;
pub use impl_item_type::*;
