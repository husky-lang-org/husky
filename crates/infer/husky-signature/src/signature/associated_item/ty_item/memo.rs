use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_memo_signature(db: &dyn SignatureDb, decl: TypeMemoDecl) -> TypeMemoSignature {
    let impl_block = decl.associated_item(db).impl_block(db);
    let parent_term_symbol_page = db.impl_block_decl(impl_block).ok().map(|decl| {
        impl_block_signature(db, decl)
            .term_sheet(db)
            .term_symbol_page()
    });
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), parent_term_symbol_page);
    let output_ty = match decl.output_ty(db) {
        Ok(output_ty) => engine.query_new(*output_ty),
        Err(_) => Abort(SignatureTermAbortion::ExprError),
    };
    TypeMemoSignature::new(db, output_ty, engine.finish())
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeMemoSignature {
    #[return_ref]
    pub output_ty: SignatureTermOutcome<Term>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
