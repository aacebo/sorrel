#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum UnOp {
    Deref,
    Not,
    Neg,
}
impl crate::ast::Visit for UnOp {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            UnOp::Deref => {}
            UnOp::Not => {}
            UnOp::Neg => {}
        }
    }
}
impl crate::ast::Fold for UnOp {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            UnOp::Deref => UnOp::Deref,
            UnOp::Not => UnOp::Not,
            UnOp::Neg => UnOp::Neg,
        }
    }
}

impl crate::Parse for UnOp {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        use crate::token::*;
        let span = stream.span();
        if stream.peek::<Star>().is_some() {
            stream.parse::<Star>()?;
            return Ok(UnOp::Deref);
        }
        if stream.peek::<Not>().is_some() {
            stream.parse::<Not>()?;
            return Ok(UnOp::Not);
        }
        if stream.peek::<Minus>().is_some() {
            stream.parse::<Minus>()?;
            return Ok(UnOp::Neg);
        }
        Err(crate::parse::ParseError::new(
            span,
            "expected unary operator",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_not() {
        let ts = crate::TokenStream::from_str("!").unwrap();
        let mut ps = ts.parse();
        assert!(matches!(ps.parse::<UnOp>().unwrap(), UnOp::Not));
    }

    #[test]
    fn parse_deref() {
        let ts = crate::TokenStream::from_str("*").unwrap();
        let mut ps = ts.parse();
        assert!(matches!(ps.parse::<UnOp>().unwrap(), UnOp::Deref));
    }
}
