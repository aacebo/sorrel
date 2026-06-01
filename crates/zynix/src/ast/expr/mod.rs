use crate::ast::precedence::Precedence;
use crate::ast::{
    AngleArgs, AssignOp, Asyncness, Attribute, BinOp, BoundLifetimes, ClosureParam, Constness, FieldValue, Ident, Label, Lit,
    MacroCall, Member, Movability, Mutability, Path, Pattern, Punctuated, QSelf, RangeLimits, ReturnType, StmtBlock, Type, UnOp,
};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{
    Await as KwAwait, Break, Const, Continue, For, If, In, Let, Loop, Match, Move, Return, Unsafe, While, Yield,
};
use crate::token::punct::{And, Comma, Dot, DotDot, Eq, Not, Or, OrOr, Question, Semi, Star};
use crate::token::{Delim, Group, LexError, Punctuation, ToTokens};
use crate::{Parse, Span, Token, TokenStream, TokenTree};

mod expr_array;
mod expr_assign;
mod expr_assign_op;
mod expr_async;
mod expr_await;
mod expr_binary;
mod expr_block;
mod expr_break;
mod expr_call;
mod expr_cast;
mod expr_closure;
mod expr_const;
mod expr_continue;
mod expr_field;
mod expr_for_loop;
mod expr_group;
mod expr_if;
mod expr_index;
mod expr_infer;
mod expr_let;
mod expr_lit;
mod expr_loop;
mod expr_macro;
mod expr_match;
mod expr_method_call;
mod expr_paren;
mod expr_path;
mod expr_range;
mod expr_reference;
mod expr_repeat;
mod expr_return;
mod expr_struct;
mod expr_try;
mod expr_try_block;
mod expr_tuple;
mod expr_type;
mod expr_unary;
mod expr_unsafe;
mod expr_while;
mod expr_yield;

pub use expr_array::*;
pub use expr_assign::*;
pub use expr_assign_op::*;
pub use expr_async::*;
pub use expr_await::*;
pub use expr_binary::*;
pub use expr_block::*;
pub use expr_break::*;
pub use expr_call::*;
pub use expr_cast::*;
pub use expr_closure::*;
pub use expr_const::*;
pub use expr_continue::*;
pub use expr_field::*;
pub use expr_for_loop::*;
pub use expr_group::*;
pub use expr_if::*;
pub use expr_index::*;
pub use expr_let::*;
pub use expr_lit::*;
pub use expr_loop::*;
pub use expr_match::*;
pub use expr_method_call::*;
pub use expr_paren::*;
pub use expr_path::*;
pub use expr_range::*;
pub use expr_reference::*;
pub use expr_repeat::*;
pub use expr_return::*;
pub use expr_struct::*;
pub use expr_try::*;
pub use expr_try_block::*;
pub use expr_tuple::*;
pub use expr_type::*;
pub use expr_unary::*;
pub use expr_unsafe::*;
pub use expr_while::*;
pub use expr_yield::*;

#[doc = "A Rust expression. The primary recursive node covering all expression forms."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Expr {
    Lit(ExprLit),
    Path(ExprPath),
    Block(ExprBlock),
    Unsafe(ExprUnsafe),
    Const(ExprConst),
    If(ExprIf),
    While(ExprWhile),
    ForLoop(ExprForLoop),
    Loop(ExprLoop),
    Match(ExprMatch),
    Closure(ExprClosure),
    Async(ExprAsync),
    Await(ExprAwait),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Yield(ExprYield),
    Return(ExprReturn),
    Break(ExprBreak),
    Continue(ExprContinue),
    Call(ExprCall),
    MethodCall(ExprMethodCall),
    Field(ExprField),
    Index(ExprIndex),
    Reference(ExprReference),
    Unary(ExprUnary),
    Binary(ExprBinary),
    Assign(ExprAssign),
    AssignOp(ExprAssignOp),
    Cast(ExprCast),
    Type(ExprType),
    Let(ExprLet),
    Struct(ExprStruct),
    Tuple(ExprTuple),
    Array(ExprArray),
    Repeat(ExprRepeat),
    Range(ExprRange),
    Macro(MacroCall),
    Group(ExprGroup),
    Paren(ExprParen),
    Infer,
    Verbatim(TokenStream),
}

pub(super) fn emit_attrs(attrs: &[Attribute], tokens: &mut TokenStream) {
    for a in attrs {
        a.to_tokens(tokens);
    }
}

pub(super) fn emit_group(expr: &Expr, tokens: &mut TokenStream) {
    expr.to_tokens(tokens);
}

macro_rules! impl_from {
    ($($variant:ident => $ty:ty),+ $(,)?) => {
        $(impl From<$ty> for Expr { fn from(value: $ty) -> Self { Expr::$variant(value) } })+
    };
}

impl_from! {
    Lit => ExprLit, Path => ExprPath, Block => ExprBlock, Unsafe => ExprUnsafe,
    Const => ExprConst, If => ExprIf, While => ExprWhile, ForLoop => ExprForLoop,
    Loop => ExprLoop, Match => ExprMatch, Closure => ExprClosure, Async => ExprAsync,
    Await => ExprAwait, Try => ExprTry, TryBlock => ExprTryBlock, Yield => ExprYield,
    Return => ExprReturn, Break => ExprBreak, Continue => ExprContinue, Call => ExprCall,
    MethodCall => ExprMethodCall, Field => ExprField, Index => ExprIndex,
    Reference => ExprReference, Unary => ExprUnary, Binary => ExprBinary,
    Assign => ExprAssign, AssignOp => ExprAssignOp, Cast => ExprCast, Type => ExprType,
    Let => ExprLet, Struct => ExprStruct, Tuple => ExprTuple, Array => ExprArray,
    Repeat => ExprRepeat, Range => ExprRange, Macro => MacroCall, Group => ExprGroup,
    Paren => ExprParen,
}

impl Parse for Expr {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        parse_expr(stream, true)
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Expr::Lit(v) => v.to_tokens(t),
            Expr::Path(v) => v.to_tokens(t),
            Expr::Block(v) => v.to_tokens(t),
            Expr::Unsafe(v) => v.to_tokens(t),
            Expr::Const(v) => v.to_tokens(t),
            Expr::If(v) => v.to_tokens(t),
            Expr::While(v) => v.to_tokens(t),
            Expr::ForLoop(v) => v.to_tokens(t),
            Expr::Loop(v) => v.to_tokens(t),
            Expr::Match(v) => v.to_tokens(t),
            Expr::Closure(v) => v.to_tokens(t),
            Expr::Async(v) => v.to_tokens(t),
            Expr::Await(v) => v.to_tokens(t),
            Expr::Try(v) => v.to_tokens(t),
            Expr::TryBlock(v) => v.to_tokens(t),
            Expr::Yield(v) => v.to_tokens(t),
            Expr::Return(v) => v.to_tokens(t),
            Expr::Break(v) => v.to_tokens(t),
            Expr::Continue(v) => v.to_tokens(t),
            Expr::Call(v) => v.to_tokens(t),
            Expr::MethodCall(v) => v.to_tokens(t),
            Expr::Field(v) => v.to_tokens(t),
            Expr::Index(v) => v.to_tokens(t),
            Expr::Reference(v) => v.to_tokens(t),
            Expr::Unary(v) => v.to_tokens(t),
            Expr::Binary(v) => v.to_tokens(t),
            Expr::Assign(v) => v.to_tokens(t),
            Expr::AssignOp(v) => v.to_tokens(t),
            Expr::Cast(v) => v.to_tokens(t),
            Expr::Type(v) => v.to_tokens(t),
            Expr::Let(v) => v.to_tokens(t),
            Expr::Struct(v) => v.to_tokens(t),
            Expr::Tuple(v) => v.to_tokens(t),
            Expr::Array(v) => v.to_tokens(t),
            Expr::Repeat(v) => v.to_tokens(t),
            Expr::Range(v) => v.to_tokens(t),
            Expr::Macro(v) => v.to_tokens(t),
            Expr::Group(v) => v.to_tokens(t),
            Expr::Paren(v) => v.to_tokens(t),
            Expr::Infer => {}
            Expr::Verbatim(v) => v.to_tokens(t),
        }
    }
}

// ===========================================================================
// Parser
// ===========================================================================

fn no_attrs() -> Vec<Attribute> {
    Vec::new()
}

pub(crate) fn parse_expr(stream: &mut ParseStream, allow_struct: bool) -> Result<Expr, ParseError> {
    let lhs = parse_unary(stream, allow_struct)?;
    parse_binary(stream, lhs, Precedence::Min, allow_struct)
}

fn parse_binary(stream: &mut ParseStream, mut lhs: Expr, min: Precedence, allow_struct: bool) -> Result<Expr, ParseError> {
    loop {
        if Precedence::Cast >= min && stream.peek::<crate::token::keyword::As>().is_some() {
            let _ = stream.parse::<crate::token::keyword::As>()?;
            let ty = Box::new(stream.parse::<Type>()?);
            lhs = Expr::Cast(ExprCast {
                span: Span::default(),
                attrs: no_attrs(),
                expr: Box::new(lhs),
                ty,
            });
            continue;
        }

        if min == Precedence::Min {
            if stream.peek::<Eq>().is_some() {
                let _ = stream.parse::<Eq>()?;
                let right = Box::new(parse_expr(stream, allow_struct)?);
                lhs = Expr::Assign(ExprAssign {
                    span: Span::default(),
                    attrs: no_attrs(),
                    left: Box::new(lhs),
                    right,
                });
                continue;
            }
            if let Some(op) = stream.peek::<AssignOp>() {
                let _ = stream.parse::<AssignOp>()?;
                let right = Box::new(parse_expr(stream, allow_struct)?);
                lhs = Expr::AssignOp(ExprAssignOp {
                    span: Span::default(),
                    attrs: no_attrs(),
                    left: Box::new(lhs),
                    op,
                    right,
                });
                continue;
            }
        }

        // Range with a left operand: `a..b`, `a..=b`, `a..` (Precedence::Range).
        if Precedence::Range >= min
            && (stream.peek::<DotDot>().is_some() || stream.peek::<crate::token::punct::DotDotEq>().is_some())
        {
            let limits = stream.parse::<RangeLimits>()?;
            let end = maybe_range_end(stream, allow_struct)?;
            lhs = Expr::Range(ExprRange {
                span: Span::default(),
                attrs: no_attrs(),
                start: Some(Box::new(lhs)),
                limits,
                end,
            });
            continue;
        }

        match stream.peek::<BinOp>() {
            Some(op) if Precedence::of(&op) >= min => {
                let prec = Precedence::of(&op);
                let _ = stream.parse::<BinOp>()?;
                let mut rhs = parse_unary(stream, allow_struct)?;
                rhs = parse_binary(stream, rhs, next(prec), allow_struct)?;
                lhs = Expr::Binary(ExprBinary {
                    span: Span::default(),
                    attrs: no_attrs(),
                    left: Box::new(lhs),
                    op,
                    right: Box::new(rhs),
                });
            }
            _ => break,
        }
    }

    Ok(lhs)
}

fn next(p: Precedence) -> Precedence {
    use Precedence::*;
    match p {
        Min => Range,
        Range => Or,
        Or => And,
        And => Compare,
        Compare => BitOr,
        BitOr => BitXor,
        BitXor => BitAnd,
        BitAnd => Shift,
        Shift => Add,
        Add => Mul,
        Mul => Cast,
        Cast => Cast,
    }
}

fn parse_unary(stream: &mut ParseStream, allow_struct: bool) -> Result<Expr, ParseError> {
    // Prefix range: `..b`, `..=b`, `..`.
    if stream.peek::<DotDot>().is_some() || stream.peek::<crate::token::punct::DotDotEq>().is_some() {
        let limits = stream.parse::<RangeLimits>()?;
        let end = maybe_range_end(stream, allow_struct)?;
        return Ok(Expr::Range(ExprRange {
            span: Span::default(),
            attrs: no_attrs(),
            start: None,
            limits,
            end,
        }));
    }

    if stream.peek::<And>().is_some() {
        let _ = stream.parse::<And>()?;
        let mutability = stream.parse::<Mutability>()?;
        let expr = Box::new(parse_unary(stream, allow_struct)?);
        return Ok(Expr::Reference(ExprReference {
            span: Span::default(),
            attrs: no_attrs(),
            mutability,
            expr,
        }));
    }

    if prefix_un_op(stream) {
        let op = stream.parse::<UnOp>()?;
        let expr = Box::new(parse_unary(stream, allow_struct)?);
        return Ok(Expr::Unary(ExprUnary {
            span: Span::default(),
            attrs: no_attrs(),
            op,
            expr,
        }));
    }

    let atom = parse_primary(stream, allow_struct)?;
    parse_postfix(stream, atom)
}

fn prefix_un_op(stream: &mut ParseStream) -> bool {
    stream.peek::<Not>().is_some() || stream.peek::<crate::token::punct::Minus>().is_some() || stream.peek::<Star>().is_some()
}

/// Parse an optional turbofish `::<...>` (method-call generic args).
fn parse_turbofish(stream: &mut ParseStream) -> Result<Option<AngleArgs>, ParseError> {
    let mut fork = stream.fork();
    if fork.peek::<crate::token::punct::PathSep>().is_none() {
        return Ok(None);
    }
    let _ = fork.parse::<crate::token::punct::PathSep>()?;
    if fork.peek::<crate::token::punct::Lt>().is_none() {
        return Ok(None);
    }
    let args = fork.parse::<AngleArgs>()?;
    stream.seek(&fork);
    Ok(Some(args))
}

fn parse_postfix(stream: &mut ParseStream, mut expr: Expr) -> Result<Expr, ParseError> {
    loop {
        if stream.peek::<Dot>().is_some() {
            let _ = stream.parse::<Dot>()?;

            if matches!(stream.curr(), Some(tt) if is_named(tt, "await")) {
                stream.advance();
                expr = Expr::Await(ExprAwait {
                    span: Span::default(),
                    attrs: no_attrs(),
                    base: Box::new(expr),
                });
                continue;
            }

            let member = stream.parse::<Member>()?;
            if let Member::Named(method) = &member {
                // Optional turbofish `::<...>` before the call parens.
                let turbofish = parse_turbofish(stream)?;
                if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Paren)) {
                    let method = method.clone();
                    let group = stream.parse_group(Delim::Paren)?;
                    let mut inner = group.parse();
                    let args = Punctuated::parse_terminated(&mut inner)?;
                    expr = Expr::MethodCall(ExprMethodCall {
                        span: Span::default(),
                        attrs: no_attrs(),
                        receiver: Box::new(expr),
                        method,
                        turbofish,
                        args,
                    });
                    continue;
                }
            }

            expr = Expr::Field(ExprField {
                span: Span::default(),
                attrs: no_attrs(),
                base: Box::new(expr),
                member,
            });
            continue;
        }

        if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Paren)) {
            let group = stream.parse_group(Delim::Paren)?;
            let mut inner = group.parse();
            let args = Punctuated::parse_terminated(&mut inner)?;
            expr = Expr::Call(ExprCall {
                span: Span::default(),
                attrs: no_attrs(),
                func: Box::new(expr),
                args,
            });
            continue;
        }

        if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Bracket)) {
            let group = stream.parse_group(Delim::Bracket)?;
            let mut inner = group.parse();
            let index = Box::new(parse_expr(&mut inner, true)?);
            expr = Expr::Index(ExprIndex {
                span: Span::default(),
                attrs: no_attrs(),
                base: Box::new(expr),
                index,
            });
            continue;
        }

        if stream.peek::<Question>().is_some() {
            let _ = stream.parse::<Question>()?;
            expr = Expr::Try(ExprTry {
                span: Span::default(),
                attrs: no_attrs(),
                expr: Box::new(expr),
            });
            continue;
        }

        break;
    }

    Ok(expr)
}

fn parse_primary(stream: &mut ParseStream, allow_struct: bool) -> Result<Expr, ParseError> {
    let at = stream.span();

    if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Paren)) {
        let group = stream.parse_group(Delim::Paren)?;
        let mut inner = group.parse();
        let elems: Punctuated<Expr, Comma> = Punctuated::parse_terminated(&mut inner)?;
        return Ok(if elems.len() == 1 && !elems.trailing_punct() {
            Expr::Paren(ExprParen {
                span: Span::default(),
                attrs: no_attrs(),
                expr: Box::new(elems.into_iter().next().unwrap()),
            })
        } else {
            Expr::Tuple(ExprTuple {
                span: Span::default(),
                attrs: no_attrs(),
                elems,
            })
        });
    }

    if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Bracket)) {
        let group = stream.parse_group(Delim::Bracket)?;
        let mut inner = group.parse();
        if let Some(rep) = try_repeat(&mut inner)? {
            return Ok(Expr::Repeat(rep));
        }
        let elems = Punctuated::parse_terminated(&mut inner)?;
        return Ok(Expr::Array(ExprArray {
            span: Span::default(),
            attrs: no_attrs(),
            elems,
        }));
    }

    // Labeled block / loop: `'a: { }`, `'a: loop { }`, etc.
    if is_label_start(stream) {
        let label = Some(stream.parse::<Label>()?);
        if stream.peek::<While>().is_some() {
            return Ok(Expr::While(parse_while(stream, label)?));
        }
        if stream.peek::<For>().is_some() {
            return Ok(Expr::ForLoop(parse_for(stream, label)?));
        }
        if stream.peek::<Loop>().is_some() {
            return Ok(Expr::Loop(parse_loop(stream, label)?));
        }
        return Ok(Expr::Block(ExprBlock {
            span: Span::default(),
            attrs: no_attrs(),
            label,
            block: stream.parse()?,
        }));
    }

    if matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Brace)) {
        return Ok(Expr::Block(ExprBlock {
            span: Span::default(),
            attrs: no_attrs(),
            label: None,
            block: stream.parse()?,
        }));
    }

    if stream.peek::<If>().is_some() {
        return parse_if(stream);
    }
    if stream.peek::<While>().is_some() {
        return Ok(Expr::While(parse_while(stream, None)?));
    }
    if stream.peek::<For>().is_some() {
        return Ok(Expr::ForLoop(parse_for(stream, None)?));
    }
    if stream.peek::<Loop>().is_some() {
        return Ok(Expr::Loop(parse_loop(stream, None)?));
    }
    if stream.peek::<Match>().is_some() {
        return parse_match(stream);
    }
    if stream.peek::<Unsafe>().is_some() {
        let _ = stream.parse::<Unsafe>()?;
        return Ok(Expr::Unsafe(ExprUnsafe {
            span: Span::default(),
            attrs: no_attrs(),
            block: stream.parse()?,
        }));
    }
    // `const { }` block (vs `const` closure, which has `|`/`move` next).
    if stream.peek::<Const>().is_some() && peek2_is_brace(stream) {
        let _ = stream.parse::<Const>()?;
        return Ok(Expr::Const(ExprConst {
            span: Span::default(),
            attrs: no_attrs(),
            block: stream.parse()?,
        }));
    }
    // `async { }` / `async move { }` block (vs `async` closure).
    if stream.peek::<crate::token::keyword::Async>().is_some() && async_is_block(stream) {
        let _ = stream.parse::<crate::token::keyword::Async>()?;
        let capture = if stream.peek::<Move>().is_some() {
            let _ = stream.parse::<Move>()?;
            true
        } else {
            false
        };
        return Ok(Expr::Async(ExprAsync {
            span: Span::default(),
            attrs: no_attrs(),
            capture,
            block: stream.parse()?,
        }));
    }
    // `try { }` block.
    if matches!(stream.curr(), Some(tt) if is_named(tt, "try")) && peek2_is_brace(stream) {
        stream.advance();
        return Ok(Expr::TryBlock(ExprTryBlock {
            span: Span::default(),
            attrs: no_attrs(),
            block: stream.parse()?,
        }));
    }
    if stream.peek::<Return>().is_some() {
        let _ = stream.parse::<Return>()?;
        return Ok(Expr::Return(ExprReturn {
            span: Span::default(),
            attrs: no_attrs(),
            expr: parse_opt_expr(stream)?,
        }));
    }
    if stream.peek::<Yield>().is_some() {
        let _ = stream.parse::<Yield>()?;
        return Ok(Expr::Yield(ExprYield {
            span: Span::default(),
            attrs: no_attrs(),
            expr: parse_opt_expr(stream)?,
        }));
    }
    if stream.peek::<Break>().is_some() {
        let _ = stream.parse::<Break>()?;
        let label = parse_opt_break_label(stream);
        return Ok(Expr::Break(ExprBreak {
            span: Span::default(),
            attrs: no_attrs(),
            label,
            expr: parse_opt_expr(stream)?,
        }));
    }
    if stream.peek::<Continue>().is_some() {
        let _ = stream.parse::<Continue>()?;
        let label = parse_opt_break_label(stream);
        return Ok(Expr::Continue(ExprContinue {
            span: Span::default(),
            attrs: no_attrs(),
            label,
        }));
    }
    if stream.peek::<Let>().is_some() {
        let _ = stream.parse::<Let>()?;
        let pat = Box::new(stream.parse::<Pattern>()?);
        let _ = stream.parse::<Eq>()?;
        let expr = Box::new(parse_expr(stream, false)?);
        return Ok(Expr::Let(ExprLet {
            span: Span::default(),
            attrs: no_attrs(),
            pat,
            expr,
        }));
    }
    if is_closure_start(stream) {
        return Ok(Expr::Closure(parse_closure(stream)?));
    }

    if matches!(stream.curr(), Some(tt) if is_literal(tt)) || is_bool_ident(stream) {
        return Ok(Expr::Lit(ExprLit {
            span: Span::default(),
            attrs: no_attrs(),
            lit: stream.parse()?,
        }));
    }

    if let Some(mac) = stream.parse_opt::<MacroCall>() {
        return Ok(Expr::Macro(mac));
    }

    // Qualified path `<T as Trait>::assoc` in expression position.
    if stream.peek::<crate::token::punct::Lt>().is_some() {
        let (qself, path) = crate::ast::ty::parse_qualified_path(stream)?;
        return Ok(Expr::Path(ExprPath {
            span: Span::default(),
            attrs: no_attrs(),
            qself: Some(qself),
            path,
        }));
    }

    if matches!(
        stream.curr(),
        Some(
            TokenTree::Token(Token::Ident(_))
                | TokenTree::Token(Token::Keyword(_))
                | TokenTree::Token(Token::Punct(Punctuation::PathSep(_)))
        )
    ) {
        let path = stream.parse::<Path>()?;
        if allow_struct && matches!(stream.curr(), Some(tt) if is_group(tt, Delim::Brace)) {
            let group = stream.parse_group(Delim::Brace)?;
            let mut inner = group.parse();
            let (fields, rest) = parse_struct_body(&mut inner)?;
            return Ok(Expr::Struct(ExprStruct {
                span: Span::default(),
                attrs: no_attrs(),
                qself: None,
                path,
                fields,
                rest,
            }));
        }
        return Ok(Expr::Path(ExprPath {
            span: Span::default(),
            attrs: no_attrs(),
            qself: None,
            path,
        }));
    }

    Err(LexError::new(at).message("expected expression").into())
}

fn parse_opt_expr(stream: &mut ParseStream) -> Result<Option<Box<Expr>>, ParseError> {
    if stream.is_empty() || stream.peek::<Semi>().is_some() || stream.peek::<Comma>().is_some() {
        return Ok(None);
    }
    let mut fork = stream.fork();
    match parse_expr(&mut fork, true) {
        Ok(e) => {
            stream.seek(&fork);
            Ok(Some(Box::new(e)))
        }
        Err(_) => Ok(None),
    }
}

fn try_repeat(stream: &mut ParseStream) -> Result<Option<ExprRepeat>, ParseError> {
    let mut fork = stream.fork();
    let Ok(elem) = parse_expr(&mut fork, true) else {
        return Ok(None);
    };
    if fork.peek::<Semi>().is_none() {
        return Ok(None);
    }
    let _ = fork.parse::<Semi>()?;
    let len = parse_expr(&mut fork, true)?;
    stream.seek(&fork);
    Ok(Some(ExprRepeat {
        span: Span::default(),
        attrs: no_attrs(),
        elem: Box::new(elem),
        len: Box::new(len),
    }))
}

fn parse_if(stream: &mut ParseStream) -> Result<Expr, ParseError> {
    let _ = stream.parse::<If>()?;
    let cond = Box::new(parse_expr(stream, false)?);
    let then_branch = stream.parse::<StmtBlock>()?;
    let else_branch = if matches!(stream.curr(), Some(tt) if is_named(tt, "else")) {
        stream.advance();
        Some(Box::new(parse_primary(stream, true)?))
    } else {
        None
    };
    Ok(Expr::If(ExprIf {
        span: Span::default(),
        attrs: no_attrs(),
        cond,
        then_branch,
        else_branch,
    }))
}

fn parse_while(stream: &mut ParseStream, label: Option<Label>) -> Result<ExprWhile, ParseError> {
    let _ = stream.parse::<While>()?;
    let cond = Box::new(parse_expr(stream, false)?);
    let body = stream.parse::<StmtBlock>()?;
    Ok(ExprWhile {
        span: Span::default(),
        attrs: no_attrs(),
        label,
        cond,
        body,
    })
}

fn parse_for(stream: &mut ParseStream, label: Option<Label>) -> Result<ExprForLoop, ParseError> {
    let _ = stream.parse::<For>()?;
    let pat = Box::new(stream.parse::<Pattern>()?);
    let _ = stream.parse::<In>()?;
    let expr = Box::new(parse_expr(stream, false)?);
    let body = stream.parse::<StmtBlock>()?;
    Ok(ExprForLoop {
        span: Span::default(),
        attrs: no_attrs(),
        label,
        pat,
        expr,
        body,
    })
}

fn parse_loop(stream: &mut ParseStream, label: Option<Label>) -> Result<ExprLoop, ParseError> {
    let _ = stream.parse::<Loop>()?;
    let body = stream.parse::<StmtBlock>()?;
    Ok(ExprLoop {
        span: Span::default(),
        attrs: no_attrs(),
        label,
        body,
    })
}

fn parse_match(stream: &mut ParseStream) -> Result<Expr, ParseError> {
    let _ = stream.parse::<Match>()?;
    let expr = Box::new(parse_expr(stream, false)?);
    let group = stream.parse_group(Delim::Brace)?;
    let mut inner = group.parse();
    let arms = inner.parse_vec::<crate::ast::MatchArm>()?;
    Ok(Expr::Match(ExprMatch {
        span: Span::default(),
        attrs: no_attrs(),
        expr,
        arms,
    }))
}

type StructBody = (Punctuated<FieldValue, Comma>, Option<Box<Expr>>);

fn parse_struct_body(stream: &mut ParseStream) -> Result<StructBody, ParseError> {
    let mut fields = Punctuated::new();
    let mut rest = None;

    while !stream.is_empty() {
        if stream.peek::<DotDot>().is_some() {
            let _ = stream.parse::<DotDot>()?;
            rest = Some(Box::new(parse_expr(stream, true)?));
            break;
        }
        fields.push_value(stream.parse::<FieldValue>()?);
        if stream.peek::<Comma>().is_some() {
            fields.push_punct(stream.parse::<Comma>()?);
        } else {
            break;
        }
    }

    Ok((fields, rest))
}

fn is_closure_start(stream: &mut ParseStream) -> bool {
    // `|...|`, `||`, `move`, or a `const`/`async` immediately followed by a
    // closure start (not a `{` block).
    if stream.peek::<Or>().is_some() || stream.peek::<OrOr>().is_some() || stream.peek::<Move>().is_some() {
        return true;
    }
    let leads_closure = matches!(
        stream.nth(1),
        Some(TokenTree::Token(Token::Punct(Punctuation::Or(_) | Punctuation::OrOr(_))))
            | Some(TokenTree::Token(Token::Keyword(_)))
    );
    (stream.peek::<Const>().is_some() || stream.peek::<crate::token::keyword::Async>().is_some())
        && leads_closure
        && !peek2_is_brace(stream)
}

fn parse_closure(stream: &mut ParseStream) -> Result<ExprClosure, ParseError> {
    let constness = stream.parse::<Constness>()?;
    let asyncness = stream.parse::<Asyncness>()?;
    let capture = if stream.peek::<Move>().is_some() {
        let _ = stream.parse::<Move>()?;
        true
    } else {
        false
    };

    let inputs = if stream.peek::<OrOr>().is_some() {
        let _ = stream.parse::<OrOr>()?;
        Punctuated::new()
    } else {
        let _ = stream.parse::<Or>()?;
        let mut params = Punctuated::new();
        while stream.peek::<Or>().is_none() && !stream.is_empty() {
            params.push_value(stream.parse::<ClosureParam>()?);
            if stream.peek::<Comma>().is_some() {
                params.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }
        let _ = stream.parse::<Or>()?;
        params
    };

    let output = stream.parse::<ReturnType>()?;
    let body = Box::new(parse_expr(stream, true)?);

    Ok(ExprClosure {
        span: Span::default(),
        attrs: no_attrs(),
        lifetimes: None,
        constness,
        movability: Movability::Movable,
        asyncness,
        capture,
        inputs,
        output,
        body,
    })
}

// --- lookahead helpers ---

/// Parse a bare lifetime label (no trailing `:`) for `break`/`continue`.
fn parse_opt_break_label(stream: &mut ParseStream) -> Option<Label> {
    if !matches!(stream.curr(), Some(TokenTree::Token(Token::Punct(Punctuation::Quote(_))))) {
        return None;
    }
    let name = stream.parse_opt::<crate::ast::Lifetime>()?;
    Some(Label {
        span: Span::default(),
        name,
    })
}

fn is_label_start(stream: &mut ParseStream) -> bool {
    // A lifetime (`'a`) directly followed by `:` is a loop/block label.
    matches!(stream.curr(), Some(TokenTree::Token(Token::Punct(Punctuation::Quote(_)))))
        && matches!(stream.nth(2), Some(TokenTree::Token(Token::Punct(Punctuation::Colon(_)))))
}

fn peek2_is_brace(stream: &ParseStream) -> bool {
    matches!(stream.nth(1), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace)
}

fn async_is_block(stream: &ParseStream) -> bool {
    // `async {` or `async move {` is a block; otherwise it's a closure.
    if peek2_is_brace(stream) {
        return true;
    }
    matches!(stream.nth(1), Some(tt) if is_named(tt, "move"))
        && matches!(stream.nth(2), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace)
}

/// Parse an optional range end — `None` if the next token can't begin an expr.
fn maybe_range_end(stream: &mut ParseStream, allow_struct: bool) -> Result<Option<Box<Expr>>, ParseError> {
    if stream.is_empty() || at_expr_terminator(stream) {
        return Ok(None);
    }
    let mut fork = stream.fork();
    match parse_unary(&mut fork, allow_struct) {
        Ok(e) => {
            let e = parse_binary(&mut fork, e, next(Precedence::Range), allow_struct)?;
            stream.seek(&fork);
            Ok(Some(Box::new(e)))
        }
        Err(_) => Ok(None),
    }
}

fn at_expr_terminator(stream: &mut ParseStream) -> bool {
    stream.is_empty() || stream.peek::<Semi>().is_some() || stream.peek::<Comma>().is_some()
}

fn is_named(tt: &TokenTree, name: &str) -> bool {
    match tt {
        TokenTree::Token(Token::Ident(id)) => id.name() == name,
        TokenTree::Token(Token::Keyword(kw)) => kw.as_str() == name,
        _ => false,
    }
}

fn is_bool_ident(stream: &mut ParseStream) -> bool {
    matches!(stream.curr(), Some(tt) if is_named(tt, "true") || is_named(tt, "false"))
}

fn is_literal(tt: &TokenTree) -> bool {
    matches!(tt, TokenTree::Token(Token::Literal(_)))
}

fn is_group(tt: &TokenTree, delim: Delim) -> bool {
    matches!(tt, TokenTree::Group(g) if g.delim() == delim)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::ast::{Stmt, StmtBlock};
    use crate::token::ToTokenStream;

    fn parse<T: Parse>(src: &str) -> T {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>().unwrap()
    }

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn literals_and_paths() {
        assert!(matches!(parse::<Expr>("42"), Expr::Lit(_)));
        assert!(matches!(parse::<Expr>("foo"), Expr::Path(_)));
        assert!(matches!(parse::<Expr>("a::b::c"), Expr::Path(_)));
        assert!(matches!(parse::<Expr>("true"), Expr::Lit(_)));
    }

    #[test]
    fn binary_precedence() {
        // `a + b * c` parses as `a + (b * c)`
        let e = parse::<Expr>("a + b * c");
        match e {
            Expr::Binary(ExprBinary {
                op: BinOp::Add, right, ..
            }) => {
                assert!(matches!(*right, Expr::Binary(ExprBinary { op: BinOp::Mul, .. })));
            }
            _ => panic!("expected top-level Add"),
        }
    }

    #[test]
    fn binary_left_assoc() {
        // `a - b - c` parses as `(a - b) - c`
        let e = parse::<Expr>("a - b - c");
        match e {
            Expr::Binary(ExprBinary {
                op: BinOp::Sub, left, ..
            }) => {
                assert!(matches!(*left, Expr::Binary(ExprBinary { op: BinOp::Sub, .. })));
            }
            _ => panic!("expected left-assoc Sub"),
        }
    }

    #[test]
    fn postfix() {
        assert!(matches!(parse::<Expr>("f(x)"), Expr::Call(_)));
        assert!(matches!(parse::<Expr>("a.b"), Expr::Field(_)));
        assert!(matches!(parse::<Expr>("a.b()"), Expr::MethodCall(_)));
        assert!(matches!(parse::<Expr>("a[0]"), Expr::Index(_)));
        assert!(matches!(parse::<Expr>("x?"), Expr::Try(_)));
        assert!(matches!(parse::<Expr>("x.await"), Expr::Await(_)));
        assert!(matches!(parse::<Expr>("a.0"), Expr::Field(_)));
    }

    #[test]
    fn method_turbofish() {
        let e = parse::<Expr>("x.collect::<Vec<_>>()");
        match e {
            Expr::MethodCall(m) => assert!(m.turbofish.is_some()),
            _ => panic!("expected method call with turbofish"),
        }
    }

    #[test]
    fn path_turbofish() {
        // `Foo::<T>` in expression position.
        assert!(matches!(parse::<Expr>("Foo::<T>"), Expr::Path(_)));
    }

    #[test]
    fn ranges() {
        assert!(matches!(parse::<Expr>("0..10"), Expr::Range(_)));
        assert!(matches!(parse::<Expr>("0..=10"), Expr::Range(_)));
        assert!(matches!(parse::<Expr>("a.."), Expr::Range(_)));
        assert!(matches!(parse::<Expr>("..b"), Expr::Range(_)));
        assert!(matches!(parse::<Expr>(".."), Expr::Range(_)));
    }

    #[test]
    fn if_while_let() {
        assert!(matches!(parse::<Expr>("if let Some(x) = o { x } else { 0 }"), Expr::If(_)));
        assert!(matches!(parse::<Expr>("while let Some(x) = it.next() { }"), Expr::While(_)));
    }

    #[test]
    fn block_exprs() {
        assert!(matches!(parse::<Expr>("async { 1 }"), Expr::Async(_)));
        assert!(matches!(parse::<Expr>("async move { x }"), Expr::Async(_)));
        assert!(matches!(parse::<Expr>("const { 1 }"), Expr::Const(_)));
        assert!(matches!(parse::<Expr>("try { 1 }"), Expr::TryBlock(_)));
    }

    #[test]
    fn closures_with_modifiers() {
        assert!(matches!(parse::<Expr>("async || 1"), Expr::Closure(_)));
        assert!(matches!(parse::<Expr>("async move |x| x"), Expr::Closure(_)));
        assert!(matches!(parse::<Expr>("const || 1"), Expr::Closure(_)));
    }

    #[test]
    fn labeled() {
        assert!(matches!(parse::<Expr>("'a: loop { break 'a 1 }"), Expr::Loop(_)));
        assert!(matches!(parse::<Expr>("'a: { 1 }"), Expr::Block(_)));
    }

    #[test]
    fn qualified_path_expr() {
        assert!(matches!(parse::<Expr>("<T as Trait>::CONST"), Expr::Path(_)));
        assert!(matches!(parse::<Expr>("::std::mem::swap"), Expr::Path(_)));
    }

    #[test]
    fn unary_and_ref() {
        assert!(matches!(parse::<Expr>("-x"), Expr::Unary(_)));
        assert!(matches!(parse::<Expr>("!x"), Expr::Unary(_)));
        assert!(matches!(parse::<Expr>("*x"), Expr::Unary(_)));
        assert!(matches!(parse::<Expr>("&x"), Expr::Reference(_)));
        assert!(matches!(parse::<Expr>("&mut x"), Expr::Reference(_)));
    }

    #[test]
    fn collections() {
        assert!(matches!(parse::<Expr>("(a, b)"), Expr::Tuple(_)));
        assert!(matches!(parse::<Expr>("(a)"), Expr::Paren(_)));
        assert!(matches!(parse::<Expr>("[a, b, c]"), Expr::Array(_)));
        assert!(matches!(parse::<Expr>("[0; 4]"), Expr::Repeat(_)));
    }

    #[test]
    fn cast_and_assign() {
        assert!(matches!(parse::<Expr>("x as u32"), Expr::Cast(_)));
        assert!(matches!(parse::<Expr>("x = y"), Expr::Assign(_)));
        assert!(matches!(parse::<Expr>("x += y"), Expr::AssignOp(_)));
    }

    #[test]
    fn control_flow() {
        assert!(matches!(parse::<Expr>("if a { b } else { c }"), Expr::If(_)));
        assert!(matches!(parse::<Expr>("while a { }"), Expr::While(_)));
        assert!(matches!(parse::<Expr>("for x in xs { }"), Expr::ForLoop(_)));
        assert!(matches!(parse::<Expr>("loop { }"), Expr::Loop(_)));
        assert!(matches!(parse::<Expr>("match x { _ => 1 }"), Expr::Match(_)));
        assert!(matches!(parse::<Expr>("{ a }"), Expr::Block(_)));
        assert!(matches!(parse::<Expr>("unsafe { }"), Expr::Unsafe(_)));
        assert!(matches!(parse::<Expr>("return x"), Expr::Return(_)));
    }

    #[test]
    fn struct_literal() {
        let e = parse::<Expr>("Foo { a: 1, b }");
        assert!(matches!(e, Expr::Struct(_)));
        // struct literal is disallowed in `if` condition position
        assert!(matches!(parse::<Expr>("if x { }"), Expr::If(_)));
    }

    #[test]
    fn closures() {
        assert!(matches!(parse::<Expr>("|x| x"), Expr::Closure(_)));
        assert!(matches!(parse::<Expr>("|x: u32| -> u32 { x }"), Expr::Closure(_)));
        assert!(matches!(parse::<Expr>("move || 1"), Expr::Closure(_)));
        assert!(matches!(parse::<Expr>("|| {}"), Expr::Closure(_)));
    }

    #[test]
    fn macro_call() {
        assert!(matches!(parse::<Expr>("vec![1, 2, 3]"), Expr::Macro(_)));
        assert!(matches!(parse::<Expr>("println!(\"hi\")"), Expr::Macro(_)));
    }

    #[test]
    fn patterns() {
        assert!(matches!(parse::<Pattern>("x"), Pattern::Ident(_)));
        assert!(matches!(parse::<Pattern>("_"), Pattern::Wild));
        assert!(matches!(parse::<Pattern>("mut x"), Pattern::Ident(_)));
        assert!(matches!(parse::<Pattern>("&x"), Pattern::Reference(_)));
        assert!(matches!(parse::<Pattern>("(a, b)"), Pattern::Tuple(_)));
        assert!(matches!(parse::<Pattern>("Some(x)"), Pattern::TupleStruct(_)));
        assert!(matches!(parse::<Pattern>("Point { x, y }"), Pattern::Struct(_)));
        assert!(matches!(parse::<Pattern>("1"), Pattern::Lit(_)));
    }

    #[test]
    fn or_and_exotic_patterns() {
        assert!(matches!(parse::<Pattern>("A | B | C"), Pattern::Or(_)));
        assert!(matches!(parse::<Pattern>("| A | B"), Pattern::Or(_)));
        assert!(matches!(parse::<Pattern>("box x"), Pattern::Box(_)));
        assert!(matches!(parse::<Pattern>("const { 1 }"), Pattern::Const(_)));
        // single alternative stays a non-Or pattern
        assert!(matches!(parse::<Pattern>("x"), Pattern::Ident(_)));
    }

    #[test]
    fn statements_and_blocks() {
        let b = parse::<StmtBlock>("{ let x = 1; x + 1 }");
        assert_eq!(b.stmts.len(), 2);
        assert!(matches!(b.stmts[0], Stmt::Local(_)));
        assert!(matches!(b.stmts[1], Stmt::Expr(..)));

        let b2 = parse::<StmtBlock>("{ foo(); bar(); }");
        assert_eq!(b2.stmts.len(), 2);
        assert!(matches!(b2.stmts[0], Stmt::Expr(_, Some(_))));
    }

    #[test]
    fn roundtrips() {
        for src in ["a + b * c", "f (x , y)", "a . b . c", "x as u32", "- x", "& mut x"] {
            let e = parse::<Expr>(src);
            // re-render and re-parse must produce an equal token string
            let r = render(&e);
            let e2 = parse::<Expr>(&r);
            assert_eq!(render(&e2), r, "unstable roundtrip for {src}");
        }
    }
}
