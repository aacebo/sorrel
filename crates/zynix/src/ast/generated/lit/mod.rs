#[allow(unused)]
use super::*;
#[doc = "A literal value in source code (string, integer, float, byte, char, or boolean)."]
#[derive(Debug, Clone)]
pub enum Lit {
    Str { value: LitStr },
    ByteStr { value: LitByteStr },
    CStr { value: LitCStr },
    Byte { value: LitByte },
    Char { value: LitChar },
    Int { value: LitInt },
    Float { value: LitFloat },
    Bool { value: LitBool },
    Verbatim { tokens: crate::TokenStream },
}
mod lit_bool;
pub use lit_bool::*;
mod lit_byte;
pub use lit_byte::*;
mod lit_byte_str;
pub use lit_byte_str::*;
mod lit_c_str;
pub use lit_c_str::*;
mod lit_char;
pub use lit_char::*;
mod lit_float;
pub use lit_float::*;
mod lit_int;
pub use lit_int::*;
mod lit_str;
pub use lit_str::*;
