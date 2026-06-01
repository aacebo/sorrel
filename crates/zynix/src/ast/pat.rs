use crate::ast::{
    Attribute, Expr, Ident, Member, Mutability, Path, Punctuated, QSelf, RangeLimits, Type,
};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Mut, Ref};
use crate::token::punct::{And, At, Colon, Comma, DotDot, Or as OrPunct};
use crate::token::{Delim, Group, LexError, Punctuation, ToTokens};
use crate::{Parse, Span, Token, TokenStream, TokenTree};

macro_rules! pat_structs {
    ($($name:ident { $($field:ident : $ty:ty),* $(,)? })*) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                pub span: Span,
                pub attrs: Vec<Attribute>,
                $(pub $field: $ty,)*
            }
        )*
    };
}

pat_structs! {
    PatIdent { by_ref: bool, mutability: Mutability, ident: Ident, subpat: Option<Box<Pattern>> }
    PatPath { qself: Option<QSelf>, path: Path }
    PatTuple { elems: Punctuated<Pattern, Comma> }
    PatTupleStruct { qself: Option<QSelf>, path: Path, elems: Punctuated<Pattern, Comma> }
    PatStruct { qself: Option<QSelf>, path: Path, fields: Punctuated<PatField, Comma>, rest: bool }
    PatSlice { elems: Punctuated<Pattern, Comma> }
    PatReference { mutability: Mutability, pat: Box<Pattern> }
    PatOr { cases: Punctuated<Pattern, OrPunct> }
    PatLit { expr: Expr }
    PatRange { start: Option<Expr>, limits: RangeLimits, end: Option<Expr> }
    PatType { pat: Box<Pattern>, ty: Box<Type> }
    PatParen { pat: Box<Pattern> }
    PatGroup { pat: Box<Pattern> }
    PatField { member: Member, pat: Pattern, shorthand: bool }
}

fn emit_attrs(attrs: &[Attribute], t: &mut TokenStream) {
    for a in attrs {
        a.to_tokens(t);
    }
}

impl ToTokens for PatIdent {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        if self.by_ref {
            Ref::default().to_tokens(t);
        }
        self.mutability.to_tokens(t);
        self.ident.to_tokens(t);
        if let Some(sub) = &self.subpat {
            At::default().to_tokens(t);
            sub.to_tokens(t);
        }
    }
}
impl ToTokens for PatPath {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.path.to_tokens(t);
    }
}
fn emit_group(delim: Delim, inner: TokenStream, t: &mut TokenStream) {
    t.extend_one(TokenTree::Group(Group::new(delim, inner)));
}
impl ToTokens for PatTuple {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        emit_group(Delim::Paren, inner, t);
    }
}
impl ToTokens for PatTupleStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.path.to_tokens(t);
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        emit_group(Delim::Paren, inner, t);
    }
}
impl ToTokens for PatStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.path.to_tokens(t);
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);
        if self.rest {
            DotDot::default().to_tokens(&mut inner);
        }
        emit_group(Delim::Brace, inner, t);
    }
}
impl ToTokens for PatSlice {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        emit_group(Delim::Bracket, inner, t);
    }
}
impl ToTokens for PatReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        And::default().to_tokens(t);
        self.mutability.to_tokens(t);
        self.pat.to_tokens(t);
    }
}
impl ToTokens for PatOr {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.cases.to_tokens(t);
    }
}
impl ToTokens for PatLit {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.expr.to_tokens(t);
    }
}
impl ToTokens for PatRange {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        if let Some(s) = &self.start {
            s.to_tokens(t);
        }
        self.limits.to_tokens(t);
        if let Some(e) = &self.end {
            e.to_tokens(t);
        }
    }
}
impl ToTokens for PatType {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.pat.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
    }
}
impl ToTokens for PatParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        let mut inner = TokenStream::new();
        self.pat.to_tokens(&mut inner);
        emit_group(Delim::Paren, inner, t);
    }
}
impl ToTokens for PatGroup {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.pat.to_tokens(t);
    }
}
impl ToTokens for PatField {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        if self.shorthand {
            self.pat.to_tokens(t);
        } else {
            self.member.to_tokens(t);
            Colon::default().to_tokens(t);
            self.pat.to_tokens(t);
        }
    }
}

#[doc = "A Rust pattern (in `let`, `match`, function params, etc.)."]
#[derive(Debug, Clone)]
pub enum Pattern {
    Wild,
    Rest,
    Ident(PatIdent),
    Path(PatPath),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Struct(PatStruct),
    Slice(PatSlice),
    Reference(PatReference),
    Or(PatOr),
    Lit(PatLit),
    Range(PatRange),
    Macro(crate::ast::MacroCall),
    Type(PatType),
    Group(PatGroup),
    Paren(PatParen),
    Box(Box<Pattern>),
    Const(crate::ast::Block),
}

macro_rules! impl_from {
    ($($variant:ident => $ty:ty),+ $(,)?) => {
        $(impl From<$ty> for Pattern { fn from(value: $ty) -> Self { Pattern::$variant(value) } })+
    };
}
impl_from! {
    Ident => PatIdent, Path => PatPath, Tuple => PatTuple, TupleStruct => PatTupleStruct,
    Struct => PatStruct, Slice => PatSlice, Reference => PatReference, Or => PatOr,
    Lit => PatLit, Range => PatRange, Type => PatType, Group => PatGroup, Paren => PatParen,
}

impl Parse for Pattern {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        // Optional leading `|`, then one-or-more `|`-separated alternatives.
        let leading = stream.peek::<OrPunct>().is_some();
        if leading {
            let _ = stream.parse::<OrPunct>()?;
        }

        let first = parse_single(stream)?;
        if !leading && stream.peek::<OrPunct>().is_none() {
            return Ok(first);
        }

        let mut cases = Punctuated::new();
        cases.push_value(first);
        while stream.peek::<OrPunct>().is_some() {
            cases.push_punct(stream.parse::<OrPunct>()?);
            cases.push_value(parse_single(stream)?);
        }
        Ok(Pattern::Or(PatOr {
            span: Span::default(),
            attrs: Vec::new(),
            cases,
        }))
    }
}

impl Pattern {
    /// Parse a single pattern alternative (no top-level `|` or-collection).
    /// Used where `|` is a delimiter (closure params), not an or-pattern.
    pub(crate) fn parse_single(stream: &mut ParseStream) -> Result<Pattern, ParseError> {
        parse_single(stream)
    }
}

fn parse_single(stream: &mut ParseStream) -> Result<Pattern, ParseError> {
    {
        let at = stream.span();
        let attrs = stream.parse_vec::<Attribute>()?;

        // Wildcard `_`
        if matches!(stream.curr(), Some(tt) if is_named(tt, "_")) {
            stream.advance();
            return Ok(Pattern::Wild);
        }
        // Rest `..`
        if stream.peek::<DotDot>().is_some() {
            let _ = stream.parse::<DotDot>()?;
            return Ok(Pattern::Rest);
        }
        // `box pat`
        if matches!(stream.curr(), Some(tt) if is_named(tt, "box")) {
            stream.advance();
            return Ok(Pattern::Box(Box::new(parse_single(stream)?)));
        }
        // `const { ... }` block pattern
        if matches!(stream.curr(), Some(tt) if is_named(tt, "const"))
            && matches!(stream.nth(1), Some(crate::TokenTree::Group(g)) if g.delim() == Delim::Brace)
        {
            stream.advance();
            return Ok(Pattern::Const(stream.parse::<crate::ast::Block>()?));
        }
        // Reference `&`/`&mut`
        if stream.peek::<And>().is_some() {
            let _ = stream.parse::<And>()?;
            let mutability = stream.parse::<Mutability>()?;
            let pat = Box::new(Pattern::parse(stream)?);
            return Ok(Pattern::Reference(PatReference {
                span: Span::default(),
                attrs,
                mutability,
                pat,
            }));
        }
        // Tuple/paren `(...)`
        if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Paren)) {
            let group = stream.parse_group(Delim::Paren)?;
            let mut inner = group.parse();
            let elems = Punctuated::parse_terminated(&mut inner)?;
            return Ok(Pattern::Tuple(PatTuple {
                span: Span::default(),
                attrs,
                elems,
            }));
        }
        // Slice `[...]`
        if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Bracket)) {
            let group = stream.parse_group(Delim::Bracket)?;
            let mut inner = group.parse();
            let elems = Punctuated::parse_terminated(&mut inner)?;
            return Ok(Pattern::Slice(PatSlice {
                span: Span::default(),
                attrs,
                elems,
            }));
        }
        // `ref`/`mut`-led binding
        if stream.peek::<Ref>().is_some() || stream.peek::<Mut>().is_some() {
            return Ok(Pattern::Ident(parse_pat_ident(stream, attrs)?));
        }
        // Literal pattern
        if matches!(stream.curr(), Some(tt) if matches!(tt, TokenTree::Token(Token::Literal(_)))) {
            let expr = stream.parse::<Expr>()?;
            return Ok(Pattern::Lit(PatLit {
                span: Span::default(),
                attrs,
                expr,
            }));
        }

        // Path-led: ident binding, path, tuple-struct, or struct pattern.
        if matches!(
            stream.curr(),
            Some(
                TokenTree::Token(Token::Ident(_))
                    | TokenTree::Token(Token::Keyword(_))
                    | TokenTree::Token(Token::Punct(Punctuation::PathSep(_)))
            )
        ) {
            // Single bare ident with no `::`/`(`/`{` → binding.
            let mut fork = stream.fork();
            let path = fork.parse::<Path>()?;

            if matches!(fork.curr(), Some(tt) if is_group(tt, Delim::Paren)) {
                stream.seek(&fork);
                let group = stream.parse_group(Delim::Paren)?;
                let mut inner = group.parse();
                let elems = Punctuated::parse_terminated(&mut inner)?;
                return Ok(Pattern::TupleStruct(PatTupleStruct {
                    span: Span::default(),
                    attrs,
                    qself: None,
                    path,
                    elems,
                }));
            }
            if matches!(fork.curr(), Some(tt) if is_group(tt, Delim::Brace)) {
                stream.seek(&fork);
                let group = stream.parse_group(Delim::Brace)?;
                let mut inner = group.parse();
                let (fields, rest) = parse_struct_pat_body(&mut inner)?;
                return Ok(Pattern::Struct(PatStruct {
                    span: Span::default(),
                    attrs,
                    qself: None,
                    path,
                    fields,
                    rest,
                }));
            }

            // Bare single-segment path with no leading colon → binding ident.
            if !path.leading_colon && path.segments.len() == 1 {
                stream.seek(&fork);
                let ident = match path.segments.into_iter().next() {
                    Some(seg) => seg.ident,
                    None => return Err(LexError::new(at).message("expected pattern").into()),
                };
                return Ok(Pattern::Ident(PatIdent {
                    span: Span::default(),
                    attrs,
                    by_ref: false,
                    mutability: Mutability::Immutable,
                    ident,
                    subpat: None,
                }));
            }

            stream.seek(&fork);
            return Ok(Pattern::Path(PatPath {
                span: Span::default(),
                attrs,
                qself: None,
                path,
            }));
        }

        Err(LexError::new(at).message("expected pattern").into())
    }
}

fn parse_pat_ident(
    stream: &mut ParseStream,
    attrs: Vec<Attribute>,
) -> Result<PatIdent, ParseError> {
    let by_ref = if stream.peek::<Ref>().is_some() {
        let _ = stream.parse::<Ref>()?;
        true
    } else {
        false
    };
    let mutability = stream.parse::<Mutability>()?;
    let ident = stream.parse::<Ident>()?;
    let subpat = if stream.peek::<At>().is_some() {
        let _ = stream.parse::<At>()?;
        Some(Box::new(Pattern::parse(stream)?))
    } else {
        None
    };
    Ok(PatIdent {
        span: Span::default(),
        attrs,
        by_ref,
        mutability,
        ident,
        subpat,
    })
}

fn parse_struct_pat_body(
    stream: &mut ParseStream,
) -> Result<(Punctuated<PatField, Comma>, bool), ParseError> {
    let mut fields = Punctuated::new();
    let mut rest = false;

    while !stream.is_empty() {
        if stream.peek::<DotDot>().is_some() {
            let _ = stream.parse::<DotDot>()?;
            rest = true;
            break;
        }
        let member = stream.parse::<Member>()?;
        let (pat, shorthand) = if stream.peek::<Colon>().is_some() {
            let _ = stream.parse::<Colon>()?;
            (stream.parse::<Pattern>()?, false)
        } else {
            // shorthand `{ field }`
            let ident = match &member {
                Member::Named(id) => id.clone(),
                Member::Unnamed(_) => {
                    return Err(LexError::new(stream.span())
                        .message("tuple index needs a pattern")
                        .into());
                }
            };
            (
                Pattern::Ident(PatIdent {
                    span: Span::default(),
                    attrs: Vec::new(),
                    by_ref: false,
                    mutability: Mutability::Immutable,
                    ident,
                    subpat: None,
                }),
                true,
            )
        };
        fields.push_value(PatField {
            span: Span::default(),
            attrs: Vec::new(),
            member,
            pat,
            shorthand,
        });
        if stream.peek::<Comma>().is_some() {
            fields.push_punct(stream.parse::<Comma>()?);
        } else {
            break;
        }
    }

    Ok((fields, rest))
}

impl ToTokens for Pattern {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Pattern::Wild => {
                crate::token::Ident::new("_", Span::default()).to_tokens(t);
            }
            Pattern::Rest => DotDot::default().to_tokens(t),
            Pattern::Ident(v) => v.to_tokens(t),
            Pattern::Path(v) => v.to_tokens(t),
            Pattern::Tuple(v) => v.to_tokens(t),
            Pattern::TupleStruct(v) => v.to_tokens(t),
            Pattern::Struct(v) => v.to_tokens(t),
            Pattern::Slice(v) => v.to_tokens(t),
            Pattern::Reference(v) => v.to_tokens(t),
            Pattern::Or(v) => v.to_tokens(t),
            Pattern::Lit(v) => v.to_tokens(t),
            Pattern::Range(v) => v.to_tokens(t),
            Pattern::Macro(v) => v.to_tokens(t),
            Pattern::Type(v) => v.to_tokens(t),
            Pattern::Group(v) => v.to_tokens(t),
            Pattern::Paren(v) => v.to_tokens(t),
            Pattern::Box(p) => {
                crate::token::keyword::Box::default().to_tokens(t);
                p.to_tokens(t);
            }
            Pattern::Const(b) => {
                crate::token::keyword::Const::default().to_tokens(t);
                b.to_tokens(t);
            }
        }
    }
}

fn is_named(tt: &TokenTree, name: &str) -> bool {
    match tt {
        TokenTree::Token(Token::Ident(id)) => id.name() == name,
        TokenTree::Token(Token::Keyword(kw)) => kw.as_str() == name,
        _ => false,
    }
}

fn is_group(tt: &TokenTree, delim: Delim) -> bool {
    matches!(tt, TokenTree::Group(g) if g.delim() == delim)
}
