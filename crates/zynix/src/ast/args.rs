use crate::ast::{AngleArgs, Expr, Ident, Lifetime, Punctuated, Type, TypeBound};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::{Colon, Eq, Lt, Plus};
use crate::token::{ToTokens, Token, TokenTree};
use crate::{Parse, Span, TokenStream};

#[doc = "An associated type binding (`Item = T`)."]
#[derive(Debug, Clone)]
pub struct AssocTypeArg {
    pub span: Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub ty: Type,
}

#[doc = "An associated const binding (`N = 8`)."]
#[derive(Debug, Clone)]
pub struct AssocConstArg {
    pub span: Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub expr: Expr,
}

#[doc = "An associated type bound constraint (`Item: Bound`)."]
#[derive(Debug, Clone)]
pub struct ConstraintArg {
    pub span: Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub bounds: Punctuated<TypeBound, Plus>,
}

fn emit_assoc_head(ident: &Ident, generics: &Option<AngleArgs>, t: &mut TokenStream) {
    ident.to_tokens(t);
    if let Some(g) = generics {
        g.to_tokens(t);
    }
}

impl ToTokens for AssocTypeArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_assoc_head(&self.ident, &self.generics, t);
        Eq::default().to_tokens(t);
        self.ty.to_tokens(t);
    }
}

impl ToTokens for AssocConstArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_assoc_head(&self.ident, &self.generics, t);
        Eq::default().to_tokens(t);
        self.expr.to_tokens(t);
    }
}

impl ToTokens for ConstraintArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_assoc_head(&self.ident, &self.generics, t);
        Colon::default().to_tokens(t);
        self.bounds.to_tokens(t);
    }
}

#[doc = "A single generic argument inside `<...>`."]
#[derive(Debug, Clone)]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Const(Expr),
    AssocType(AssocTypeArg),
    AssocConst(AssocConstArg),
    Constraint(ConstraintArg),
}

impl Parse for GenericArgument {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        // Lifetime argument.
        if matches!(
            stream.curr(),
            Some(TokenTree::Token(Token::Punct(
                crate::token::Punctuation::Quote(_)
            )))
        ) {
            return Ok(GenericArgument::Lifetime(stream.parse()?));
        }

        // `Ident (<...>)? = ...` (assoc) or `Ident (<...>)? : bounds` (constraint).
        if let Some(arg) = try_assoc_or_constraint(stream)? {
            return Ok(arg);
        }

        // Const argument: a literal or a `{ ... }` block expression.
        let is_const = matches!(stream.curr(), Some(TokenTree::Token(Token::Literal(_))))
            || matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == crate::token::Delim::Brace);
        if is_const {
            return Ok(GenericArgument::Const(stream.parse()?));
        }

        // Otherwise a type.
        Ok(GenericArgument::Type(stream.parse()?))
    }
}

/// Try to parse `Ident (<...>)? = T|expr` or `Ident (<...>)? : bounds`.
fn try_assoc_or_constraint(
    stream: &mut ParseStream,
) -> Result<Option<GenericArgument>, ParseError> {
    let mut fork = stream.fork();

    // A bare ident, optionally with its own `<...>`.
    let Ok(ident) = fork.parse::<Ident>() else {
        return Ok(None);
    };
    let generics = if fork.peek::<Lt>().is_some() {
        Some(fork.parse::<AngleArgs>()?)
    } else {
        None
    };

    if fork.peek::<Eq>().is_some() {
        let _ = fork.parse::<Eq>()?;
        // `Item = Type` vs `N = const-expr`: prefer a type, fall back to expr.
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
        let expr = fork.parse::<Expr>()?;
        stream.seek(&fork);
        return Ok(Some(GenericArgument::AssocConst(AssocConstArg {
            span: Span::default(),
            ident,
            generics,
            expr,
        })));
    }

    if fork.peek::<Colon>().is_some() {
        let _ = fork.parse::<Colon>()?;
        let mut bounds = Punctuated::new();
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
