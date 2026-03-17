extern crate proc_macro;

mod level;

pub use level::*;

use crate::Span;

#[derive(Debug, Clone)]
pub struct Label {
    span: Span,
    message: String,
}
