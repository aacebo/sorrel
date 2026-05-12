use crate::ast::BinOp;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Precedence {
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
    pub(crate) fn of(op: &BinOp) -> Self {
        match op {
            BinOp::Add | BinOp::Sub => Precedence::Add,
            BinOp::Mul | BinOp::Div | BinOp::Rem => Precedence::Mul,
            BinOp::And => Precedence::And,
            BinOp::Or => Precedence::Or,
            BinOp::BitXor => Precedence::BitXor,
            BinOp::BitAnd => Precedence::BitAnd,
            BinOp::BitOr => Precedence::BitOr,
            BinOp::Shl | BinOp::Shr => Precedence::Shift,
            BinOp::Eq | BinOp::Lt | BinOp::Le | BinOp::Ne | BinOp::Ge | BinOp::Gt => {
                Precedence::Compare
            }
        }
    }
}
