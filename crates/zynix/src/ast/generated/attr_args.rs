#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum AttrArgs {
    Empty {},
    Delimited {
        delim: DelimiterKind,
        tokens: crate::TokenStream,
    },
    Meta {
        meta: Box<Meta>,
    },
}
impl crate::ast::Visit for AttrArgs {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            AttrArgs::Empty {} => {}
            AttrArgs::Delimited { delim, tokens } => {
                let _ = &delim;
                let _ = &tokens;
            }
            AttrArgs::Meta { meta } => {
                let _ = &meta;
            }
        }
    }
}
impl crate::ast::Fold for AttrArgs {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            AttrArgs::Empty {} => AttrArgs::Empty {},
            AttrArgs::Delimited { delim, tokens } => AttrArgs::Delimited { delim, tokens },
            AttrArgs::Meta { meta } => AttrArgs::Meta { meta },
        }
    }
}
