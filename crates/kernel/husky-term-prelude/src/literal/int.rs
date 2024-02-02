use super::*;

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermI64Literal {
    pub value: i64,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermI128Literal {
    pub value: i128,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermI256Literal {
    pub value: [i128; 2],
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermISizeLiteral {
    pub value: i64,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermU64Literal {
    pub value: u64,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermU128Literal {
    pub value: u128,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermU256Literal {
    pub value: [u128; 2],
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermUSizeLiteral {
    pub value: u64,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermR64Literal {
    pub value: u64,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermR128Literal {
    pub value: u128,
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermR256Literal {
    pub value: [u128; 2],
}

#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermRSizeLiteral {
    pub value: u64,
}

impl Literal {
    pub fn from_unspecified_int(
        int_ty_path: PreludeIntTypePath,
        val: i128,
        db: &::salsa::Db,
    ) -> Literal {
        match int_ty_path {
            // signed integer types
            PreludeIntTypePath::I8 => Literal::I8(val.try_into().expect("todo")),
            PreludeIntTypePath::I16 => Literal::I16(val.try_into().expect("todo")),
            PreludeIntTypePath::I32 => Literal::I32(val.try_into().expect("todo")),
            PreludeIntTypePath::I64 => {
                Literal::I64(TermI64Literal::new(db, val.try_into().expect("todo")))
            }
            PreludeIntTypePath::I128 => {
                Literal::I128(TermI128Literal::new(db, val.try_into().expect("todo")))
            }
            PreludeIntTypePath::ISize => {
                Literal::ISize(TermISizeLiteral::new(db, val.try_into().expect("ok")))
            }
            // Unsigned integer types
            PreludeIntTypePath::U8 => {
                Literal::U8(val.try_into().expect("value out of range for u8"))
            }
            PreludeIntTypePath::U16 => {
                Literal::U16(val.try_into().expect("value out of range for u16"))
            }
            PreludeIntTypePath::U32 => {
                Literal::U32(val.try_into().expect("value out of range for u32"))
            }
            PreludeIntTypePath::U64 => {
                Literal::U64(TermU64Literal::new(db, val.try_into().expect("todo")))
            }
            PreludeIntTypePath::U128 => {
                Literal::U128(TermU128Literal::new(db, val.try_into().expect("todo")))
            }

            PreludeIntTypePath::USize => {
                Literal::USize(TermUSizeLiteral::new(db, val.try_into().expect("ok")))
            }
            // raw bits
            PreludeIntTypePath::R8 => {
                Literal::R8(val.try_into().expect("value out of range for r8"))
            }
            PreludeIntTypePath::R16 => {
                Literal::R16(val.try_into().expect("value out of range for r16"))
            }
            PreludeIntTypePath::R32 => {
                Literal::R32(val.try_into().expect("value out of range for r32"))
            }
            PreludeIntTypePath::R64 => {
                Literal::R64(TermR64Literal::new(db, val.try_into().expect("todo")))
            }
            PreludeIntTypePath::R128 => {
                Literal::R128(TermR128Literal::new(db, val.try_into().expect("todo")))
            }
            PreludeIntTypePath::RSize => {
                Literal::RSize(TermRSizeLiteral::new(db, val.try_into().expect("ok")))
            }
        }
    }
}
