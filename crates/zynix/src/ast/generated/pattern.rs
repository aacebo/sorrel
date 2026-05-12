use super::*;
#[derive(Debug, Clone)]
pub enum Pattern {
    Wild {},
    Rest {},
    Ident { value: PatIdent },
    Path { value: PatPath },
    Tuple { value: PatTuple },
    TupleStruct { value: PatTupleStruct },
    Struct { value: PatStruct },
    Slice { value: PatSlice },
    Reference { value: PatReference },
    Or { value: PatOr },
    Lit { value: PatLit },
    Range { value: PatRange },
    Macro { value: MacroCall },
    Type { value: PatType },
    Group { value: PatGroup },
    Paren { value: PatParen },
}
