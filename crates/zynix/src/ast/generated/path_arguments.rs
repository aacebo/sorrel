#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum PathArguments {
    None {},
    AngleBracketed {
        args: crate::ast::Punctuated<GenericArgument, crate::token::Comma>,
    },
    Parenthesized {
        inputs: crate::ast::Punctuated<Type, crate::token::Comma>,
        output: Option<Box<Type>>,
    },
}
impl crate::ast::Visit for PathArguments {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            PathArguments::None {} => {}
            PathArguments::AngleBracketed { args } => {
                let _ = &args;
            }
            PathArguments::Parenthesized { inputs, output } => {
                let _ = &inputs;
                let _ = &output;
            }
        }
    }
}
impl crate::ast::Fold for PathArguments {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            PathArguments::None {} => PathArguments::None {},
            PathArguments::AngleBracketed { args } => PathArguments::AngleBracketed { args },
            PathArguments::Parenthesized { inputs, output } => {
                PathArguments::Parenthesized { inputs, output }
            }
        }
    }
}
