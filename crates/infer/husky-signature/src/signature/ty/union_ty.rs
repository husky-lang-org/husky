use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn union_ty_signature(db: &dyn SignatureDb, decl: UnionTypeDecl) -> UnionTypeSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    UnionTypeSignature::new(db,    ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine), engine.finish())
}

#[salsa::tracked(jar = SignatureJar)]
pub struct UnionTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl UnionTypeSignature {}
