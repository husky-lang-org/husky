use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct EnumUnitTypeVariantHirDecl {
    pub parent_ty_template: EnumHirDecl,
}
