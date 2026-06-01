use crate::ast::{
    Asyncness, Attribute, Constness, Generics, Ident, Lifetime, Mutability, Punctuated, ReturnType,
    Type, TypedParam, Unsafety,
};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Extern, Fn, SelfValue};
use crate::token::punct::{And, Comma, DotDotDot};
use crate::token::{self, Delim, ToTokens, Token, TokenTree};
use crate::{Parse, Span, TokenStream};

// --- Abi: `"C"` (after `extern`) ---

#[doc = "An ABI string (`extern \"C\"`)."]
#[derive(Debug, Clone)]
pub struct Abi {
    pub span: Span,
    pub name: Option<String>,
}

impl Parse for Abi {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<Extern>()?;
        let name = match stream.curr() {
            Some(TokenTree::Token(Token::Literal(lit))) if lit.repr().starts_with('"') => {
                let repr = lit.repr().to_string();
                stream.advance();
                Some(repr.trim_matches('"').to_string())
            }
            _ => None,
        };
        Ok(Self {
            span: Span::default(),
            name,
        })
    }
}

impl ToTokens for Abi {
    fn to_tokens(&self, t: &mut TokenStream) {
        Extern::default().to_tokens(t);
        if let Some(name) = &self.name {
            token::Literal::string(name).to_tokens(t);
        }
    }
}

// --- Receiver: `self`, `&self`, `&mut self`, `&'a self` ---

#[doc = "A method receiver parameter (`self`, `&self`, `&mut self`)."]
#[derive(Debug, Clone)]
pub struct Receiver {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub reference: bool,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
}

impl Parse for Receiver {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let reference = if stream.peek::<And>().is_some() {
            let _ = stream.parse::<And>()?;
            true
        } else {
            false
        };
        let lifetime = if reference {
            stream.parse_opt::<Lifetime>()
        } else {
            None
        };
        let mutability = stream.parse::<Mutability>()?;
        let _ = stream.parse::<SelfValue>()?;
        Ok(Self {
            span: Span::default(),
            attrs,
            reference,
            lifetime,
            mutability,
        })
    }
}

impl ToTokens for Receiver {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        if self.reference {
            And::default().to_tokens(t);
            if let Some(l) = &self.lifetime {
                l.to_tokens(t);
            }
        }
        self.mutability.to_tokens(t);
        SelfValue::default().to_tokens(t);
    }
}

// --- Variadic: `...` ---

#[doc = "A C-style variadic marker (`...`)."]
#[derive(Debug, Clone)]
pub struct Variadic {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub name: Option<Ident>,
}

impl Parse for Variadic {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let _ = stream.parse::<DotDotDot>()?;
        Ok(Self {
            span: Span::default(),
            attrs,
            name: None,
        })
    }
}

impl ToTokens for Variadic {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        DotDotDot::default().to_tokens(t);
    }
}

// --- BareFnArg: `[name:] ty` ---

#[doc = "An argument of a bare function pointer type."]
#[derive(Debug, Clone)]
pub struct BareFnArg {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub name: Option<Ident>,
    pub ty: Type,
}

impl Parse for BareFnArg {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        // `name: ty` — only when an ident is directly followed by `:`.
        let name = {
            let mut fork = stream.fork();
            if let Ok(id) = fork.parse::<Ident>() {
                if fork.peek::<crate::token::punct::Colon>().is_some() {
                    stream.seek(&fork);
                    let _ = stream.parse::<crate::token::punct::Colon>()?;
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
            name,
            ty,
        })
    }
}

impl ToTokens for BareFnArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        if let Some(n) = &self.name {
            n.to_tokens(t);
            crate::token::punct::Colon::default().to_tokens(t);
        }
        self.ty.to_tokens(t);
    }
}

// --- FnParam: receiver or typed ---

#[doc = "A function parameter (receiver or typed pattern)."]
#[derive(Debug, Clone)]
pub enum FnParam {
    Receiver(Box<Receiver>),
    Typed(Box<TypedParam>),
}

impl Parse for FnParam {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        // `self`/`&self`/`&mut self`/`&'a self` → receiver.
        if is_receiver(stream) {
            return Ok(FnParam::Receiver(Box::new(stream.parse()?)));
        }
        Ok(FnParam::Typed(Box::new(stream.parse()?)))
    }
}

fn is_receiver(stream: &mut ParseStream) -> bool {
    let mut fork = stream.fork();
    let _ = fork.parse_vec::<Attribute>();
    if fork.peek::<SelfValue>().is_some() {
        return true;
    }
    if fork.peek::<And>().is_some() {
        let _ = fork.parse::<And>();
        let _ = fork.parse_opt::<Lifetime>();
        let _ = fork.parse::<Mutability>();
        return fork.peek::<SelfValue>().is_some();
    }
    false
}

impl ToTokens for FnParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            FnParam::Receiver(v) => v.to_tokens(t),
            FnParam::Typed(v) => v.to_tokens(t),
        }
    }
}

// --- Signature ---

#[doc = "A function signature."]
#[derive(Debug, Clone)]
pub struct Signature {
    pub span: Span,
    pub constness: Constness,
    pub asyncness: Asyncness,
    pub unsafety: Unsafety,
    pub abi: Option<Abi>,
    pub ident: Ident,
    pub generics: Generics,
    pub inputs: Punctuated<FnParam, Comma>,
    pub variadic: Option<Variadic>,
    pub output: ReturnType,
}

impl Parse for Signature {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let constness = stream.parse::<Constness>()?;
        let asyncness = stream.parse::<Asyncness>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let abi = if stream.peek::<Extern>().is_some() {
            Some(stream.parse::<Abi>()?)
        } else {
            None
        };
        let _ = stream.parse::<Fn>()?;
        let ident = stream.parse::<Ident>()?;
        let mut generics = stream.parse::<Generics>()?;

        let group = stream.parse_group(Delim::Paren)?;
        let mut inner = group.parse();
        // Parse params, stopping to capture a trailing C-variadic `...`.
        let mut inputs = Punctuated::new();
        let mut variadic = None;
        while !inner.is_empty() {
            if let Some(v) = inner.parse_opt::<Variadic>() {
                variadic = Some(v);
                break;
            }
            inputs.push_value(inner.parse::<FnParam>()?);
            if inner.peek::<Comma>().is_some() {
                inputs.push_punct(inner.parse::<Comma>()?);
            } else {
                break;
            }
        }

        let output = stream.parse::<ReturnType>()?;

        // A trailing `where` clause may follow the return type.
        if stream.peek::<crate::token::keyword::Where>().is_some() {
            generics.where_clause = Some(stream.parse()?);
        }

        Ok(Self {
            span: Span::default(),
            constness,
            asyncness,
            unsafety,
            abi,
            ident,
            generics,
            inputs,
            variadic,
            output,
        })
    }
}

impl ToTokens for Signature {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.constness.to_tokens(t);
        self.asyncness.to_tokens(t);
        self.unsafety.to_tokens(t);
        if let Some(abi) = &self.abi {
            abi.to_tokens(t);
        }
        Fn::default().to_tokens(t);
        self.ident.to_tokens(t);
        // Emit only the angle-params here; the where clause goes after the output.
        let mut params = TokenStream::new();
        emit_angle_params(&self.generics, &mut params);
        t.extend(params);
        let mut inner = TokenStream::new();
        self.inputs.to_tokens(&mut inner);
        if let Some(v) = &self.variadic {
            if !self.inputs.is_empty() && !self.inputs.trailing_punct() {
                Comma::default().to_tokens(&mut inner);
            }
            v.to_tokens(&mut inner);
        }
        t.extend_one(TokenTree::Group(crate::token::Group::new(
            Delim::Paren,
            inner,
        )));
        self.output.to_tokens(t);
        if let Some(w) = &self.generics.where_clause {
            w.to_tokens(t);
        }
    }
}

fn emit_angle_params(generics: &Generics, t: &mut TokenStream) {
    if !generics.params.is_empty() {
        crate::token::punct::Lt::default().to_tokens(t);
        generics.params.to_tokens(t);
        crate::token::punct::Gt::default().to_tokens(t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::ToTokenStream;
    use std::str::FromStr;

    fn parse<T: Parse>(src: &str) -> T {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>().unwrap()
    }

    #[test]
    fn signature_basic() {
        let s: Signature = parse("fn foo(x: u8) -> u8");
        assert_eq!(s.ident.text, "foo");
        assert_eq!(s.inputs.len(), 1);
        assert!(matches!(s.output, ReturnType::Type(_)));
    }

    #[test]
    fn signature_generic_where() {
        let s: Signature = parse("fn f<T>(x: T) where T: Clone");
        assert_eq!(s.generics.params.len(), 1);
        assert!(s.generics.where_clause.is_some());
    }

    #[test]
    fn receiver_param() {
        let s: Signature = parse("fn m(&self, x: u8)");
        assert!(matches!(s.inputs.first().unwrap(), FnParam::Receiver(_)));
    }

    #[test]
    fn bare_fn_type() {
        use crate::ast::Type;
        assert!(matches!(parse::<Type>("fn(u8) -> u8"), Type::BareFn(_)));
        assert_eq!(
            parse::<Type>("fn(u8) -> u8").to_token_stream().to_string(),
            "fn (u8) -> u8"
        );
    }
}
