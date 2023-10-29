use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeMemoizedFieldHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl TypeMemoizedFieldHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TypeItemPath,
        hir_decl: TypeMemoizedFieldHirDecl,
    ) -> TypeMemoizedFieldHirDefn {
        let Ok(TypeItemSynDefn::MemoizedField(syn_defn)) = path.syn_defn(db) else {
            unreachable!()
        };
        TypeMemoizedFieldHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_eager_body_with_expr_region(syn_defn.body_with_syn_expr_region(db), db),
        )
    }

    pub fn hir_eager_expr_region(self, db: &dyn HirDefnDb) -> Option<HirEagerExprRegion> {
        Some(self.eager_body_with_hir_eager_expr_region(db)?.1)
    }
}
