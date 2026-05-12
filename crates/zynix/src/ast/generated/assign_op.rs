#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum AssignOp {
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    BitXorAssign,
    BitAndAssign,
    BitOrAssign,
    ShlAssign,
    ShrAssign,
}
impl crate::ast::Visit for AssignOp {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            AssignOp::AddAssign => {}
            AssignOp::SubAssign => {}
            AssignOp::MulAssign => {}
            AssignOp::DivAssign => {}
            AssignOp::RemAssign => {}
            AssignOp::BitXorAssign => {}
            AssignOp::BitAndAssign => {}
            AssignOp::BitOrAssign => {}
            AssignOp::ShlAssign => {}
            AssignOp::ShrAssign => {}
        }
    }
}
impl crate::ast::Fold for AssignOp {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            AssignOp::AddAssign => AssignOp::AddAssign,
            AssignOp::SubAssign => AssignOp::SubAssign,
            AssignOp::MulAssign => AssignOp::MulAssign,
            AssignOp::DivAssign => AssignOp::DivAssign,
            AssignOp::RemAssign => AssignOp::RemAssign,
            AssignOp::BitXorAssign => AssignOp::BitXorAssign,
            AssignOp::BitAndAssign => AssignOp::BitAndAssign,
            AssignOp::BitOrAssign => AssignOp::BitOrAssign,
            AssignOp::ShlAssign => AssignOp::ShlAssign,
            AssignOp::ShrAssign => AssignOp::ShrAssign,
        }
    }
}

impl crate::Parse for AssignOp {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        use crate::token::*;
        let span = stream.span();
        // Multi-char before single-char.
        if stream.peek::<ShlEq>().is_some() {
            stream.parse::<ShlEq>()?;
            return Ok(AssignOp::ShlAssign);
        }
        if stream.peek::<ShrEq>().is_some() {
            stream.parse::<ShrEq>()?;
            return Ok(AssignOp::ShrAssign);
        }
        if stream.peek::<PlusEq>().is_some() {
            stream.parse::<PlusEq>()?;
            return Ok(AssignOp::AddAssign);
        }
        if stream.peek::<MinusEq>().is_some() {
            stream.parse::<MinusEq>()?;
            return Ok(AssignOp::SubAssign);
        }
        if stream.peek::<StarEq>().is_some() {
            stream.parse::<StarEq>()?;
            return Ok(AssignOp::MulAssign);
        }
        if stream.peek::<SlashEq>().is_some() {
            stream.parse::<SlashEq>()?;
            return Ok(AssignOp::DivAssign);
        }
        if stream.peek::<PercentEq>().is_some() {
            stream.parse::<PercentEq>()?;
            return Ok(AssignOp::RemAssign);
        }
        if stream.peek::<CaretEq>().is_some() {
            stream.parse::<CaretEq>()?;
            return Ok(AssignOp::BitXorAssign);
        }
        if stream.peek::<AndEq>().is_some() {
            stream.parse::<AndEq>()?;
            return Ok(AssignOp::BitAndAssign);
        }
        if stream.peek::<OrEq>().is_some() {
            stream.parse::<OrEq>()?;
            return Ok(AssignOp::BitOrAssign);
        }
        Err(crate::parse::ParseError::new(
            span,
            "expected assignment operator",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_add_assign() {
        let ts = crate::TokenStream::from_str("+=").unwrap();
        let mut ps = ts.parse();
        assert!(matches!(
            ps.parse::<AssignOp>().unwrap(),
            AssignOp::AddAssign
        ));
    }

    #[test]
    fn parse_shl_assign() {
        let ts = crate::TokenStream::from_str("<<=").unwrap();
        let mut ps = ts.parse();
        assert!(matches!(
            ps.parse::<AssignOp>().unwrap(),
            AssignOp::ShlAssign
        ));
    }
}
