#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum ImplItem {
    Fn { value: ImplItemFn },
    Const { value: ImplItemConst },
    Type { value: ImplItemType },
    Macro { value: ImplItemMacro },
}
impl crate::ast::Visit for ImplItem {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            ImplItem::Fn { value } => {
                let _ = &value;
            }
            ImplItem::Const { value } => {
                let _ = &value;
            }
            ImplItem::Type { value } => {
                let _ = &value;
            }
            ImplItem::Macro { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for ImplItem {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            ImplItem::Fn { value } => ImplItem::Fn { value },
            ImplItem::Const { value } => ImplItem::Const { value },
            ImplItem::Type { value } => ImplItem::Type { value },
            ImplItem::Macro { value } => ImplItem::Macro { value },
        }
    }
}
mod impl_item_const;
pub use impl_item_const::*;
mod impl_item_fn;
pub use impl_item_fn::*;
mod impl_item_macro;
pub use impl_item_macro::*;
mod impl_item_type;
pub use impl_item_type::*;
