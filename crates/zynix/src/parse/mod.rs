mod cursor;
mod error;
mod stream;

#[allow(unused)]
pub use cursor::*;
pub use error::*;
pub use stream::*;

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream<'_>) -> Result<Self, ParseError>;
}
