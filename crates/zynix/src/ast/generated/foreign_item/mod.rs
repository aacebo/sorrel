#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum ForeignItem {
    Fn { value: ForeignItemFn },
    Static { value: ForeignItemStatic },
    Type { value: ForeignItemType },
    Macro { value: ForeignItemMacro },
}
mod foreign_item_fn;
pub use foreign_item_fn::*;
mod foreign_item_macro;
pub use foreign_item_macro::*;
mod foreign_item_static;
pub use foreign_item_static::*;
mod foreign_item_type;
pub use foreign_item_type::*;
