use crate::ast::precedence::Precedence;
use crate::ast::{
    AngleArgs, AssignOp, Asyncness, Attribute, BinOp, Block, BoundLifetimes, ClosureParam, Constness, FieldValue, Ident, Label,
    Lit, MacroCall, Member, Movability, Mutability, Path, Pattern, Punctuated, QSelf, RangeLimits, ReturnType, Type, UnOp,
};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{
    Await as KwAwait, Break, Const, Continue, For, If, In, Let, Loop, Match, Move, Return, Unsafe, While, Yield,
};
use crate::token::punct::{And, Comma, Dot, DotDot, Eq, Not, Or, OrOr, Question, Semi, Star};
use crate::token::{Delim, Group, LexError, Punctuation, ToTokens};
use crate::{Parse, Span, Token, TokenStream, TokenTree};

// ===========================================================================
// Variant structs (constructed by the hand-written parser; ToTokens by hand).
// ===========================================================================

macro_rules! expr_structs {
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

expr_structs! {
    ExprLit { lit: Lit }
    ExprPath { qself: Option<QSelf>, path: Path }
    ExprReference { mutability: Mutability, expr: Box<Expr> }
    ExprUnary { op: UnOp, expr: Box<Expr> }
    ExprParen { expr: Box<Expr> }
    ExprGroup { expr: Box<Expr> }
    ExprTry { expr: Box<Expr> }
    ExprAwait { base: Box<Expr> }
    ExprTuple { elems: Punctuated<Expr, Comma> }
    ExprArray { elems: Punctuated<Expr, Comma> }
    ExprRepeat { elem: Box<Expr>, len: Box<Expr> }
    ExprRange { start: Option<Box<Expr>>, limits: RangeLimits, end: Option<Box<Expr>> }
    ExprCast { expr: Box<Expr>, ty: Box<Type> }
    ExprType { expr: Box<Expr>, ty: Box<Type> }
    ExprField { base: Box<Expr>, member: Member }
    ExprIndex { base: Box<Expr>, index: Box<Expr> }
    ExprReturn { expr: Option<Box<Expr>> }
    ExprContinue { label: Option<Label> }
    ExprBreak { label: Option<Label>, expr: Option<Box<Expr>> }
    ExprYield { expr: Option<Box<Expr>> }
    ExprBinary { left: Box<Expr>, op: BinOp, right: Box<Expr> }
    ExprAssign { left: Box<Expr>, right: Box<Expr> }
    ExprAssignOp { left: Box<Expr>, op: AssignOp, right: Box<Expr> }
    ExprCall { func: Box<Expr>, args: Punctuated<Expr, Comma> }
    ExprMethodCall {
        receiver: Box<Expr>, method: Ident, turbofish: Option<AngleArgs>,
        args: Punctuated<Expr, Comma>
    }
    ExprStruct {
        qself: Option<QSelf>, path: Path, fields: Punctuated<FieldValue, Comma>,
        rest: Option<Box<Expr>>
    }
    ExprBlock { label: Option<Label>, block: Block }
    ExprUnsafe { block: Block }
    ExprConst { block: Block }
    ExprTryBlock { block: Block }
    ExprAsync { capture: bool, block: Block }
    ExprLoop { label: Option<Label>, body: Block }
    ExprWhile { label: Option<Label>, cond: Box<Expr>, body: Block }
    ExprForLoop { label: Option<Label>, pat: Box<Pattern>, expr: Box<Expr>, body: Block }
    ExprIf { cond: Box<Expr>, then_branch: Block, else_branch: Option<Box<Expr>> }
    ExprMatch { expr: Box<Expr>, arms: Vec<crate::ast::MatchArm> }
    ExprLet { pat: Box<Pattern>, expr: Box<Expr> }
    ExprClosure {
        lifetimes: Option<BoundLifetimes>, constness: Constness, movability: Movability,
        asyncness: Asyncness, capture: bool, inputs: Punctuated<ClosureParam, Comma>,
        output: ReturnType, body: Box<Expr>
    }
}

// ===========================================================================
// ToTokens for each variant struct
// ===========================================================================

fn emit_attrs(attrs: &[Attribute], tokens: &mut TokenStream) {
    for a in attrs {
        a.to_tokens(tokens);
    }
}

impl ToTokens for ExprLit {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.lit.to_tokens(t);
    }
}
impl ToTokens for ExprPath {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.path.to_tokens(t);
    }
}
impl ToTokens for ExprReference {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        And::default().to_tokens(t);
        self.mutability.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
impl ToTokens for ExprUnary {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.op.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
impl ToTokens for ExprParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        let mut inner = TokenStream::new();
        self.expr.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
    }
}
impl ToTokens for ExprGroup {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.expr.to_tokens(t);
    }
}
impl ToTokens for ExprTry {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.expr.to_tokens(t);
        Question::default().to_tokens(t);
    }
}
impl ToTokens for ExprAwait {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.base.to_tokens(t);
        Dot::default().to_tokens(t);
        KwAwait::default().to_tokens(t);
    }
}
impl ToTokens for ExprTuple {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
    }
}
impl ToTokens for ExprArray {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Bracket, inner)));
    }
}
impl ToTokens for ExprRepeat {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        let mut inner = TokenStream::new();
        self.elem.to_tokens(&mut inner);
        Semi::default().to_tokens(&mut inner);
        self.len.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Bracket, inner)));
    }
}
impl ToTokens for ExprRange {
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
impl ToTokens for ExprCast {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.expr.to_tokens(t);
        crate::token::keyword::As::default().to_tokens(t);
        self.ty.to_tokens(t);
    }
}
impl ToTokens for ExprType {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.expr.to_tokens(t);
        crate::token::punct::Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
    }
}
impl ToTokens for ExprField {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.base.to_tokens(t);
        Dot::default().to_tokens(t);
        self.member.to_tokens(t);
    }
}
impl ToTokens for ExprIndex {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.base.to_tokens(t);
        let mut inner = TokenStream::new();
        self.index.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Bracket, inner)));
    }
}
impl ToTokens for ExprReturn {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Return::default().to_tokens(t);
        if let Some(e) = &self.expr {
            e.to_tokens(t);
        }
    }
}
impl ToTokens for ExprContinue {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Continue::default().to_tokens(t);
        if let Some(l) = &self.label {
            l.name.to_tokens(t);
        }
    }
}
impl ToTokens for ExprBreak {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Break::default().to_tokens(t);
        if let Some(l) = &self.label {
            l.name.to_tokens(t);
        }
        if let Some(e) = &self.expr {
            e.to_tokens(t);
        }
    }
}
impl ToTokens for ExprYield {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Yield::default().to_tokens(t);
        if let Some(e) = &self.expr {
            e.to_tokens(t);
        }
    }
}
impl ToTokens for ExprBinary {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.left.to_tokens(t);
        self.op.to_tokens(t);
        self.right.to_tokens(t);
    }
}
impl ToTokens for ExprAssign {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.left.to_tokens(t);
        Eq::default().to_tokens(t);
        self.right.to_tokens(t);
    }
}
impl ToTokens for ExprAssignOp {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.left.to_tokens(t);
        self.op.to_tokens(t);
        self.right.to_tokens(t);
    }
}
fn emit_paren_args(args: &Punctuated<Expr, Comma>, t: &mut TokenStream) {
    let mut inner = TokenStream::new();
    args.to_tokens(&mut inner);
    t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
}
impl ToTokens for ExprCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.func.to_tokens(t);
        emit_paren_args(&self.args, t);
    }
}
impl ToTokens for ExprMethodCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.receiver.to_tokens(t);
        Dot::default().to_tokens(t);
        self.method.to_tokens(t);
        if let Some(tf) = &self.turbofish {
            tf.to_tokens(t);
        }
        emit_paren_args(&self.args, t);
    }
}
impl ToTokens for ExprStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.path.to_tokens(t);
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);
        if let Some(rest) = &self.rest {
            DotDot::default().to_tokens(&mut inner);
            rest.to_tokens(&mut inner);
        }
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}
impl ToTokens for ExprBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        if let Some(l) = &self.label {
            l.to_tokens(t);
        }
        self.block.to_tokens(t);
    }
}
impl ToTokens for ExprUnsafe {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Unsafe::default().to_tokens(t);
        self.block.to_tokens(t);
    }
}
impl ToTokens for ExprConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Const::default().to_tokens(t);
        self.block.to_tokens(t);
    }
}
impl ToTokens for ExprTryBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        crate::token::keyword::Try::default().to_tokens(t);
        self.block.to_tokens(t);
    }
}
impl ToTokens for ExprAsync {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        crate::token::keyword::Async::default().to_tokens(t);
        if self.capture {
            Move::default().to_tokens(t);
        }
        self.block.to_tokens(t);
    }
}
fn emit_label(label: &Option<Label>, t: &mut TokenStream) {
    if let Some(l) = label {
        l.to_tokens(t);
    }
}
impl ToTokens for ExprLoop {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        emit_label(&self.label, t);
        Loop::default().to_tokens(t);
        self.body.to_tokens(t);
    }
}
impl ToTokens for ExprWhile {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        emit_label(&self.label, t);
        While::default().to_tokens(t);
        self.cond.to_tokens(t);
        self.body.to_tokens(t);
    }
}
impl ToTokens for ExprForLoop {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        emit_label(&self.label, t);
        For::default().to_tokens(t);
        self.pat.to_tokens(t);
        In::default().to_tokens(t);
        self.expr.to_tokens(t);
        self.body.to_tokens(t);
    }
}
impl ToTokens for ExprIf {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        If::default().to_tokens(t);
        self.cond.to_tokens(t);
        self.then_branch.to_tokens(t);
        if let Some(e) = &self.else_branch {
            crate::token::keyword::Else::default().to_tokens(t);
            e.to_tokens(t);
        }
    }
}
impl ToTokens for ExprMatch {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Match::default().to_tokens(t);
        self.expr.to_tokens(t);
        let mut inner = TokenStream::new();
        for arm in &self.arms {
            arm.to_tokens(&mut inner);
        }
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}
impl ToTokens for ExprLet {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Let::default().to_tokens(t);
        self.pat.to_tokens(t);
        Eq::default().to_tokens(t);
        self.expr.to_tokens(t);
    }
}
impl ToTokens for ExprClosure {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.constness.to_tokens(t);
        self.movability.to_tokens(t);
        self.asyncness.to_tokens(t);
        if self.capture {
            Move::default().to_tokens(t);
        }
        Or::default().to_tokens(t);
        self.inputs.to_tokens(t);
        Or::default().to_tokens(t);
        self.output.to_tokens(t);
        self.body.to_tokens(t);
    }
}

// ===========================================================================
// Expr enum
// ===========================================================================

#[doc = "A Rust expression. The primary recursive node covering all expression forms."]
#[derive(Debug, Clone)]
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

fn parse_expr(stream: &mut ParseStream, allow_struct: bool) -> Result<Expr, ParseError> {
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
    let then_branch = stream.parse::<Block>()?;
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
    let body = stream.parse::<Block>()?;
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
    let body = stream.parse::<Block>()?;
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
    let body = stream.parse::<Block>()?;
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
    use crate::ast::{Block, Stmt};
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
        let b = parse::<Block>("{ let x = 1; x + 1 }");
        assert_eq!(b.stmts.len(), 2);
        assert!(matches!(b.stmts[0], Stmt::Local(_)));
        assert!(matches!(b.stmts[1], Stmt::Expr(_)));

        let b2 = parse::<Block>("{ foo(); bar(); }");
        assert_eq!(b2.stmts.len(), 2);
        assert!(matches!(b2.stmts[0], Stmt::Semi(_)));
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
