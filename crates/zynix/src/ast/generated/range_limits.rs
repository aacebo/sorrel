#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum RangeLimits {
    HalfOpen,
    Closed,
}
impl crate::ast::Visit for RangeLimits {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            RangeLimits::HalfOpen => {}
            RangeLimits::Closed => {}
        }
    }
}
impl crate::ast::Fold for RangeLimits {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            RangeLimits::HalfOpen => RangeLimits::HalfOpen,
            RangeLimits::Closed => RangeLimits::Closed,
        }
    }
}
