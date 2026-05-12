#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TraitBound {
    pub span: crate::Span,
    pub polarity: BoundPolarity,
    pub lifetimes: Option<BoundLifetimes>,
    pub modifier: TraitBoundModifier,
    pub path: Path,
}
