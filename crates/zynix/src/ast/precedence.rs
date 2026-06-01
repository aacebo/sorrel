use crate::ast::BinOp;

#[doc = "Operator precedence level used when parsing and printing expressions without parentheses."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Precedence {
    Min = 0,
    Range,   // .. ..=
    Or,      // ||
    And,     // &&
    Compare, // == != < > <= >=
    BitOr,   // |
    BitXor,  // ^
    BitAnd,  // &
    Shift,   // << >>
    Add,     // + -
    Mul,     // * / %
    Cast,    // as
}

impl Precedence {
    pub fn of(op: &BinOp) -> Self {
        match op {
            BinOp::Add | BinOp::Sub => Precedence::Add,
            BinOp::Mul | BinOp::Div | BinOp::Rem => Precedence::Mul,
            BinOp::And => Precedence::And,
            BinOp::Or => Precedence::Or,
            BinOp::BitXor => Precedence::BitXor,
            BinOp::BitAnd => Precedence::BitAnd,
            BinOp::BitOr => Precedence::BitOr,
            BinOp::Shl | BinOp::Shr => Precedence::Shift,
            BinOp::Eq | BinOp::Lt | BinOp::Le | BinOp::Ne | BinOp::Ge | BinOp::Gt => Precedence::Compare,
        }
    }
}
