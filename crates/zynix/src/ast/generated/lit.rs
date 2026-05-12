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
