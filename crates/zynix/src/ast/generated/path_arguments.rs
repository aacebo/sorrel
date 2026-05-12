use super::*;
#[derive(Debug, Clone)]
pub enum PathArguments {
    None {},
    AngleBracketed {
        args: crate::ast::Punctuated<GenericArgument, crate::token::Comma>,
    },
    Parenthesized {
        inputs: crate::ast::Punctuated<Type, crate::token::Comma>,
        output: Option<Type>,
    },
}
