use zynix_macros::{Parse, ToTokens};

use crate::ast::{Attribute, BoundLifetimes, BoundPolarity, Expr, Ident, Lifetime, Path, Punctuated, TraitBoundModifier, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::{Const, Where};
use crate::token::punct::{Colon, Comma, Eq, Gt, Lt, Plus};
use crate::{Parse, Span, TokenStream};

// --- Bounds ---

#[doc = "A trait reference (`Trait`, `!Trait`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
pub struct TraitRef {
    #[parse(skip)]
    pub span: Span,
    pub polarity: BoundPolarity,
    pub path: Path,
}

#[doc = "A trait bound (`Trait`, `?Sized`, `for<'a> Trait`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
pub struct TraitBound {
    #[parse(skip)]
    pub span: Span,
    pub polarity: BoundPolarity,
    pub lifetimes: Option<BoundLifetimes>,
    pub modifier: TraitBoundModifier,
    pub path: Path,
}

#[doc = "A `use<'a, T>` bound (precise capturing)."]
#[derive(Debug, Clone)]
pub struct UseBound {
    pub span: Span,
    pub lifetimes: Punctuated<Lifetime, Comma>,
}

impl Parse for UseBound {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<crate::token::keyword::Use>()?;
        let _ = stream.parse::<Lt>()?;
        let mut lifetimes = Punctuated::new();
        while !stream.peek_angle_close() && !stream.is_empty() {
            lifetimes.push_value(stream.parse::<Lifetime>()?);
            if stream.peek::<Comma>().is_some() {
                lifetimes.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }
        stream.eat_angle_close()?;
        Ok(Self {
            span: Span::default(),
            lifetimes,
        })
    }
}

impl ToTokens for UseBound {
    fn to_tokens(&self, t: &mut TokenStream) {
        crate::token::keyword::Use::default().to_tokens(t);
        Lt::default().to_tokens(t);
        self.lifetimes.to_tokens(t);
        Gt::default().to_tokens(t);
    }
}

#[doc = "A bound on a type parameter (`Trait`, `'a`, `use<>`)."]
#[derive(Debug, Clone)]
pub enum TypeBound {
    Trait(TraitBound),
    Lifetime(Lifetime),
    Use(UseBound),
}

impl From<TraitBound> for TypeBound {
    fn from(v: TraitBound) -> Self {
        TypeBound::Trait(v)
    }
}
impl From<UseBound> for TypeBound {
    fn from(v: UseBound) -> Self {
        TypeBound::Use(v)
    }
}

impl Parse for TypeBound {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if matches!(
            stream.curr(),
            Some(crate::TokenTree::Token(crate::Token::Punct(
                crate::token::Punctuation::Quote(_)
            )))
        ) {
            return Ok(TypeBound::Lifetime(stream.parse()?));
        }
        if stream.peek::<crate::token::keyword::Use>().is_some() {
            return Ok(TypeBound::Use(stream.parse()?));
        }
        Ok(TypeBound::Trait(stream.parse()?))
    }
}

impl ToTokens for TypeBound {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            TypeBound::Trait(v) => v.to_tokens(t),
            TypeBound::Lifetime(v) => v.to_tokens(t),
            TypeBound::Use(v) => v.to_tokens(t),
        }
    }
}

// --- Generic params ---

#[doc = "A lifetime parameter (`'a: 'b + 'c`)."]
#[derive(Debug, Clone)]
pub struct LifetimeParam {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub lifetime: Lifetime,
    pub bounds: Punctuated<Lifetime, Plus>,
}

impl Parse for LifetimeParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let lifetime = stream.parse::<Lifetime>()?;
        let bounds = parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            attrs,
            lifetime,
            bounds,
        })
    }
}

impl ToTokens for LifetimeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.lifetime.to_tokens(t);
        emit_bounds(&self.bounds, t);
    }
}

#[doc = "A const generic parameter (`const N: usize = 0`)."]
#[derive(Debug, Clone)]
pub struct ConstParam {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
}

impl Parse for ConstParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let _ = stream.parse::<Const>()?;
        let ident = stream.parse::<Ident>()?;
        let _ = stream.parse::<Colon>()?;
        let ty = stream.parse::<Type>()?;
        let default = if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            Some(stream.parse::<Expr>()?)
        } else {
            None
        };
        Ok(Self {
            span: Span::default(),
            attrs,
            ident,
            ty,
            default,
        })
    }
}

impl ToTokens for ConstParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Const::default().to_tokens(t);
        self.ident.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
        if let Some(d) = &self.default {
            Eq::default().to_tokens(t);
            d.to_tokens(t);
        }
    }
}

#[doc = "A type parameter (`T: Bound = Default`)."]
#[derive(Debug, Clone)]
pub struct TypeParam {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub bounds: Punctuated<TypeBound, Plus>,
    pub default: Option<Type>,
}

impl Parse for TypeParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let ident = stream.parse::<Ident>()?;
        let bounds = if stream.peek::<Colon>().is_some() {
            let _ = stream.parse::<Colon>()?;
            parse_type_bounds(stream)?
        } else {
            Punctuated::new()
        };
        let default = if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            Some(stream.parse::<Type>()?)
        } else {
            None
        };
        Ok(Self {
            span: Span::default(),
            attrs,
            ident,
            bounds,
            default,
        })
    }
}

impl ToTokens for TypeParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.ident.to_tokens(t);
        if !self.bounds.is_empty() {
            Colon::default().to_tokens(t);
            self.bounds.to_tokens(t);
        }
        if let Some(d) = &self.default {
            Eq::default().to_tokens(t);
            d.to_tokens(t);
        }
    }
}

#[doc = "A generic parameter (lifetime, type, or const)."]
#[derive(Debug, Clone)]
pub enum GenericParam {
    Lifetime(LifetimeParam),
    Type(TypeParam),
    Const(ConstParam),
}

impl From<LifetimeParam> for GenericParam {
    fn from(v: LifetimeParam) -> Self {
        GenericParam::Lifetime(v)
    }
}
impl From<TypeParam> for GenericParam {
    fn from(v: TypeParam) -> Self {
        GenericParam::Type(v)
    }
}
impl From<ConstParam> for GenericParam {
    fn from(v: ConstParam) -> Self {
        GenericParam::Const(v)
    }
}

impl Parse for GenericParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        // attrs belong to the inner param; peek past them is handled by each.
        if matches!(
            stream.curr(),
            Some(crate::TokenTree::Token(crate::Token::Punct(
                crate::token::Punctuation::Quote(_)
            )))
        ) {
            return Ok(GenericParam::Lifetime(stream.parse()?));
        }
        // `const N: T` vs a type param starting with `const`-like? only const params start with `const`.
        let mut fork = stream.fork();
        let _ = fork.parse_vec::<Attribute>();
        if fork.peek::<Const>().is_some() {
            return Ok(GenericParam::Const(stream.parse()?));
        }
        Ok(GenericParam::Type(stream.parse()?))
    }
}

impl ToTokens for GenericParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            GenericParam::Lifetime(v) => v.to_tokens(t),
            GenericParam::Type(v) => v.to_tokens(t),
            GenericParam::Const(v) => v.to_tokens(t),
        }
    }
}

// --- where clause ---

#[doc = "A predicate in a `where` clause."]
#[derive(Debug, Clone)]
pub struct LifetimePredicate {
    pub span: Span,
    pub lifetime: Lifetime,
    pub bounds: Punctuated<Lifetime, Plus>,
}

impl Parse for LifetimePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lifetime = stream.parse::<Lifetime>()?;
        let bounds = parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            lifetime,
            bounds,
        })
    }
}

impl ToTokens for LifetimePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lifetime.to_tokens(t);
        emit_bounds(&self.bounds, t);
    }
}

#[doc = "A type predicate in a `where` clause (`T: Bound`)."]
#[derive(Debug, Clone)]
pub struct TypePredicate {
    pub span: Span,
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: Type,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl Parse for TypePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lifetimes = stream.parse_opt::<BoundLifetimes>();
        let bounded_ty = stream.parse::<Type>()?;
        let _ = stream.parse::<Colon>()?;
        let bounds = parse_type_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            lifetimes,
            bounded_ty,
            bounds,
        })
    }
}

impl ToTokens for TypePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        if let Some(l) = &self.lifetimes {
            l.to_tokens(t);
        }
        self.bounded_ty.to_tokens(t);
        Colon::default().to_tokens(t);
        self.bounds.to_tokens(t);
    }
}

#[doc = "A `where` clause predicate (lifetime or type)."]
#[derive(Debug, Clone)]
pub enum WherePredicate {
    Lifetime(LifetimePredicate),
    Type(Box<TypePredicate>),
}

impl Parse for WherePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if matches!(
            stream.curr(),
            Some(crate::TokenTree::Token(crate::Token::Punct(
                crate::token::Punctuation::Quote(_)
            )))
        ) {
            return Ok(WherePredicate::Lifetime(stream.parse()?));
        }
        Ok(WherePredicate::Type(Box::new(stream.parse()?)))
    }
}

impl ToTokens for WherePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            WherePredicate::Lifetime(v) => v.to_tokens(t),
            WherePredicate::Type(v) => v.to_tokens(t),
        }
    }
}

#[doc = "A `where` clause."]
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub span: Span,
    pub predicates: Punctuated<WherePredicate, Comma>,
}

impl Parse for WhereClause {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<Where>()?;
        // Parse predicates until a `{`/`;`/`(` body or end.
        let mut predicates = Punctuated::new();
        while !stream.is_empty() && !matches!(stream.curr(), Some(crate::TokenTree::Group(_))) {
            predicates.push_value(stream.parse::<WherePredicate>()?);
            if stream.peek::<Comma>().is_some() {
                predicates.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }
        Ok(Self {
            span: Span::default(),
            predicates,
        })
    }
}

impl ToTokens for WhereClause {
    fn to_tokens(&self, t: &mut TokenStream) {
        Where::default().to_tokens(t);
        self.predicates.to_tokens(t);
    }
}

// --- Generics ---

#[doc = "Generic parameters and an optional `where` clause."]
#[derive(Debug, Clone)]
pub struct Generics {
    pub span: Span,
    pub params: Punctuated<GenericParam, Comma>,
    pub where_clause: Option<WhereClause>,
}

impl Parse for Generics {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let params = if stream.peek::<Lt>().is_some() {
            let _ = stream.parse::<Lt>()?;
            let mut params = Punctuated::new();
            while !stream.peek_angle_close() && !stream.is_empty() {
                params.push_value(stream.parse::<GenericParam>()?);
                if stream.peek::<Comma>().is_some() {
                    params.push_punct(stream.parse::<Comma>()?);
                } else {
                    break;
                }
            }
            stream.eat_angle_close()?;
            params
        } else {
            Punctuated::new()
        };

        let where_clause = if stream.peek::<Where>().is_some() {
            Some(stream.parse::<WhereClause>()?)
        } else {
            None
        };

        Ok(Self {
            span: Span::default(),
            params,
            where_clause,
        })
    }
}

impl ToTokens for Generics {
    fn to_tokens(&self, t: &mut TokenStream) {
        if !self.params.is_empty() {
            Lt::default().to_tokens(t);
            self.params.to_tokens(t);
            Gt::default().to_tokens(t);
        }
        if let Some(w) = &self.where_clause {
            w.to_tokens(t);
        }
    }
}

// --- helpers ---

fn parse_bounds(stream: &mut ParseStream) -> Result<Punctuated<Lifetime, Plus>, ParseError> {
    let mut bounds = Punctuated::new();
    if stream.peek::<Colon>().is_some() {
        let _ = stream.parse::<Colon>()?;
        loop {
            bounds.push_value(stream.parse::<Lifetime>()?);
            if stream.peek::<Plus>().is_some() {
                bounds.push_punct(stream.parse::<Plus>()?);
            } else {
                break;
            }
        }
    }
    Ok(bounds)
}

fn parse_type_bounds(stream: &mut ParseStream) -> Result<Punctuated<TypeBound, Plus>, ParseError> {
    let mut bounds = Punctuated::new();
    loop {
        bounds.push_value(stream.parse::<TypeBound>()?);
        if stream.peek::<Plus>().is_some() {
            bounds.push_punct(stream.parse::<Plus>()?);
        } else {
            break;
        }
    }
    Ok(bounds)
}

fn emit_bounds(bounds: &Punctuated<Lifetime, Plus>, t: &mut TokenStream) {
    if !bounds.is_empty() {
        Colon::default().to_tokens(t);
        bounds.to_tokens(t);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::TokenStream;
    use crate::token::ToTokenStream;

    fn parse<T: Parse>(src: &str) -> T {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>().unwrap()
    }

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn generics_basic() {
        let g: Generics = parse("<T>");
        assert_eq!(g.params.len(), 1);
        assert!(matches!(g.params.first().unwrap(), GenericParam::Type(_)));

        let g2: Generics = parse("<'a, T: Clone, const N: usize>");
        assert_eq!(g2.params.len(), 3);
        assert!(matches!(g2.params.first().unwrap(), GenericParam::Lifetime(_)));
    }

    #[test]
    fn generics_where() {
        let g: Generics = parse("<T> where T: Clone");
        assert!(g.where_clause.is_some());
    }

    #[test]
    fn type_bounds() {
        assert!(matches!(parse::<TypeBound>("Clone"), TypeBound::Trait(_)));
        assert!(matches!(parse::<TypeBound>("'a"), TypeBound::Lifetime(_)));
        assert!(matches!(parse::<TypeBound>("?Sized"), TypeBound::Trait(_)));
    }

    #[test]
    fn impl_dyn_types() {
        use crate::ast::Type;
        assert!(matches!(parse::<Type>("impl Clone"), Type::ImplTrait(_)));
        assert!(matches!(parse::<Type>("dyn Clone + 'a"), Type::TraitObject(_)));
        assert_eq!(render(&parse::<Type>("impl Clone")), "impl Clone");
    }
}
