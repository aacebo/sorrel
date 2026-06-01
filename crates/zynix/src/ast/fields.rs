use crate::ast::{Attribute, Expr, Ident, Mutability, Punctuated, Type, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::{Colon, Comma, Eq};
use crate::token::{Delim, Group, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "A struct/enum field definition (`pub name: Type` or `pub Type`)."]
#[derive(Debug, Clone)]
pub struct FieldDef {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Option<Ident>,
    pub ty: Type,
}

impl Parse for FieldDef {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let vis = stream.parse::<Visibility>()?;
        let mutability = stream.parse::<Mutability>()?;
        // Named field: `ident: ty` (an ident directly followed by `:`).
        let ident = {
            let mut fork = stream.fork();
            if let Ok(id) = fork.parse::<Ident>() {
                if fork.peek::<Colon>().is_some() {
                    stream.seek(&fork);
                    let _ = stream.parse::<Colon>()?;
                    Some(id)
                } else {
                    None
                }
            } else {
                None
            }
        };
        let ty = stream.parse::<Type>()?;
        Ok(Self {
            span: Span::default(),
            attrs,
            vis,
            mutability,
            ident,
            ty,
        })
    }
}

impl ToTokens for FieldDef {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.vis.to_tokens(t);
        self.mutability.to_tokens(t);
        if let Some(id) = &self.ident {
            id.to_tokens(t);
            Colon::default().to_tokens(t);
        }
        self.ty.to_tokens(t);
    }
}

#[doc = "Named struct fields (`{ a: A, b: B }`)."]
#[derive(Debug, Clone)]
pub struct FieldsNamed {
    pub span: Span,
    pub fields: Punctuated<FieldDef, Comma>,
}

#[doc = "Tuple-struct fields (`(A, B)`)."]
#[derive(Debug, Clone)]
pub struct FieldsUnnamed {
    pub span: Span,
    pub fields: Punctuated<FieldDef, Comma>,
}

impl Parse for FieldsNamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let group = stream.parse_group(Delim::Brace)?;
        let mut inner = group.parse();
        let fields = Punctuated::parse_terminated(&mut inner)?;
        Ok(Self {
            span: Span::default(),
            fields,
        })
    }
}

impl Parse for FieldsUnnamed {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let group = stream.parse_group(Delim::Paren)?;
        let mut inner = group.parse();
        let fields = Punctuated::parse_terminated(&mut inner)?;
        Ok(Self {
            span: Span::default(),
            fields,
        })
    }
}

impl ToTokens for FieldsNamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}

impl ToTokens for FieldsUnnamed {
    fn to_tokens(&self, t: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
    }
}

#[doc = "The fields of a struct/enum variant (named, unnamed, or unit)."]
#[derive(Debug, Clone)]
pub enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}

impl From<FieldsNamed> for Fields {
    fn from(v: FieldsNamed) -> Self {
        Fields::Named(v)
    }
}
impl From<FieldsUnnamed> for Fields {
    fn from(v: FieldsUnnamed) -> Self {
        Fields::Unnamed(v)
    }
}

impl Parse for Fields {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        match stream.curr() {
            Some(TokenTree::Group(g)) if g.delim() == Delim::Brace => {
                Ok(Fields::Named(stream.parse()?))
            }
            Some(TokenTree::Group(g)) if g.delim() == Delim::Paren => {
                Ok(Fields::Unnamed(stream.parse()?))
            }
            _ => Ok(Fields::Unit),
        }
    }
}

impl ToTokens for Fields {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Fields::Named(v) => v.to_tokens(t),
            Fields::Unnamed(v) => v.to_tokens(t),
            Fields::Unit => {}
        }
    }
}

#[doc = "An enum variant (`Name`, `Name(T)`, `Name { x: T }`, `Name = 1`)."]
#[derive(Debug, Clone)]
pub struct Variant {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub fields: Fields,
    pub discriminant: Option<Expr>,
}

impl Parse for Variant {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let ident = stream.parse::<Ident>()?;
        let fields = stream.parse::<Fields>()?;
        let discriminant = if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            Some(stream.parse::<Expr>()?)
        } else {
            None
        };
        Ok(Self {
            span: Span::default(),
            attrs,
            ident,
            fields,
            discriminant,
        })
    }
}

impl ToTokens for Variant {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.ident.to_tokens(t);
        self.fields.to_tokens(t);
        if let Some(d) = &self.discriminant {
            Eq::default().to_tokens(t);
            d.to_tokens(t);
        }
    }
}
