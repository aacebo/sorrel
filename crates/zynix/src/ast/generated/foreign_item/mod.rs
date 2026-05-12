#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum ForeignItem {
    Fn { value: ForeignItemFn },
    Static { value: ForeignItemStatic },
    Type { value: ForeignItemType },
    Macro { value: ForeignItemMacro },
}
impl crate::ast::Visit for ForeignItem {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            ForeignItem::Fn { value } => {
                let _ = &value;
            }
            ForeignItem::Static { value } => {
                let _ = &value;
            }
            ForeignItem::Type { value } => {
                let _ = &value;
            }
            ForeignItem::Macro { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for ForeignItem {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            ForeignItem::Fn { value } => ForeignItem::Fn { value },
            ForeignItem::Static { value } => ForeignItem::Static { value },
            ForeignItem::Type { value } => ForeignItem::Type { value },
            ForeignItem::Macro { value } => ForeignItem::Macro { value },
        }
    }
}
mod foreign_item_fn;
pub use foreign_item_fn::*;
mod foreign_item_macro;
pub use foreign_item_macro::*;
mod foreign_item_static;
pub use foreign_item_static::*;
mod foreign_item_type;
pub use foreign_item_type::*;
