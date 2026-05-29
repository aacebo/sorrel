#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TraitRef {
    pub span: crate::Span,
    pub polarity: BoundPolarity,
    pub path: Path,
}
