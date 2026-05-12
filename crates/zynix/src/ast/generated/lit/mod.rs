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
impl crate::ast::Visit for Lit {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Lit::Str { value } => {
                let _ = &value;
            }
            Lit::ByteStr { value } => {
                let _ = &value;
            }
            Lit::CStr { value } => {
                let _ = &value;
            }
            Lit::Byte { value } => {
                let _ = &value;
            }
            Lit::Char { value } => {
                let _ = &value;
            }
            Lit::Int { value } => {
                let _ = &value;
            }
            Lit::Float { value } => {
                let _ = &value;
            }
            Lit::Bool { value } => {
                let _ = &value;
            }
            Lit::Verbatim { tokens } => {
                let _ = &tokens;
            }
        }
    }
}
impl crate::ast::Fold for Lit {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Lit::Str { value } => Lit::Str { value },
            Lit::ByteStr { value } => Lit::ByteStr { value },
            Lit::CStr { value } => Lit::CStr { value },
            Lit::Byte { value } => Lit::Byte { value },
            Lit::Char { value } => Lit::Char { value },
            Lit::Int { value } => Lit::Int { value },
            Lit::Float { value } => Lit::Float { value },
            Lit::Bool { value } => Lit::Bool { value },
            Lit::Verbatim { tokens } => Lit::Verbatim { tokens },
        }
    }
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
