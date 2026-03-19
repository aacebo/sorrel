#![cfg_attr(
    nightly,
    feature(proc_macro_diagnostic, proc_macro_span, proc_macro_totokens)
)]

#[cfg(nightly)]
extern crate proc_macro;

mod parse;
mod span;
mod token;

#[cfg(feature = "report")]
pub mod report;

#[cfg(feature = "ast")]
pub mod ast;

pub use parse::*;
pub use span::*;
pub use token::*;
