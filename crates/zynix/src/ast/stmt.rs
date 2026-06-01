use crate::ast::{Attribute, Expr, MacroCall, Pattern, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Else, Let};
use crate::token::punct::{Eq, Semi};
use crate::token::{Delim, Group, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "A braced block of statements (`{ stmt; stmt; expr }`)."]
#[derive(Debug, Clone)]
pub struct Block {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

impl Parse for Block {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let group = stream.parse_group(Delim::Brace)?;
        let mut inner = group.parse();
        let mut stmts = Vec::new();
        while !inner.is_empty() {
            stmts.push(inner.parse::<Stmt>()?);
        }
        Ok(Self {
            span: Span::default(),
            stmts,
        })
    }
}

impl ToTokens for Block {
    fn to_tokens(&self, t: &mut TokenStream) {
        let mut inner = TokenStream::new();
        for s in &self.stmts {
            s.to_tokens(&mut inner);
        }
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}

#[derive(Debug, Clone)]
pub struct Local {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub ty: Option<Type>,
    pub init: Option<LocalInit>,
}

#[derive(Debug, Clone)]
pub struct LocalInit {
    pub span: Span,
    pub expr: Expr,
    pub diverge: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct StmtMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}

impl ToTokens for Local {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Let::default().to_tokens(t);
        self.pat.to_tokens(t);
        if let Some(ty) = &self.ty {
            crate::token::punct::Colon::default().to_tokens(t);
            ty.to_tokens(t);
        }
        if let Some(init) = &self.init {
            Eq::default().to_tokens(t);
            init.expr.to_tokens(t);
            if let Some(div) = &init.diverge {
                Else::default().to_tokens(t);
                div.to_tokens(t);
            }
        }
        Semi::default().to_tokens(t);
    }
}

impl ToTokens for StmtMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.mac.to_tokens(t);
        if self.semi {
            Semi::default().to_tokens(t);
        }
    }
}

#[doc = "A statement in a block."]
#[derive(Debug, Clone)]
pub enum Stmt {
    Local(Box<Local>),
    Item(Box<crate::ast::Item>),
    Expr(Box<Expr>),
    Semi(Box<Expr>),
    Macro(StmtMacro),
}

impl Parse for Stmt {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;

        // `let` binding.
        if stream.peek::<Let>().is_some() {
            let _ = stream.parse::<Let>()?;
            let pat = stream.parse::<Pattern>()?;
            let ty = if stream.peek::<crate::token::punct::Colon>().is_some() {
                let _ = stream.parse::<crate::token::punct::Colon>()?;
                Some(stream.parse::<Type>()?)
            } else {
                None
            };
            let init = if stream.peek::<Eq>().is_some() {
                let _ = stream.parse::<Eq>()?;
                let expr = stream.parse::<Expr>()?;
                let diverge = if stream.peek::<Else>().is_some() {
                    let _ = stream.parse::<Else>()?;
                    Some(Box::new(stream.parse::<Expr>()?))
                } else {
                    None
                };
                Some(LocalInit {
                    span: Span::default(),
                    expr,
                    diverge,
                })
            } else {
                None
            };
            let _ = stream.parse::<Semi>();
            return Ok(Stmt::Local(Box::new(Local {
                span: Span::default(),
                attrs,
                pat,
                ty,
                init,
            })));
        }

        // Expression statement (with or without trailing `;`).
        let expr = stream.parse::<Expr>()?;
        if stream.peek::<Semi>().is_some() {
            let _ = stream.parse::<Semi>()?;
            Ok(Stmt::Semi(Box::new(expr)))
        } else {
            Ok(Stmt::Expr(Box::new(expr)))
        }
    }
}

impl ToTokens for Stmt {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Stmt::Local(v) => v.to_tokens(t),
            Stmt::Item(v) => v.to_tokens(t),
            Stmt::Expr(v) => v.to_tokens(t),
            Stmt::Semi(v) => {
                v.to_tokens(t);
                Semi::default().to_tokens(t);
            }
            Stmt::Macro(v) => v.to_tokens(t),
        }
    }
}
