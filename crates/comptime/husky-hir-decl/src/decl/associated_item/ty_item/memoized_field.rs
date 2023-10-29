use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMemoizedFieldHirDecl {
    pub path: TypeItemPath,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TypeMemoizedFieldHirDecl {
    pub(super) fn from_ethereal(
        path: TypeItemPath,
        ethereal_signature_template: TypeMemoizedFieldEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let TypeItemSynDecl::MemoizedField(syn_decl) = path.syn_decl(db).expect("ok") else {
            unreachable!()
        };
        let return_ty = HirType::from_ethereal(ethereal_signature_template.return_ty(db), db);
        Self::new(
            db,
            path,
            return_ty,
            hir_eager_expr_region(syn_decl.syn_expr_region(db), db),
        )
    }
}
