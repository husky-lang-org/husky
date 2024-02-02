pub mod float;
pub mod int;

use self::float::*;
use self::int::*;
use crate::*;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub enum Literal {
    /// unit literal
    Unit(()),
    // todo: /// char literal
    // Char(char),
    /// boolean literal
    Bool(bool),
    /// 8-bit integer literal
    I8(i8),
    /// 16-bit integer literal
    I16(i16),
    /// 32-bit integer literal
    I32(i32),
    /// 64-bit integer literal
    I64(TermI64Literal),
    /// 128-bit integer literal
    I128(TermI128Literal),
    /// for cross compilation
    ISize(TermISizeLiteral),
    /// 8-bit unsigned integer literal
    U8(u8),
    /// 16-bit unsigned integer liteUal
    U16(u16),
    /// 32-bit unsigned integer liteUal
    U32(u32),
    /// 64-bit unsigned integer liteUal
    U64(TermU64Literal),
    /// 128-bit unsigned integer liteUal
    U128(TermU128Literal),
    /// for cross compilation
    USize(TermUSizeLiteral),
    /// 8-bit raw bit literal
    R8(u8),
    /// 16-bit raw bit literal
    R16(u16),
    /// 32-bit raw bit literal
    R32(u32),
    /// 64-bit raw bit literal
    R64(TermR64Literal),
    /// 128-bit raw bit literal
    R128(TermR128Literal),
    /// for cross compilation
    RSize(TermRSizeLiteral),
    /// natural number literal
    Nat(TermNatLiteral),
    /// 32-bit float literal
    F32(TermF32Literal),
    /// 64-bit float literal
    F64(TermF64Literal),
    /// string literal
    String(StringLiteralTokenData),
    /// static lifetime
    StaticLifetime,
}

#[salsa::tracked(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct StringLiteralTokenData {
    #[return_ref]
    pub data: String,
}

#[test]
fn term_literal_size_works() {
    assert_eq!(std::mem::size_of::<Literal>(), std::mem::size_of::<u64>())
}

impl From<()> for Literal {
    fn from((): ()) -> Self {
        Literal::Unit(())
    }
}

impl Literal {
    pub fn ty(self) -> PreludeTypePath {
        match self {
            Literal::Unit(()) => PreludeBasicTypePath::Unit.into(),
            Literal::Bool(_) => PreludeBasicTypePath::Bool.into(),
            Literal::I8(_) => PreludeIntTypePath::I8.into(),
            Literal::I16(_) => PreludeIntTypePath::I16.into(),
            Literal::I32(_) => PreludeIntTypePath::I32.into(),
            Literal::I64(_) => PreludeIntTypePath::I64.into(),
            Literal::I128(_) => PreludeIntTypePath::I128.into(),
            Literal::ISize(_) => PreludeIntTypePath::ISize.into(),
            Literal::U8(_) => PreludeIntTypePath::U8.into(),
            Literal::U16(_) => PreludeIntTypePath::U16.into(),
            Literal::U32(_) => PreludeIntTypePath::U32.into(),
            Literal::U64(_) => PreludeIntTypePath::U64.into(),
            Literal::U128(_) => PreludeIntTypePath::U128.into(),
            Literal::USize(_) => PreludeIntTypePath::USize.into(),
            Literal::R8(_) => PreludeIntTypePath::R8.into(),
            Literal::R16(_) => PreludeIntTypePath::R16.into(),
            Literal::R32(_) => PreludeIntTypePath::R32.into(),
            Literal::R64(_) => PreludeIntTypePath::R64.into(),
            Literal::R128(_) => PreludeIntTypePath::R128.into(),
            Literal::RSize(_) => PreludeIntTypePath::RSize.into(),
            Literal::Nat(_) => todo!(),
            // PreludeIntegerTypePath::Nat.into(),
            Literal::F32(_) => PreludeFloatTypePath::F32.into(),
            Literal::F64(_) => PreludeFloatTypePath::F64.into(),
            Literal::String(_) => PreludeTypePath::StringLiteral,
            Literal::StaticLifetime => PreludeTypePath::Lifetime,
        }
    }

    pub fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        use std::fmt::Display;
        match self {
            Literal::Unit(()) => f.write_str("unit"),
            Literal::I8(val) => f.write_fmt(format_args!("{}", val)),
            Literal::I16(val) => f.write_fmt(format_args!("{}", val)),
            Literal::I32(val) => f.write_fmt(format_args!("{}", val)),
            Literal::I64(val) => f.write_fmt(format_args!("{}", val.value(db))),
            Literal::I128(val) => f.write_fmt(format_args!("{}", val.value(db))),
            Literal::ISize(val) => f.write_fmt(format_args!("{}", val.value(db))),
            Literal::Nat(_) => todo!(),
            Literal::F32(val) => f.write_fmt(format_args!("{}", val.text(db))),
            Literal::F64(val) => f.write_fmt(format_args!("{}", val.text(db))),
            Literal::Bool(val) => f.write_fmt(format_args!("{}", val)),
            Literal::String(val) => f.write_fmt(format_args!("{:?}", val.data(db))),
            Literal::StaticLifetime => f.write_str("'static"),
            Literal::U8(val) => f.write_fmt(format_args!("{}", val)),
            Literal::U16(val) => f.write_fmt(format_args!("{}", val)),
            Literal::U32(val) => f.write_fmt(format_args!("{}", val)),
            Literal::U64(val) => f.write_fmt(format_args!("{}", val.value(db))),
            Literal::U128(val) => f.write_fmt(format_args!("{}", val.value(db))),
            Literal::USize(lit) => lit.value(db).fmt(f),
            Literal::R8(val) => f.write_fmt(format_args!("{}r8", val)),
            Literal::R16(val) => f.write_fmt(format_args!("{}r16", val)),
            Literal::R32(val) => f.write_fmt(format_args!("{}r32", val)),
            Literal::R64(val) => f.write_fmt(format_args!("{}", val.value(db))),
            Literal::R128(val) => f.write_fmt(format_args!("{}", val.value(db))),
            Literal::RSize(val) => f.write_fmt(format_args!("{}", val.value(db))),
        }
    }
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermNatLiteral {
    pub bits: Vec<usize>,
}

impl salsa::DisplayWithDb for Literal {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.display_fmt_with_db_and_ctx(f, db)
    }
}
