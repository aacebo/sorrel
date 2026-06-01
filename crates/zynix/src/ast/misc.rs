use crate::ast::{Attribute, Expr, Lifetime, Pattern, Punctuated, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::{For, If};
use crate::token::punct::{Colon, Comma, FatArrow, Gt, Lt, RArrow};
use crate::{Parse, Span, TokenStream};

// --- MatchArm: `pat (if guard)? => body` ---

#[doc = "A single arm of a `match` expression (`pat (if guard)? => body`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MatchArm {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub guard: Option<Box<Expr>>,
    pub body: Expr,
}

impl Parse for MatchArm {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let pat = stream.parse::<Pattern>()?;
        let guard = if stream.peek::<If>().is_some() {
            let _ = stream.parse::<If>()?;
            Some(Box::new(stream.parse::<Expr>()?))
        } else {
            None
        };
        let _ = stream.parse::<FatArrow>()?;
        let body = stream.parse::<Expr>()?;
        let _ = stream.parse::<Comma>();
        Ok(Self {
            span: Span::default(),
            attrs,
            pat,
            guard,
            body,
        })
    }
}

impl ToTokens for MatchArm {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.pat.to_tokens(t);
        if let Some(g) = &self.guard {
            crate::token::keyword::If::default().to_tokens(t);
            g.to_tokens(t);
        }
        FatArrow::default().to_tokens(t);
        self.body.to_tokens(t);
        Comma::default().to_tokens(t);
    }
}

// --- ClosureParam: `pat` or `pat: ty` ---

#[doc = "A closure parameter, either type-annotated (`pat: ty`) or inferred (`pat`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ClosureParam {
    Typed { pat: Box<Pattern>, ty: Box<Type> },
    Inferred { pat: Box<Pattern> },
}

impl Parse for ClosureParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        // `|` delimits the closure param list, so don't collect or-patterns here.
        let pat = Box::new(Pattern::parse_single(stream)?);
        if stream.peek::<Colon>().is_some() {
            let _ = stream.parse::<Colon>()?;
            let ty = Box::new(stream.parse::<Type>()?);
            Ok(ClosureParam::Typed { pat, ty })
        } else {
            Ok(ClosureParam::Inferred { pat })
        }
    }
}

impl ToTokens for ClosureParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            ClosureParam::Typed { pat, ty } => {
                pat.to_tokens(t);
                Colon::default().to_tokens(t);
                ty.to_tokens(t);
            }
            ClosureParam::Inferred { pat } => pat.to_tokens(t),
        }
    }
}

// --- ReturnType: `(-> Type)?` ---

#[doc = "The optional return type of a function (`-> Type` or nothing)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ReturnType {
    Default,
    Type(Box<Type>),
}

impl Parse for ReturnType {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<RArrow>().is_some() {
            let _ = stream.parse::<RArrow>()?;
            Ok(ReturnType::Type(Box::new(stream.parse::<Type>()?)))
        } else {
            Ok(ReturnType::Default)
        }
    }
}

impl ToTokens for ReturnType {
    fn to_tokens(&self, t: &mut TokenStream) {
        if let ReturnType::Type(ty) = self {
            RArrow::default().to_tokens(t);
            ty.to_tokens(t);
        }
    }
}

// --- BoundLifetimes: `for<'a, 'b>` ---

#[doc = "A `for<'a, 'b>` higher-ranked lifetime binder."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BoundLifetimes {
    pub span: Span,
    pub params: Punctuated<Lifetime, Comma>,
}

impl Parse for BoundLifetimes {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<For>()?;
        let _ = stream.parse::<Lt>()?;
        let mut params = Punctuated::new();
        while stream.peek::<Gt>().is_none() && !stream.is_empty() {
            params.push_value(stream.parse::<Lifetime>()?);
            if stream.peek::<Comma>().is_some() {
                params.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }
        let _ = stream.parse::<Gt>()?;
        Ok(Self {
            span: Span::default(),
            params,
        })
    }
}

impl ToTokens for BoundLifetimes {
    fn to_tokens(&self, t: &mut TokenStream) {
        For::default().to_tokens(t);
        Lt::default().to_tokens(t);
        self.params.to_tokens(t);
        Gt::default().to_tokens(t);
    }
}
