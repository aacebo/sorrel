#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    BitXor,
    BitAnd,
    BitOr,
    Shl,
    Shr,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
}
impl crate::ast::Visit for BinOp {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            BinOp::Add => {}
            BinOp::Sub => {}
            BinOp::Mul => {}
            BinOp::Div => {}
            BinOp::Rem => {}
            BinOp::And => {}
            BinOp::Or => {}
            BinOp::BitXor => {}
            BinOp::BitAnd => {}
            BinOp::BitOr => {}
            BinOp::Shl => {}
            BinOp::Shr => {}
            BinOp::Eq => {}
            BinOp::Lt => {}
            BinOp::Le => {}
            BinOp::Ne => {}
            BinOp::Ge => {}
            BinOp::Gt => {}
        }
    }
}
impl crate::ast::Fold for BinOp {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            BinOp::Add => BinOp::Add,
            BinOp::Sub => BinOp::Sub,
            BinOp::Mul => BinOp::Mul,
            BinOp::Div => BinOp::Div,
            BinOp::Rem => BinOp::Rem,
            BinOp::And => BinOp::And,
            BinOp::Or => BinOp::Or,
            BinOp::BitXor => BinOp::BitXor,
            BinOp::BitAnd => BinOp::BitAnd,
            BinOp::BitOr => BinOp::BitOr,
            BinOp::Shl => BinOp::Shl,
            BinOp::Shr => BinOp::Shr,
            BinOp::Eq => BinOp::Eq,
            BinOp::Lt => BinOp::Lt,
            BinOp::Le => BinOp::Le,
            BinOp::Ne => BinOp::Ne,
            BinOp::Ge => BinOp::Ge,
            BinOp::Gt => BinOp::Gt,
        }
    }
}

impl crate::Parse for BinOp {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        use crate::token::*;
        let span = stream.span();
        // Multi-char operators must be checked before their single-char prefixes.
        if stream.peek::<AndAnd>().is_some() {
            stream.parse::<AndAnd>()?;
            return Ok(BinOp::And);
        }
        if stream.peek::<OrOr>().is_some() {
            stream.parse::<OrOr>()?;
            return Ok(BinOp::Or);
        }
        if stream.peek::<Shl>().is_some() {
            stream.parse::<Shl>()?;
            return Ok(BinOp::Shl);
        }
        if stream.peek::<Shr>().is_some() {
            stream.parse::<Shr>()?;
            return Ok(BinOp::Shr);
        }
        if stream.peek::<EqEq>().is_some() {
            stream.parse::<EqEq>()?;
            return Ok(BinOp::Eq);
        }
        if stream.peek::<Ne>().is_some() {
            stream.parse::<Ne>()?;
            return Ok(BinOp::Ne);
        }
        if stream.peek::<Le>().is_some() {
            stream.parse::<Le>()?;
            return Ok(BinOp::Le);
        }
        if stream.peek::<Ge>().is_some() {
            stream.parse::<Ge>()?;
            return Ok(BinOp::Ge);
        }
        if stream.peek::<Plus>().is_some() {
            stream.parse::<Plus>()?;
            return Ok(BinOp::Add);
        }
        if stream.peek::<Minus>().is_some() {
            stream.parse::<Minus>()?;
            return Ok(BinOp::Sub);
        }
        if stream.peek::<Star>().is_some() {
            stream.parse::<Star>()?;
            return Ok(BinOp::Mul);
        }
        if stream.peek::<Slash>().is_some() {
            stream.parse::<Slash>()?;
            return Ok(BinOp::Div);
        }
        if stream.peek::<Percent>().is_some() {
            stream.parse::<Percent>()?;
            return Ok(BinOp::Rem);
        }
        if stream.peek::<Caret>().is_some() {
            stream.parse::<Caret>()?;
            return Ok(BinOp::BitXor);
        }
        if stream.peek::<And>().is_some() {
            stream.parse::<And>()?;
            return Ok(BinOp::BitAnd);
        }
        if stream.peek::<Or>().is_some() {
            stream.parse::<Or>()?;
            return Ok(BinOp::BitOr);
        }
        if stream.peek::<Lt>().is_some() {
            stream.parse::<Lt>()?;
            return Ok(BinOp::Lt);
        }
        if stream.peek::<Gt>().is_some() {
            stream.parse::<Gt>()?;
            return Ok(BinOp::Gt);
        }
        Err(crate::parse::ParseError::new(
            span,
            "expected binary operator",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_add() {
        let ts = crate::TokenStream::from_str("+").unwrap();
        let mut ps = ts.parse();
        assert!(matches!(ps.parse::<BinOp>().unwrap(), BinOp::Add));
    }

    #[test]
    fn parse_and_and() {
        let ts = crate::TokenStream::from_str("&&").unwrap();
        let mut ps = ts.parse();
        assert!(matches!(ps.parse::<BinOp>().unwrap(), BinOp::And));
    }

    #[test]
    fn parse_eq_eq() {
        let ts = crate::TokenStream::from_str("==").unwrap();
        let mut ps = ts.parse();
        assert!(matches!(ps.parse::<BinOp>().unwrap(), BinOp::Eq));
    }
}
