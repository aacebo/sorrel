use crate::ast::{Lifetime, Type, TypeBound};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::{Comma, Eq, Lt, Plus};
use crate::token::{ToTokens, Token, TokenTree};
use crate::{Parse, Span, TokenStream};

mod angle_args;
mod assoc_const_arg;
mod assoc_type_arg;
mod constraint_arg;

pub use angle_args::*;
pub use assoc_const_arg::*;
pub use assoc_type_arg::*;
pub use constraint_arg::*;

#[doc = "A single generic argument inside `<...>`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Const(crate::ast::Expr),
    AssocType(AssocTypeArg),
    AssocConst(AssocConstArg),
    Constraint(ConstraintArg),
}

impl Parse for GenericArgument {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if matches!(
            stream.curr(),
            Some(TokenTree::Token(Token::Punct(crate::token::Punctuation::Quote(_))))
        ) {
            return Ok(GenericArgument::Lifetime(stream.parse()?));
        }

        if let Some(arg) = try_assoc_or_constraint(stream)? {
            return Ok(arg);
        }

        let is_const = matches!(stream.curr(), Some(TokenTree::Token(Token::Literal(_))))
            || matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == crate::token::Delim::Brace);
        if is_const {
            return Ok(GenericArgument::Const(stream.parse()?));
        }

        Ok(GenericArgument::Type(stream.parse()?))
    }
}

impl ToTokens for GenericArgument {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            GenericArgument::Lifetime(v) => v.to_tokens(t),
            GenericArgument::Type(v) => v.to_tokens(t),
            GenericArgument::Const(v) => v.to_tokens(t),
            GenericArgument::AssocType(v) => v.to_tokens(t),
            GenericArgument::AssocConst(v) => v.to_tokens(t),
            GenericArgument::Constraint(v) => v.to_tokens(t),
        }
    }
}

fn try_assoc_or_constraint(stream: &mut ParseStream) -> Result<Option<GenericArgument>, ParseError> {
    let mut fork = stream.fork();

    let Ok(ident) = fork.parse::<crate::ast::Ident>() else {
        return Ok(None);
    };
    let generics = if fork.peek::<Lt>().is_some() {
        Some(fork.parse::<AngleArgs>()?)
    } else {
        None
    };

    if fork.peek::<Eq>().is_some() {
        let _ = fork.parse::<Eq>()?;
        let mut ty_fork = fork.fork();
        if let Ok(ty) = ty_fork.parse::<Type>() {
            stream.seek(&ty_fork);
            return Ok(Some(GenericArgument::AssocType(AssocTypeArg {
                span: Span::default(),
                ident,
                generics,
                ty,
            })));
        }
        let expr = fork.parse::<crate::ast::Expr>()?;
        stream.seek(&fork);
        return Ok(Some(GenericArgument::AssocConst(AssocConstArg {
            span: Span::default(),
            ident,
            generics,
            expr,
        })));
    }

    if fork.peek::<crate::token::punct::Colon>().is_some() {
        let _ = fork.parse::<crate::token::punct::Colon>()?;
        let mut bounds = crate::ast::Punctuated::new();
        loop {
            bounds.push_value(fork.parse::<TypeBound>()?);
            if fork.peek::<Plus>().is_some() {
                bounds.push_punct(fork.parse::<Plus>()?);
            } else {
                break;
            }
        }
        stream.seek(&fork);
        return Ok(Some(GenericArgument::Constraint(ConstraintArg {
            span: Span::default(),
            ident,
            generics,
            bounds,
        })));
    }

    Ok(None)
}
