use crate::ast::{Attribute, Expr, Ident, Member, Mutability, Path, Punctuated, QSelf, RangeLimits, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Mut, Ref};
use crate::token::punct::{And, At, Colon, Comma, DotDot, Or as OrPunct};
use crate::token::{Delim, Group, LexError, Punctuation, ToTokens};
use crate::{Parse, Span, Token, TokenStream, TokenTree};

mod pat_field;
mod pat_group;
mod pat_ident;
mod pat_lit;
mod pat_or;
mod pat_paren;
mod pat_path;
mod pat_range;
mod pat_reference;
mod pat_slice;
mod pat_struct;
mod pat_tuple;
mod pat_tuple_struct;
mod pat_type;

pub use pat_field::*;
pub use pat_group::*;
pub use pat_ident::*;
pub use pat_lit::*;
pub use pat_or::*;
pub use pat_paren::*;
pub use pat_path::*;
pub use pat_range::*;
pub use pat_reference::*;
pub use pat_slice::*;
pub use pat_struct::*;
pub use pat_tuple::*;
pub use pat_tuple_struct::*;
pub use pat_type::*;

#[doc = "A Rust pattern (in `let`, `match`, function params, etc.)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
    Const(crate::ast::StmtBlock),
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

pub(super) fn emit_attrs(attrs: &[Attribute], t: &mut TokenStream) {
    for a in attrs {
        a.to_tokens(t);
    }
}

pub(super) fn emit_group(delim: Delim, inner: TokenStream, t: &mut TokenStream) {
    t.extend_one(TokenTree::Group(Group::new(delim, inner)));
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
            return Ok(Pattern::Const(stream.parse::<crate::ast::StmtBlock>()?));
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

fn parse_pat_ident(stream: &mut ParseStream, attrs: Vec<Attribute>) -> Result<PatIdent, ParseError> {
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

fn parse_struct_pat_body(stream: &mut ParseStream) -> Result<(Punctuated<PatField, Comma>, bool), ParseError> {
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
                    return Err(LexError::new(stream.span()).message("tuple index needs a pattern").into());
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::token::ToTokenStream;

    fn parse_pat(src: &str) -> Result<Pattern, ParseError> {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<Pattern>()
    }

    fn pat(src: &str) -> Pattern {
        parse_pat(src).unwrap()
    }

    fn roundtrip(src: &str) -> String {
        pat(src).to_token_stream().to_string()
    }

    #[test]
    fn wildcard() {
        assert!(matches!(pat("_"), Pattern::Wild));
        assert_eq!(roundtrip("_"), "_");
    }

    #[test]
    fn rest() {
        assert!(matches!(pat(".."), Pattern::Rest));
        assert_eq!(roundtrip(".."), "..");
    }

    #[test]
    fn ident_binding() {
        assert!(matches!(pat("x"), Pattern::Ident(_)));
        assert_eq!(roundtrip("x"), "x");
    }

    #[test]
    fn mut_binding() {
        assert!(matches!(pat("mut x"), Pattern::Ident(_)));
    }

    #[test]
    fn ref_binding() {
        assert!(matches!(pat("ref x"), Pattern::Ident(_)));
    }

    #[test]
    fn tuple_pattern() {
        assert!(matches!(pat("(a, b)"), Pattern::Tuple(_)));
    }

    #[test]
    fn slice_pattern() {
        assert!(matches!(pat("[a, b]"), Pattern::Slice(_)));
    }

    #[test]
    fn reference_pattern() {
        assert!(matches!(pat("&x"), Pattern::Reference(_)));
    }

    #[test]
    fn lit_pattern() {
        assert!(matches!(pat("42"), Pattern::Lit(_)));
    }

    #[test]
    fn or_pattern() {
        assert!(matches!(pat("A | B"), Pattern::Or(_)));
    }
}
