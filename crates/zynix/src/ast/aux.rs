use crate::ast::{Attribute, Expr, GenericArgument, Lifetime, Member, Pattern, Punctuated, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::{For, If};
use crate::token::punct::{Colon, Comma, FatArrow, Gt, Lt, RArrow};
use crate::{Parse, Span, TokenStream};

// --- MatchArm: `pat (if guard)? => body` ---

#[derive(Debug, Clone)]
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

// --- FieldValue: `member: expr` or shorthand `member` ---

#[derive(Debug, Clone)]
pub struct FieldValue {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub member: Member,
    pub expr: Expr,
    pub shorthand: bool,
}

impl Parse for FieldValue {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let member = stream.parse::<Member>()?;
        if stream.peek::<Colon>().is_some() {
            let _ = stream.parse::<Colon>()?;
            let expr = stream.parse::<Expr>()?;
            Ok(Self {
                span: Span::default(),
                attrs,
                member,
                expr,
                shorthand: false,
            })
        } else {
            // shorthand: `field` means `field: field`
            let expr = match &member {
                Member::Named(id) => Expr::Path(crate::ast::ExprPath {
                    span: Span::default(),
                    attrs: Vec::new(),
                    qself: None,
                    path: id.clone().into(),
                }),
                Member::Unnamed(_) => {
                    return Err(crate::token::LexError::new(stream.span())
                        .message("tuple index needs a value")
                        .into());
                }
            };
            Ok(Self {
                span: Span::default(),
                attrs,
                member,
                expr,
                shorthand: true,
            })
        }
    }
}

impl ToTokens for FieldValue {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        if self.shorthand {
            self.member.to_tokens(t);
        } else {
            self.member.to_tokens(t);
            Colon::default().to_tokens(t);
            self.expr.to_tokens(t);
        }
    }
}

// --- ClosureParam: `pat` or `pat: ty` ---

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

// --- AngleArgs: `<args,*>` ---

#[derive(Debug, Clone)]
pub struct AngleArgs {
    pub span: Span,
    pub args: Punctuated<GenericArgument, Comma>,
}

impl Parse for AngleArgs {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<Lt>()?;
        let mut args = Punctuated::new();
        while !stream.peek_angle_close() && !stream.is_empty() {
            args.push_value(stream.parse::<GenericArgument>()?);
            if stream.peek::<Comma>().is_some() {
                args.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }
        stream.eat_angle_close()?;
        Ok(Self {
            span: Span::default(),
            args,
        })
    }
}

impl ToTokens for AngleArgs {
    fn to_tokens(&self, t: &mut TokenStream) {
        Lt::default().to_tokens(t);
        self.args.to_tokens(t);
        Gt::default().to_tokens(t);
    }
}

// --- BoundLifetimes: `for<'a, 'b>` ---

#[derive(Debug, Clone)]
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
