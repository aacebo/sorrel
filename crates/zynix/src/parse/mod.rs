mod error;
mod stream;

pub use error::*;
pub use stream::*;

pub trait Parse: Sized {
    fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError>;
}
